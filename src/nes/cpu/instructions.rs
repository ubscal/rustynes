use super::super::cpu_registers::CpuRegisters;
use super::super::bus::cpu_bus::CpuBus;
use super::super::types::{Data, Addr, Word};
use super::super::helper::*;

pub fn lda<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    let computed = bus.read(opeland);
    registers
        .set_A(computed)
        .update_negative_by(computed)
        .update_zero_by(computed);
}

pub fn lda_imm<T: CpuRegisters>(opeland: Word, registers: &mut T) {
    registers
        .set_A(opeland as Data)
        .update_negative_by(opeland as Data)
        .update_zero_by(opeland as Data);
}

pub fn ldx<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    let computed = bus.read(opeland);
    registers
        .set_X(computed)
        .update_negative_by(computed)
        .update_zero_by(computed);
}

pub fn ldx_imm<T: CpuRegisters>(opeland: Word, registers: &mut T) {
    registers
        .set_X(opeland as Data)
        .update_negative_by(opeland as Data)
        .update_zero_by(opeland as Data);
}

pub fn ldy<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    let computed = bus.read(opeland);
    registers
        .set_Y(computed)
        .update_negative_by(computed)
        .update_zero_by(computed);
}

pub fn ldy_imm<T: CpuRegisters>(opeland: Word, registers: &mut T) {
    registers
        .set_Y(opeland as Data)
        .update_negative_by(opeland as Data)
        .update_zero_by(opeland as Data);
}

pub fn sta<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    bus.write(opeland, registers.get_A());
}

pub fn stx<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    bus.write(opeland, registers.get_X());
}

pub fn sty<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    bus.write(opeland, registers.get_Y());
}

pub fn txa<T: CpuRegisters>(registers: &mut T) {
    let x = registers.get_X();
    registers
        .set_A(x)
        .update_negative_by(x)
        .update_zero_by(x);
}

pub fn tya<T: CpuRegisters>(registers: &mut T) {
    let y = registers.get_Y();
    registers
        .set_A(y)
        .update_negative_by(y)
        .update_zero_by(y);
}

pub fn txs<T: CpuRegisters>(registers: &mut T) {
    let x = registers.get_X();
    registers.set_SP(x);
}

pub fn tay<T: CpuRegisters>(registers: &mut T) {
    let acc = registers.get_A();
    registers
        .set_Y(acc)
        .update_negative_by(acc)
        .update_zero_by(acc);
}

pub fn tax<T: CpuRegisters>(registers: &mut T) {
    let acc = registers.get_A();
    registers
        .set_X(acc)
        .update_negative_by(acc)
        .update_zero_by(acc);
}

pub fn tsx<T: CpuRegisters>(registers: &mut T) {
    let sp = registers.get_SP();
    registers
        .set_X(sp)
        .update_negative_by(sp)
        .update_zero_by(sp);
}

pub fn php<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) {
    registers.set_break(true);
    push_status(registers, bus);
}

pub fn plp<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) {
    registers.set_reserved(true);
    let status = pop(registers, bus);
    registers.set_P(status);
}

pub fn pha<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) {
    let acc = registers.get_A();
    push(acc, registers, bus);
}

pub fn pla<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) {
    let v = pop(registers, bus);
    registers
        .set_A(v)
        .update_negative_by(v)
        .update_zero_by(v);
}

pub fn adc_imm<T: CpuRegisters>(opeland: Word, registers: &mut T) {
    let computed = (opeland as Data) + registers.get_A() + bool_to_u8(registers.get_carry());
    registers
        .update_overflow_by((opeland as Data), computed)
        .update_negative_by(computed)
        .update_zero_by(computed)
        .set_carry(computed > 0xFF as u8)
        .set_A(computed);
}

pub fn adc<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    let fetched = bus.read(opeland);
    let computed = fetched + registers.get_A() + bool_to_u8(registers.get_carry());
    registers
        .update_overflow_by((opeland as Data), computed)
        .update_negative_by(computed)
        .update_zero_by(computed)
        .set_carry(computed > 0xFF as u8)
        .set_A(computed);
}

