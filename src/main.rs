use std::io;

fn main() {
    println!("What is your name?");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");
    let parsed_name = name.trim();
    println!("Hello, {parsed_name}!");
}
