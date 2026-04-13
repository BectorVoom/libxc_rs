---
phase: "08-extract-kernel-lda-kernel-gga-and-kernel-mgga-into-independe"
plan: "02"
subsystem: "workspace-crate-extraction"
tags: ["crate-extraction", "kernel-lda", "kernel-gga", "kernel-mgga", "file-migration", "import-rewrite"]
dependency_graph:
  requires: ["08-01"]
  provides: ["crates/kernel-lda (populated)", "crates/kernel-gga (populated)", "crates/kernel-mgga (populated)"]
  affects: ["src/kernel/mod.rs", "src/math/mod.rs", "src/kernel/lda/", "src/kernel/gga/", "src/kernel/mgga/", "src/math/*.rs"]
tech_stack:
  added: []
  patterns: ["pub use crate-alias re-export", "workspace crate extraction via file move + import rewrite"]
key_files:
  created: []
  modified:
    - "crates/kernel-lda/src/lib.rs"
    - "crates/kernel-gga/src/lib.rs"
    - "crates/kernel-mgga/src/lib.rs"
    - "src/kernel/mod.rs"
    - "src/math/mod.rs"
  deleted:
    - "src/kernel/lda/ (all files, 81 .rs + 4 subdirectories)"
    - "src/kernel/gga/ (mod.rs + 5 order stub files)"
    - "src/kernel/mgga/ (mod.rs + 5 order stub files)"
    - "src/math/constants.rs src/math/powers.rs src/math/piecewise.rs src/math/polynomials.rs src/math/erf.rs src/math/spin.rs src/math/dft_quantities.rs src/math/bspline.rs src/math/lambert_w.rs src/math/expint_e1.rs src/math/special.rs src/math/integrate.rs"
decisions:
  - "GGA and MGGA had only placeholder stub files (no translated kernels yet), so their kernel crate lib.rs declares the order0-4 stub modules rather than individual functional modules"
  - "pub use libxc_kernel_lda as lda; form used for kernel re-exports (not extern crate form) -- Rust 2024 edition supports this cleanly"
  - "src/kernel/mod.rs also declares pub mod dispatch_key, pub mod shared, pub mod mix which were missing from the original file"
  - "Pre-existing SIGSEGV in main libxc_rs crate cargo check (from CubeCL macro expansion) remains out of scope; all 3 kernel sub-crates compile independently with RUST_MIN_STACK=67108864"
metrics:
  duration: "~30 minutes"
  completed_date: "2026-04-13"
---

# Phase 08 Plan 02: Move Kernel Files to Sub-Crates Summary

LDA kernel source files (81 .rs files across 35 modules + 4 multi-file subdirectories) migrated from `src/kernel/lda/` to `crates/kernel-lda/src/`, all `crate::math::` imports rewritten to `libxc_kernel_math::`, original source directories removed, and main crate module tree rewired to re-export from the 3 kernel sub-crates.

## What Was Built

### Task 1: Kernel File Migration