pub fn sbc_imm<T: CpuRegisters>(opeland: Word, registers: &mut T) {
    let computed = registers.get_A() - (opeland as Data) - bool_to_u8(!registers.get_carry());
    registers
        .update_overflow_by(computed, (opeland as Data))
        .update_negative_by(computed)
        .update_zero_by(computed)
        .set_carry(computed >= 0 as u8)
        .set_A(computed);
}

pub fn sbc<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    let fetched = bus.read(opeland);
    let computed = registers.get_A() - fetched - bool_to_u8(!registers.get_carry());
    registers
        .update_overflow_by(computed, fetched)
        .update_negative_by(computed)
        .update_zero_by(computed)
        .set_carry(computed >= 0 as u8)
        .set_A(computed);
}

pub fn cpx_imm<T: CpuRegisters>(opeland: Word, registers: &mut T) {
    let computed = registers.get_X() - (opeland as u8);
    registers
        .update_negative_by(computed)
        .update_zero_by(computed)
        .set_carry(computed >= 0 as u8);
}

pub fn cpx<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    let fetched = bus.read(opeland);
    let computed = registers.get_X() - fetched;
    registers
        .update_negative_by(computed)
        .update_zero_by(computed)
        .set_carry(computed >= 0 as u8);
}

pub fn cpy_imm<T: CpuRegisters>(opeland: Word, registers: &mut T) {
    let computed = registers.get_Y() - (opeland as u8);
    registers
        .update_negative_by(computed)
        .update_zero_by(computed)
        .set_carry(computed >= 0 as u8);
}

pub fn cpy<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    let fetched = bus.read(opeland);
    let computed = registers.get_Y() - fetched;
    registers
        .update_negative_by(computed)
        .update_zero_by(computed)
        .set_carry(computed >= 0 as u8);
}

pub fn cmp_imm<T: CpuRegisters>(opeland: Word, registers: &mut T) {
    let computed = registers.get_A() - (opeland as u8);
    registers
        .update_negative_by(computed)
        .update_zero_by(computed)
        .set_carry(computed >= 0 as u8);
}

pub fn cmp<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    let fetched = bus.read(opeland);
    let computed = registers.get_A() - fetched;
    registers
        .update_negative_by(computed)
        .update_zero_by(computed)
        .set_carry(computed >= 0 as u8);
}

pub fn and_imm<T: CpuRegisters>(opeland: Word, registers: &mut T) {
    let computed = registers.get_A() & (opeland as u8);
    registers
        .update_negative_by(computed)
        .update_zero_by(computed)
        .set_A(computed);
}

pub fn and<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    let fetched = bus.read(opeland);
    let computed = registers.get_A() & fetched;
    registers
        .update_negative_by(computed)
        .update_zero_by(computed)
        .set_A(computed);
}

pub fn eor_imm<T: CpuRegisters>(opeland: Word, registers: &mut T) {
    let computed = registers.get_A() ^ (opeland as u8);
    registers
        .update_negative_by(computed)
        .update_zero_by(computed)
        .set_A(computed);
}

pub fn eor<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    let fetched = bus.read(opeland);
    let computed = registers.get_A() ^ fetched;
    registers
        .update_negative_by(computed)
        .update_zero_by(computed)
        .set_A(computed);
}

pub fn ora_imm<T: CpuRegisters>(opeland: Word, registers: &mut T) {
    let computed = registers.get_A() | (opeland as u8);
    registers
        .update_negative_by(computed)
        .update_zero_by(computed)
        .set_A(computed);
}

pub fn ora<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    let fetched = bus.read(opeland);
    let computed = registers.get_A() | fetched;
    registers
        .update_negative_by(computed)
        .update_zero_by(computed)
        .set_A(computed);
}

pub fn bit<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    let fetched = bus.read(opeland);
    let acc = registers.get_A();
    registers
        .update_negative_by(fetched)
        .update_zero_by(fetched & acc)
        .set_overflow((fetched & 0x40) == 0x40);
}

pub fn asl_acc<T: CpuRegisters>(registers: &mut T) {
    let acc = registers.get_A();
    let shifted = (acc << 1) as u8;
    registers
        .set_carry(acc & 0x80 == 0x80)
        .update_negative_by(shifted)
        .update_zero_by(shifted)
        .set_A(shifted);
}

