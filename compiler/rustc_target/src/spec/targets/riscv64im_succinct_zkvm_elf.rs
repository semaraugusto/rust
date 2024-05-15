use crate::spec::{Cc, LinkerFlavor, Lld, PanicStrategy, RelocModel};
use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        data_layout: "e-m:e-p:64:64-i64:64-i128:128-n32:64-S128".into(),
        llvm_target: "riscv64".into(),
        pointer_width: 64,
        arch: "riscv64".into(),

        options: TargetOptions {
            os: "zkvm".into(),
            vendor: "succinct".into(),
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: Some("rust-lld".into()),
            cpu: "generic-rv64".into(),

            // Some crates (*cough* crossbeam) assume you have 64 bit
            // atomics if the target name is not in a hardcoded list.
            // Since zkvm is singlethreaded and all operations are
            // atomic, I guess we can just say we support 64-bit
            // atomics.
            max_atomic_width: Some(64),
            atomic_cas: true,

            features: "+m".into(),
            // features: "+m,+a,+f,+c".into(),
            llvm_abiname: "lp64".into(),
            executables: true,
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,
            singlethread: true,
            ..Default::default()
        },
    }
}
