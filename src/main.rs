use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Advinhe o número");
    let mut guesses: u32 = 0;
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Informe um número: ");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler o número");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        guesses += 1;
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Você ganhou!");
                break;
            }
            Ordering::Greater => println!("Está para baixo"),
            Ordering::Less => println!("Está para cima"),
        };
    }

    println!("Você usou {guesses} tentativas")
}