pub fn asl<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    let fetched = bus.read(opeland);
    let shifted = (fetched << 1) as u8;
    registers
        .set_carry(fetched & 0x80 == 0x80)
        .update_negative_by(shifted)
        .update_zero_by(shifted);
    bus.write(opeland, shifted);
}

pub fn lsr_acc<T: CpuRegisters>(registers: &mut T) {
    let acc = registers.get_A();
    let shifted = (acc >> 1) as u8;
    registers
        .set_carry((acc & 0x01) == 0x01)
        .update_negative_by(shifted)
        .update_zero_by(shifted)
        .set_A(shifted);
}

pub fn lsr<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    let fetched = bus.read(opeland);
    let shifted = (fetched >> 1) as u8;
    registers
        .set_carry(fetched & 0x01 == 0x01)
        .update_negative_by(shifted)
        .update_zero_by(shifted);
    bus.write(opeland, shifted);
}

pub fn rol_acc<T: CpuRegisters>(registers: &mut T) {
    let acc = registers.get_A();
    let rotated = rotate_to_left(registers, acc);
    registers
        .set_carry(acc & 0x01 == 0x01)
        .update_negative_by(rotated)
        .update_zero_by(rotated)
        .set_A(rotated);
}

pub fn rol<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    let fetched = bus.read(opeland);
    let rotated = rotate_to_left(registers, fetched);
    registers
        .set_carry(fetched & 0x01 == 0x01)
        .update_negative_by(rotated)
        .update_zero_by(rotated);
    bus.write(opeland, rotated);
}

pub fn ror_acc<T: CpuRegisters>(registers: &mut T) {
    let acc = registers.get_A();
    let rotated = rotate_to_right(registers, acc);
    registers
        .set_carry(acc & 0x01 == 0x01)
        .update_negative_by(rotated)
        .update_zero_by(rotated)
        .set_A(rotated);
}

pub fn ror<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    let fetched = bus.read(opeland);
    let rotated = rotate_to_right(registers, fetched);
    registers
        .set_carry(fetched & 0x01 == 0x01)
        .update_negative_by(rotated)
        .update_zero_by(rotated);
    bus.write(opeland, rotated);
}

pub fn inx<T: CpuRegisters>(registers: &mut T) {
    let x = registers.get_X() + 1;
    registers
        .set_X(x)
        .update_negative_by(x)
        .update_zero_by(x);
}

pub fn iny<T: CpuRegisters>(registers: &mut T) {
    let y = registers.get_Y() + 1;
    registers
        .set_Y(y)
        .update_negative_by(y)
        .update_zero_by(y);
}

pub fn inc<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    let data = bus.read(opeland) + 1 as u8;
    registers.update_negative_by(data).update_zero_by(data);
    bus.write(opeland, data);
}

pub fn dex<T: CpuRegisters>(registers: &mut T) {
    let x = registers.get_X() - 1;
    registers
        .set_X(x)
        .update_negative_by(x)
        .update_zero_by(x);
}

pub fn dey<T: CpuRegisters>(registers: &mut T) {
    let y = registers.get_Y() - 1;
    registers
        .set_Y(y)
        .update_negative_by(y)
        .update_zero_by(y);
}

pub fn dec<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    let data = bus.read(opeland) - 1 as u8;
    registers.update_negative_by(data).update_zero_by(data);
    bus.write(opeland, data);
}

pub fn clc<T: CpuRegisters>(registers: &mut T) {
    registers.set_carry(false);
}

pub fn cli<T: CpuRegisters>(registers: &mut T) {
    registers.set_interrupt(false);
}

pub fn clv<T: CpuRegisters>(registers: &mut T) {
    registers.set_overflow(false);
}

pub fn sec<T: CpuRegisters>(registers: &mut T) {
    registers.set_carry(true);
}

pub fn sei<T: CpuRegisters>(registers: &mut T) {
    registers.set_interrupt(true);
}

pub fn brk<T: CpuRegisters, U: CpuBus>(opeland: Word, registers: &mut T, bus: &mut U) {
    let interrupt = registers.get_interrupt();
    registers.inc_PC();
    push_pc(registers, bus);
    registers.set_break(true);
    push_status(registers, bus);
    registers.set_interrupt(true);
    // Ignore interrupt when already set.
    if !interrupt {
        let fetched = bus.read_word(0xFFFE);
        registers.set_PC(fetched);
    }
    registers.dec_PC();
}