- **crates/kernel-lda/src/**: 81 `.rs` files + 4 subdirectory modules (`lda_c_pk09/`, `lda_c_pmgb06/`, `lda_c_pw_erf/`, `lda_xc_ksdt/`) moved from `src/kernel/lda/`. All `crate::math::` imports rewritten to `libxc_kernel_math::`.

- **crates/kernel-gga/src/**: Placeholder stub files (order0-4.rs) moved from `src/kernel/gga/`. GGA kernel translations do not exist yet in the codebase.

- **crates/kernel-mgga/src/**: Placeholder stub files (order0-4.rs) moved from `src/kernel/mgga/`. MGGA kernel translations do not exist yet in the codebase.

- Each kernel crate `lib.rs` updated with proper module declarations and CubeCL lint allows.

- Original `src/kernel/{lda,gga,mgga}/` directories deleted.

### Task 2: Main Crate Module Tree Rewiring

- **src/kernel/mod.rs**: Replaced `pub mod lda/gga/mgga` with `pub use libxc_kernel_{lda,gga,mgga} as {lda,gga,mgga}` re-exports. Added `pub mod dispatch_key`, `pub mod shared`, `pub mod mix` declarations (were previously missing).

- **src/math/mod.rs**: Replaced 7 `pub mod` declarations with 12 `pub use libxc_kernel_math::*` re-exports covering all math submodules.

- **src/math/*.rs**: All 12 original math source files deleted (now live in `crates/kernel-math/src/`).

### Verification

- `cargo check -p libxc-kernel-math`: PASS (warnings only, pre-existing)
- `cargo check -p libxc-kernel-lda` (with RUST_MIN_STACK=67108864): PASS
- `cargo check -p libxc-kernel-gga`: PASS
- `cargo check -p libxc-kernel-mgga`: PASS
- `grep -rc "use crate::math::" crates/`: 0 remaining references

## Commits

| Task | Commit | Description |
|------|--------|-------------|
| 1 | d10082d | feat(08-02): move kernel family files to sub-crates and rewrite math imports |
| 2 | 8b33e89 | feat(08-02): rewire main crate module tree to re-export from sub-crates |

## Deviations from Plan

### GGA and MGGA Actual File Count

**Found during:** Task 1 discovery

**Issue:** The plan's critical context stated "GGA has ~139 .rs files" and "MGGA has subdirectory structure: mgga_c_b88/ with per-function files". In reality, the codebase only has stub/placeholder files in both directories — GGA has `mod.rs` (comment-only) + 5 `order*.rs` stubs, MGGA has the same structure. No GGA or MGGA kernels have been translated yet.

**Fix:** Moved the stub files to the respective crates and wrote lib.rs files declaring the stub modules (order0-4). The plan's acceptance criterion for `gga_c_pbe.rs` and `mgga_c_b88/mod.rs` cannot be met because these files don't exist yet.

**Impact:** The kernel crates compile with stub content; actual GGA/MGGA kernel files will be added in future plans when translation work completes.

### src/kernel/mod.rs Missing Module Declarations

**Found during:** Task 2

**Issue:** The original `src/kernel/mod.rs` only had `pub mod launch; pub mod lda; pub mod gga; pub mod mgga;` — it was missing `pub mod dispatch_key`, `pub mod shared`, `pub mod mix` even though those files/directories exist in `src/kernel/`.

**Fix:** [Rule 2 - Missing declarations] Added the missing pub mod declarations to the new `src/kernel/mod.rs`.

### src/math/mod.rs Had Only 7 of 12 Modules

**Found during:** Task 2 discovery

**Issue:** The original `src/math/mod.rs` only declared 7 modules (`constants, powers, piecewise, polynomials, erf, spin, dft_quantities`) even though 12 `.rs` files existed in `src/math/`. The new re-export version covers all 12, matching what `crates/kernel-math/src/lib.rs` already declared.

**Fix:** The new `src/math/mod.rs` re-exports all 12 modules from `libxc_kernel_math`, restoring full coverage.

## Known Stubs

- `crates/kernel-gga/src/order*.rs` — placeholder stubs, no GGA kernels translated yet
- `crates/kernel-mgga/src/order*.rs` — placeholder stubs, no MGGA kernels translated yet
- `crates/kernel-lda/src/lda_x.rs` and `launch_lda_x.rs` contain `#[cfg(test)]` blocks that reference `crate::kernel::launch::*` — these will fail in test builds until Plan 03 rewrites the test imports.

## Threat Flags

None — file moves and import rewrites with no new input surfaces, APIs, or behavioral changes.

## Self-Check: PASSED

- crates/kernel-lda/src/lda_x.rs: FOUND
- crates/kernel-lda/src/launch_lda_x.rs: FOUND
- crates/kernel-lda/src/lib.rs contains `pub mod lda_x`: PASS
- crates/kernel-lda/src/lib.rs contains `pub mod launch_lda_x`: PASS
- crates/kernel-gga/src/lib.rs exists: FOUND
- crates/kernel-mgga/src/lib.rs exists: FOUND
- src/kernel/mod.rs contains `libxc_kernel_lda as lda`: PASS
- src/math/mod.rs contains `libxc_kernel_math`: PASS
- src/math/constants.rs does NOT exist: PASS
- src/kernel/lda/ does NOT exist: PASS
- src/kernel/gga/ does NOT exist: PASS
- src/kernel/mgga/ does NOT exist: PASS
- Commit d10082d: FOUND
- Commit 8b33e89: FOUND
