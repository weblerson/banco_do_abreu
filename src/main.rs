use std::io;

use clearscreen;

struct Account {
    num: String,
    password: String,
    balance: f64
}

fn main() {
    let mut account = Account{
        num: String::from("12345-6"),
        password: String::from("12345"),
        balance: 200.0
    };

    clear_terminal();

    loop {
        println!("\n********* BANCO DO ABREU *********");
        println!("---- Se ele não pagar, nem eu ----");

        let mut num = String::new();
        let mut password = String::new();

        println!("\nDigite o número da conta: ");
        io::stdin()
            .read_line(&mut num)
            .expect("Invalid entry");

        println!("\nDigite a senha: ");
        io::stdin()
            .read_line(&mut password)
            .expect("Invalid entry");

        if num.trim() != account.num || password.trim() != account.password {
            clear_terminal();
            println!("\nNúmero da conta ou senha incorretos. Tente novamente!");

            continue;
        }

        clear_terminal();

        break;
    }

    let option: char = loop {
        println!("\n****** MENU ******");

        println!("\nS - SAQUE");
        println!("D - DEPÓSITO");
        println!("P - PAGAMENTO");
        println!("Q - SAIR");

        println!("\nDigite sua opção: ");

        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Invalid entry");

        let option: char = match option.trim().parse() {
            Ok(chr) => chr,
            Err(_) => {
                println!("\nDigite um caractere válido!");

                continue
            },
        };

        match option {
            'S' | 'D' | 'P' | 'Q' => break option,
            _ => {
                println!("\nDigite uma opção válida!");

                continue;
            },
        };
    };

    loop {
        match option {
            'S' | 'P' => {
                println!("\nDigite o valor do saque/pagamento: ");
                println!("(Para cancelar, digite qualquer valor não numérico)");

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("invalid entry");

                let (_wanna_cancel, value) = wanna_cancel(input);

                if _wanna_cancel { break; }

                if withdraw(&mut account, value) {
                    println!("\nSaque realizado com sucesso!");
                    println!("Seu saldo atual: {}\n", account.balance);

                    return;
                }

                println!("\nImpossível realizar o saque. Saldo insuficiente!");
                println!("Seu saldo atual: {}\n", account.balance);
            },

            'D' => {
                println!("\nDigite o valor depósito");
                println!("(Para cancelar, digite qualquer valor não numérico)");

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("invalid entry");

                let (_wanna_cancel, value) = wanna_cancel(input);
                if _wanna_cancel { break; }

                deposit(&mut account, value);

                println!("\nValor depositado com sucesso!");
                println!("Seu saldo atual: {}\n", account.balance);

                return;
            },

            'Q' | _ => break,
        };
    };

    return;
}

fn clear_terminal() {
    clearscreen::clear().expect("failed to clear screen");

    return;
}

fn wanna_cancel(value: String) -> (bool, f64) {
    let value_as_float = value.trim().parse::<f64>();

    match value_as_float {
        Ok(num) => {
            return (false, num);
        },
        Err(_) => {
            return (true, 0.0);
        },
    };
}

fn withdraw(account: &mut Account, value: f64) -> bool {
    const EPSILON: f64 = 10e-2;

    let result =
        if (value - account.balance).abs() < EPSILON { 'E' }
        else if value < account.balance { 'L' }
        else { 'G' };

    let result = match result {
        'E' | 'L' => {
            account.balance -= value;

            true
        },
        _ => false,
    };

    result
}

fn deposit(account: &mut Account, value: f64) {
    account.balance += value;

    return;
}