fn rotate_to_right<T: CpuRegisters>(registers: &mut T, v: Data) -> Data {
    ((v >> 1) | if registers.get_carry() { 0x80 } else { 0x00 }) as Data
}

fn rotate_to_left<T: CpuRegisters>(registers: &mut T, v: Data) -> Data {
    ((v << 1) | if registers.get_carry() { 0x01 } else { 0x00 }) as Data
}

fn push<T: CpuRegisters, U: CpuBus>(data: Data, registers: &mut T, bus: &mut U) {
    let addr = registers.get_SP() as Addr;
    bus.write((addr | 0x0100), data);
    registers.dec_SP();
}

fn push_status<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) {
    let status = registers.get_P();
    push(status, registers, bus);
}

fn pop<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) -> Data {
    registers.inc_SP();
    let addr = 0x0100 | registers.get_SP() as Addr;
    bus.read(addr)
}

fn push_pc<T: CpuRegisters, U: CpuBus>(registers: &mut T, bus: &mut U) {
    let pc = registers.get_PC();
    push((pc >> 8) as u8, registers, bus);
    push(pc as u8, registers, bus);
}

/*
    fn branch(&self, addr: Addr) {
        self.registers.borrow_mut().set_pc(addr);
    }


    fn pop_pc<R>(&self, read: R)
        where R: Fn(Addr) -> Data
    {
        let lower = self.pop(&read) as u16;
        let upper = self.pop(&read) as u16;
        self.registers.borrow_mut().set_pc(upper << 8 | lower);
    }



    fn jsr<W>(&self, opeland: Word, write: W)
        where W: Fn(Addr, Data)
    {
        let pc = self.registers.borrow().get_pc() - 1;
        self.push((pc >> 8) as u8, &write);
        self.push(pc as u8, &write);
        self.registers.borrow_mut().set_pc(opeland);
    }

    fn jmp(&self, opeland: Word) {
        self.registers.borrow_mut().set_pc(opeland);
    }

    fn rti<R>(&self, read: R)
        where R: Fn(Addr) -> Data
    {
        self.pop_status(&read);
        self.pop_pc(&read);
        self.registers.borrow_mut().set_reserved(true);
    }

    fn rts<R>(&self, read: R)
        where R: Fn(Addr) -> Data
    {
        self.pop_pc(&read);
        self.registers.borrow_mut().inc_pc();
    }

    fn bcc(&self, opeland: Word) {
        if !self.registers.borrow().get_status(StatusName::carry) {
            self.branch(opeland)
        }
    }

    fn bcs(&self, opeland: Word) {
        if self.registers.borrow().get_status(StatusName::carry) {
            self.branch(opeland)
        }
    }

    fn beq(&self, opeland: Word) {
        if self.registers.borrow().get_status(StatusName::zero) {
            self.branch(opeland)
        }
    }

    fn bmi(&self, opeland: Word) {
        if self.registers.borrow().get_status(StatusName::negative) {
            self.branch(opeland)
        }
    }

    fn bne(&self, opeland: Word) {
        if !self.registers.borrow().get_status(StatusName::zero) {
            self.branch(opeland)
        }
    }

    fn bpl(&self, opeland: Word) {
        if !self.registers.borrow().get_status(StatusName::negative) {
            self.branch(opeland)
        }
    }

    fn bvs(&self, opeland: Word) {
        if self.registers.borrow().get_status(StatusName::overflow) {
            self.branch(opeland)
        }
    }

    fn bvc(&self, opeland: Word) {
        if !self.registers.borrow().get_status(StatusName::overflow) {
            self.branch(opeland)
        }
    }

    fn cld(&self) {
        self.registers.borrow_mut().set_decimal(true);
    }

    fn sed(&self) {
        self.registers.borrow_mut().set_decimal(true);
    }
    */


#[cfg(test)]
mod test {
    use super::super::super::cpu_registers::Registers;
    use super::*;

    struct MockBus {
        pub mem: Vec<Data>,
    }

    impl MockBus {
        pub fn new() -> Self {
            MockBus { mem: vec!(0; 1024) }
        }
    }

