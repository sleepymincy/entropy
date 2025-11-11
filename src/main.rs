use atty::Stream;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::io::{self, Read};
use std::process::exit;

fn estimate_charset(password: &str) -> usize {
    let mut lower: bool = false;
    let mut upper: bool = false;
    let mut digit: bool = false;
    let mut punct: bool = false;
    let mut space: bool = false;
    let mut other: HashSet<_> = HashSet::new();

    for c in password.chars() {
        if c.is_ascii_lowercase() {
            lower = true;
            continue;
        }
        if c.is_ascii_uppercase() {
            upper = true;
            continue;
        }
        if c.is_ascii_digit() {
            digit = true;
            continue;
        }
        if c.is_ascii_whitespace() {
            space = true;
            continue;
        }
        if c.is_ascii_punctuation() {
            punct = true;
            continue;
        }
        other.insert(c);
    }

    let mut size: usize = 0usize;
    if lower {
        size += 26
    }
    if upper {
        size += 26
    }
    if digit {
        size += 10
    }
    if punct {
        size += 32
    }
    if space {
        size += 1
    }
    size += other.len();

    if size == 0 {
        size = 26
    }
    size
}

fn shannon_entropy(password: &str) -> (f64, f64) {
    let password_length: usize = password.chars().count();
    if password_length == 0 {
        return (0.0, 0.0);
    }
    let mut character_frequency: HashMap<char, usize> = HashMap::new();
    for c in password.chars() {
        *character_frequency.entry(c).or_insert(0) += 1;
    }
    let password_length_float: f64 = password_length as f64;
    let mut entropy: f64 = 0.0;

    for &values in character_frequency.values() {
        let probability: f64 = (values as f64) / password_length_float;
        entropy += -probability * probability.log2();
    }

    (entropy, entropy * password_length_float)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        eprintln!("Usage: {} '<password>'", args[0]);
        exit(1);
    }

    let password: String = if args.len() > 1 {
        args[1].clone()
    } else if !atty::is(Stream::Stdin) {
        let mut s: String = String::new();
        io::stdin().read_to_string(&mut s).unwrap();
        if s.ends_with('\n') {
            s.pop();
            if s.ends_with('\r') {
                s.pop();
            }
        }
        s
    } else {
        eprintln!("Usage: {} '<password>'", args[0]);
        exit(1);
    };

    let password_length: usize = password.chars().count();
    let character_set: usize = estimate_charset(&password);
    let charset_bits: f64 = if password_length > 0 && character_set > 0 {
        (password_length as f64) * (character_set as f64).log2()
    } else {
        0.0
    };
    let (shannon_char, shannon_total) = shannon_entropy(&password);
    let strength: &str = if shannon_total < 30.0 {
        "Weak"
    } else if shannon_total < 40.0 {
        "Fair"
    } else if shannon_total < 50.0 {
        "Strong"
    } else if shannon_total < 80.0 {
        "Very strong"
    } else {
        "Extremely strong"
    };

    println!("Password length: {}", password_length);
    println!("Charset size: {}", character_set);
    println!("Charset bits: {:.6}", charset_bits);
    println!("Shannon entropy per character: {:.6}", shannon_char);
    println!("Shannon entropy total: {:.6}", shannon_total);
    println!("Password total strength: {}", strength)
}
