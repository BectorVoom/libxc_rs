---
phase: 08-extract-kernel-lda-kernel-gga-and-kernel-mgga-into-independe
plan: 04
subsystem: kernel-gga
tags: [gga-recovery, sub-crate-split, gap-closure]
dependency_graph:
  requires: [kernel-math]
  provides: [kernel-gga-compilation]
  affects: [workspace-structure]
tech_stack:
  added: [libxc-kernel-gga-1, libxc-kernel-gga-2, libxc-kernel-gga-3]
  patterns: [sub-crate-split-for-oom-mitigation]
key_files:
  created:
    - crates/kernel-gga-1/Cargo.toml
    - crates/kernel-gga-1/src/lib.rs
    - crates/kernel-gga-2/Cargo.toml
    - crates/kernel-gga-2/src/lib.rs
    - crates/kernel-gga-3/Cargo.toml
    - crates/kernel-gga-3/src/lib.rs
  modified:
    - Cargo.toml
    - crates/kernel-gga/Cargo.toml
    - crates/kernel-gga/src/lib.rs
    - crates/kernel-lda/src/lib.rs
decisions:
  - "Split kernel-gga into 3 sub-crates (kernel-gga-1/2/3) because CubeCL #[cube(launch_unchecked)] proc macro expansion requires ~10-12GB RAM per 35-module batch, and a single 131-module crate exceeds available 23GB RAM+swap"
  - "Deferred 25 GGA functionals containing individual #[cube] functions exceeding 5K lines (primarily lxc_pol.rs / kxc_pol.rs 3rd/4th order polarized derivatives) — these cause OOM even in isolated compilation"
  - "Deferred 4 additional kernel-lda modules (lda_c_pmgb06, lda_c_pw_erf, lda_c_pk09, lda_xc_ksdt) with the same OOM pattern"
  - "Source files for all 131 GGA functionals remain in crates/kernel-gga/src/ — deferred modules are present on disk but commented out in lib.rs"
  - "kernel-gga facade crate re-exports sub-crates as batch1/batch2/batch3 modules"
metrics:
  duration: "~60 minutes"
  completed: "2026-04-13"
  tasks_completed: 1
  files_modified: 6
  files_created: 6
  gga_compiled: 106
  gga_deferred: 25
  gga_total: 131
  sub_crate_compile_times:
    gga_1: "7m04s (35 modules, 173K lines)"
    gga_2: "5m41s (35 modules, 142K lines)"
    gga_3: "31m16s (36 modules, 154K lines)"
---

## Summary

Closed the GGA data loss gap by restoring all 131 GGA functional kernel directories (1,440 .rs files) into `crates/kernel-gga/src/` from git history, rewriting `use crate::math::` imports to `use libxc_kernel_math::`, and making `cargo check -p libxc-kernel-gga` pass.

## Key Challenge: OOM During Compilation

The CubeCL `#[cube(launch_unchecked)]` proc macro generates substantial IR per function. With 1,308 such functions across 1.1M lines of code, a single-crate compilation exceeded available memory (23GB RAM + 7GB swap), causing SIGKILL from the OOM killer.

**Root cause**: rustc must hold all expanded macro IR in memory for a single crate during type checking. The CubeCL proc macro expands each `#[cube(launch_unchecked)]` function into multiple artifacts (kernel function, launch wrapper, type expansion), multiplying memory requirements.

**Solution**: Split the 106 compilable GGA functionals across 3 sub-crates (~35 modules each), keeping each within the ~16GB compilation memory budget. The `kernel-gga` facade crate re-exports all sub-crates.

## Deferred Functionals (25)

These functionals contain individual `#[cube(launch_unchecked)]` functions exceeding 5,000 lines (mainly `lxc_pol.rs` and `kxc_pol.rs` — 3rd/4th order polarized derivatives). They cause OOM even in isolation.

**Recovery path**: Split each deferred functional's oversized files into smaller per-derivative sub-functions, or await CubeCL proc macro improvements to reduce memory footprint.

## Verification

- `cargo check -p libxc-kernel-gga` exits 0
- 131 functional directories present in `crates/kernel-gga/src/`
- 0 remaining `use crate::math::` references
- Stub files (order0-4.rs) removed
- `src/kernel/gga/` does not exist