    impl CpuBus for MockBus {
        fn read(&self, addr: Addr) -> Data {
            self.mem[addr as usize]
        }
        fn read_word(&self, addr: Addr) -> Word {
            let lower = self.read(addr) as u16;
            let upper = self.read(addr + 1) as u16;
            (upper << 8 | lower) as u16
        }
        fn write(&mut self, addr: Addr, data: Data) {
            self.mem[addr as usize] = data;
        }
    }

    #[test]
    fn test_lda_immidiate() {
        let mut reg = Registers::new();
        lda_imm(0xA5, &mut reg);
        assert_eq!(reg.get_A(), 0xA5);
    }

    #[test]
    fn test_lda() {
        let mut reg = Registers::new();
        let mut bus = MockBus::new();
        bus.mem[0xAA] = 0xA5;
        lda(0xAA, &mut reg, &mut bus);
        assert_eq!(reg.get_A(), 0xA5);
    }

    #[test]
    fn test_ldx_immidiate() {
        let mut reg = Registers::new();
        ldx_imm(0xA5, &mut reg);
        assert_eq!(reg.get_X(), 0xA5);
    }

    #[test]
    fn test_ldx() {
        let mut reg = Registers::new();
        let mut bus = MockBus::new();
        bus.mem[0xAA] = 0xA5;
        ldx(0xAA, &mut reg, &mut bus);
        assert_eq!(reg.get_X(), 0xA5);
    }

    #[test]
    fn test_ldy_immidiate() {
        let mut reg = Registers::new();
        ldy_imm(0xA5, &mut reg);
        assert_eq!(reg.get_Y(), 0xA5);
    }

    #[test]
    fn test_ldy() {
        let mut reg = Registers::new();
        let mut bus = MockBus::new();
        bus.mem[0xAA] = 0xA5;
        ldy(0xAA, &mut reg, &mut bus);
        assert_eq!(reg.get_Y(), 0xA5);
    }

    #[test]
    fn test_sta() {
        let mut reg = Registers::new();
        reg.set_A(0xA5);
        let mut bus = MockBus::new();
        sta(0xAA, &mut reg, &mut bus);
        assert_eq!(bus.mem[0xAA], 0xA5);
    }

    #[test]
    fn test_stx() {
        let mut reg = Registers::new();
        reg.set_X(0xA5);
        let mut bus = MockBus::new();
        stx(0xAA, &mut reg, &mut bus);
        assert_eq!(bus.mem[0xAA], 0xA5);
    }

    #[test]
    fn test_sty() {
        let mut reg = Registers::new();
        reg.set_Y(0xA5);
        let mut bus = MockBus::new();
        sty(0xAA, &mut reg, &mut bus);
        assert_eq!(bus.mem[0xAA], 0xA5);
    }

    #[test]
    fn test_txa() {
        let mut reg = Registers::new();
        reg.set_X(0xA5);
        txa(&mut reg);
        assert_eq!(reg.get_A(), 0xA5);
    }

    #[test]
    fn test_tax() {
        let mut reg = Registers::new();
        reg.set_A(0xA5);
        tax(&mut reg);
        assert_eq!(reg.get_X(), 0xA5);
    }

    #[test]
    fn test_tay() {
        let mut reg = Registers::new();
        reg.set_A(0xA5);
        tay(&mut reg);
        assert_eq!(reg.get_Y(), 0xA5);
    }


    #[test]
    fn test_tya() {
        let mut reg = Registers::new();
        reg.set_Y(0xA5);
        tya(&mut reg);
        assert_eq!(reg.get_A(), 0xA5);
    }

    #[test]
    fn test_txs() {
        let mut reg = Registers::new();
        reg.set_X(0xA5);
        txs(&mut reg);
        assert_eq!(reg.get_SP(), 0xA5);
    }

    #[test]
    fn test_tsx() {
        let mut reg = Registers::new();
        reg.set_SP(0xA5);
        tsx(&mut reg);
        assert_eq!(reg.get_X(), 0xA5);
    }

    #[test]
    fn test_php() {
        let mut reg = Registers::new();
        reg.set_SP(0xA5);
        let mut bus = MockBus::new();
        php(&mut reg, &mut bus);
        assert_eq!(bus.mem[0x01A5], 0x34);
    }

