---
phase: 08-extract-kernel-lda-kernel-gga-and-kernel-mgga-into-independe
plan: 05
subsystem: kernel-math
tags: [clippy, code-quality, gap-closure]
dependency_graph:
  requires: []
  provides: [clippy-clean-kernel-math]
  affects: [workspace-wide-clippy-checks]
tech_stack:
  added: []
  patterns: [targeted-allow-attributes-for-cubecl-patterns]
key_files:
  created: []
  modified:
    - crates/kernel-math/src/bspline.rs
    - crates/kernel-math/src/expint_e1.rs
    - crates/kernel-math/src/integrate.rs
    - crates/kernel-math/src/lambert_w.rs
    - crates/kernel-math/src/special.rs
decisions:
  - "Used targeted function-level #[allow(unused_assignments)] for CubeCL #[cube] functions that require mutable result variables with initial values due to CubeCL macro constraints"
  - "Folded b2 Clenshaw recurrence declarations into first step (let mut b2 = b1; b1 = b0; ...) to eliminate unused initial assignment"
metrics:
  duration: "~25 minutes"
  completed: "2026-04-13"
  tasks_completed: 1
  files_modified: 5
---

# Phase 08 Plan 05: Kernel-Math Clippy Fix Summary

**One-liner:** Resolved all 145 clippy warnings in kernel-math via auto-fix plus targeted manual fixes for Clenshaw recurrence and CubeCL mutable result patterns.

## What Was Done

### Task 1: Auto-fix clippy warnings in kernel-math and verify tests

Ran `cargo clippy --fix --lib -p libxc-kernel-math --allow-dirty --allow-staged` which auto-fixed 132 `assign_op_pattern` warnings across all kernel-math source files (the `s = s + x` -> `s += x` pattern).

After auto-fix, 13 remaining warnings required manual fixes:

**Clenshaw recurrence `b2` initialization (9 instances):** The pattern `let mut b2: f64 = 0.0;` followed immediately by `b2 = b1;` triggered `unused_assignments`. Fixed by folding the declaration into the first Clenshaw step: `let mut b2 = b1; b1 = b0; b0 = twox * b1 - b2 + c;` in files `expint_e1.rs` and `special.rs`.

**CubeCL mutable result variables (4 instances):** Functions `lambert_w`, `xc_e1_scaled`, `xc_dilogarithm`, `xc_erfcx`, and `erfcx_y100` use the `let mut result = 0.0f64;` + if/else branches pattern. CubeCL `#[cube]` macro requires initialized mutable variables for result accumulation (no early return support), and Rust's borrow checker cannot verify exhaustiveness through the macro expansion. Added targeted `#[allow(unused_assignments)]` at the function level for each affected function.

## Verification

- `cargo clippy -p libxc-kernel-math` produces zero code-level warnings (4 profile warnings are cargo workspace configuration notices, not lint issues)
- `cargo test -p libxc-kernel-math` passes all 51 tests

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Pattern] Targeted function-level allow attributes for CubeCL pattern**
- **Found during:** Task 1 manual fix phase
- **Issue:** CubeCL `#[cube]` functions require `let mut result = 0.0f64;` pattern with all branches assigning before return (no early return support), but Rust's `unused_assignments` lint fires on the initial value. Uninitialized `let result;` causes E0381 through the `#[cube]` macro expansion.
- **Fix:** Used `#[allow(unused_assignments)]` at function level (not crate-level) for 4 affected functions. This is the minimal targeted suppression.
- **Files modified:** `lambert_w.rs`, `expint_e1.rs`, `special.rs`
- **Commit:** 087b90d

## Known Stubs

None.

## Threat Flags

None. Clippy fixes do not introduce new security-relevant surface.

## Self-Check: PASSED

- `crates/kernel-math/src/expint_e1.rs` - FOUND
- `crates/kernel-math/src/lambert_w.rs` - FOUND
- `crates/kernel-math/src/special.rs` - FOUND
- Commit 087b90d - FOUND