    #[test]
    fn test_plp() {
        let mut reg = Registers::new();
        reg.set_SP(0xA5);
        let mut bus = MockBus::new();
        bus.mem[0x1A6] = 0xA5;
        plp(&mut reg, &mut bus);
        assert_eq!(reg.get_P(), 0xA5);
    }

    #[test]
    fn test_pha() {
        let mut reg = Registers::new();
        reg.set_SP(0xA5).set_A(0x5A);
        let mut bus = MockBus::new();
        pha(&mut reg, &mut bus);
        assert_eq!(bus.mem[0x01A5], 0x5A);
    }

    #[test]
    fn test_pla() {
        let mut reg = Registers::new();
        reg.set_SP(0xA5);
        let mut bus = MockBus::new();
        bus.mem[0x1A6] = 0xAA;
        pla(&mut reg, &mut bus);
        assert_eq!(reg.get_A(), 0xAA);
    }

    #[test]
    fn test_adc_immediate() {
        let mut reg = Registers::new();
        reg.set_A(0x05);
        adc_imm(0xA5, &mut reg);
        assert_eq!(reg.get_A(), 0xAA);
    }

    #[test]
    fn test_adc() {
        let mut reg = Registers::new();
        reg.set_A(0x05);
        let mut bus = MockBus::new();
        bus.mem[0xA5] = 0xAA;
        adc(0xA5, &mut reg, &mut bus);
        assert_eq!(reg.get_A(), 0xAF);
    }

    #[test]
    fn test_sbc_immediate() {
        let mut reg = Registers::new();
        reg.set_A(0x10);
        sbc_imm(0x06, &mut reg);
        assert_eq!(reg.get_A(), 0x09);
    }

    #[test]
    fn test_sbc() {
        let mut reg = Registers::new();
        reg.set_A(0x10);
        let mut bus = MockBus::new();
        bus.mem[0xA5] = 0x06;
        sbc(0xA5, &mut reg, &mut bus);
        assert_eq!(reg.get_A(), 0x09);
    }

    #[test]
    fn test_cpx_immediate() {
        let mut reg = Registers::new();
        reg.set_X(0x05);
        cpx_imm(0x04, &mut reg);
        assert_eq!(reg.get_carry(), true);
    }

    #[test]
    fn test_cpx() {
        let mut reg = Registers::new();
        reg.set_X(0x05);
        let mut bus = MockBus::new();
        bus.mem[0xA5] = 0x04;
        cpx(0xA5, &mut reg, &mut bus);
        assert_eq!(reg.get_carry(), true);
    }

    #[test]
    fn test_cpy_immediate() {
        let mut reg = Registers::new();
        reg.set_Y(0x05);
        cpy_imm(0x04, &mut reg);
        assert_eq!(reg.get_carry(), true);
    }

    #[test]
    fn test_cpy() {
        let mut reg = Registers::new();
        reg.set_Y(0x05);
        let mut bus = MockBus::new();
        bus.mem[0xA5] = 0x04;
        cpy(0xA5, &mut reg, &mut bus);
        assert_eq!(reg.get_carry(), true);
    }

    #[test]
    fn test_cmp_immediate() {
        let mut reg = Registers::new();
        reg.set_A(0x05);
        cmp_imm(0x04, &mut reg);
        assert_eq!(reg.get_carry(), true);
    }

    #[test]
    fn test_cmp() {
        let mut reg = Registers::new();
        reg.set_A(0x05);
        let mut bus = MockBus::new();
        bus.mem[0xA5] = 0x04;
        cmp(0xA5, &mut reg, &mut bus);
        assert_eq!(reg.get_carry(), true);
    }

    #[test]
    fn test_and_immediate() {
        let mut reg = Registers::new();
        reg.set_A(0xA5);
        and_imm(0x05, &mut reg);
        assert_eq!(reg.get_A(), 0x05);
    }

    #[test]
    fn test_and() {
        let mut reg = Registers::new();
        reg.set_A(0xA5);
        let mut bus = MockBus::new();
        bus.mem[0xA5] = 0x05;
        and(0xA5, &mut reg, &mut bus);
        assert_eq!(reg.get_A(), 0x05);
    }

    #[test]
    fn test_eor_immediate() {
        let mut reg = Registers::new();
        reg.set_A(0xA5);
        eor_imm(0x05, &mut reg);
        assert_eq!(reg.get_A(), 0xA0);
    }

    #[test]
    fn test_eor() {
        let mut reg = Registers::new();
        reg.set_A(0xA5);
        let mut bus = MockBus::new();
        bus.mem[0xA5] = 0x05;
        eor(0xA5, &mut reg, &mut bus);
        assert_eq!(reg.get_A(), 0xA0);
    }

    #[test]
    fn test_ora_immediate() {
        let mut reg = Registers::new();
        reg.set_A(0xA0);
        ora_imm(0x05, &mut reg);
        assert_eq!(reg.get_A(), 0xA5);
    }

    #[test]
    fn test_ora() {
        let mut reg = Registers::new();
        reg.set_A(0xA0);
        let mut bus = MockBus::new();
        bus.mem[0xA5] = 0x05;
        ora(0xA5, &mut reg, &mut bus);
        assert_eq!(reg.get_A(), 0xA5);
    }

    #[test]
    fn test_asl_acc() {
        let mut reg = Registers::new();
        reg.set_A(0x55);
        asl_acc(&mut reg);
        assert_eq!(reg.get_A(), 0xAA);
    }

    #[test]
    fn test_asl() {
        let mut reg = Registers::new();
        let mut bus = MockBus::new();
        bus.mem[0x00] = 0x55;
        asl(0x00, &mut reg, &mut bus);
        assert_eq!(bus.mem[0x00], 0xAA);
    }

    #[test]
    fn test_lsr_acc() {
        let mut reg = Registers::new();
        reg.set_A(0xAA);
        lsr_acc(&mut reg);
        assert_eq!(reg.get_A(), 0x55);
    }

    #[test]
    fn test_lsr() {
        let mut reg = Registers::new();
        let mut bus = MockBus::new();
        bus.mem[0x00] = 0xAA;
        lsr(0x00, &mut reg, &mut bus);
        assert_eq!(bus.mem[0x00], 0x55);
    }

    #[test]
    fn test_rol_accumlator_with_carry() {
        let mut reg = Registers::new();
        reg.set_A(0x55).set_carry(true);
        rol_acc(&mut reg);
        assert_eq!(reg.get_A(), 0xAB);
    }

    #[test]
    fn test_ror_accumlator_with_carry() {
        let mut reg = Registers::new();
        reg.set_A(0x55).set_carry(true);
        ror_acc(&mut reg);
        assert_eq!(reg.get_A(), 0xAA);
    }

    #[test]
    fn test_ror_accumlator_without_carry() {
        let mut reg = Registers::new();
        reg.set_A(0x55);
        ror_acc(&mut reg);
        assert_eq!(reg.get_A(), 0x2A);
    }

    #[test]
    fn test_inx() {
        let mut reg = Registers::new();
        reg.set_X(0x55);
        inx(&mut reg);
        assert_eq!(reg.get_X(), 0x56);
    }

    #[test]
    fn test_iny() {
        let mut reg = Registers::new();
        reg.set_Y(0x55);
        iny(&mut reg);
        assert_eq!(reg.get_Y(), 0x56);
    }
    #[test]
    fn test_inc() {
        let mut reg = Registers::new();
        let mut bus = MockBus::new();
        bus.mem[0x10] = 0xAA;
        inc(0x10, &mut reg, &mut bus);
        assert_eq!(bus.mem[0x10], 0xAB);
    }

    #[test]
    fn test_dex() {
        let mut reg = Registers::new();
        reg.set_X(0x55);
        dex(&mut reg);
        assert_eq!(reg.get_X(), 0x54);
    }

    #[test]
    fn test_dey() {
        let mut reg = Registers::new();
        reg.set_Y(0x55);
        dey(&mut reg);
        assert_eq!(reg.get_Y(), 0x54);
    }

    #[test]
    fn test_dec() {
        let mut reg = Registers::new();
        let mut bus = MockBus::new();
        bus.mem[0x10] = 0xAA;
        dec(0x10, &mut reg, &mut bus);
        assert_eq!(bus.mem[0x10], 0xA9);
    }
}
