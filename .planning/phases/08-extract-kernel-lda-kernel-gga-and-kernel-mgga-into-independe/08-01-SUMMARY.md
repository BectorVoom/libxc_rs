---
phase: "08-extract-kernel-lda-kernel-gga-and-kernel-mgga-into-independe"
plan: "01"
subsystem: "workspace-structure"
tags: ["crate-extraction", "kernel-math", "workspace", "scaffold"]
dependency_graph:
  requires: []
  provides: ["crates/kernel-math", "crates/kernel-lda", "crates/kernel-gga", "crates/kernel-mgga"]
  affects: ["Cargo.toml", "workspace members"]
tech_stack:
  added: ["libxc-kernel-math crate", "libxc-kernel-lda crate", "libxc-kernel-gga crate", "libxc-kernel-mgga crate"]
  patterns: ["cargo workspace with path dependencies", "per-crate profile overrides"]
key_files:
  created:
    - "crates/kernel-math/Cargo.toml"
    - "crates/kernel-math/src/lib.rs"
    - "crates/kernel-math/src/constants.rs"
    - "crates/kernel-math/src/powers.rs"
    - "crates/kernel-math/src/piecewise.rs"
    - "crates/kernel-math/src/polynomials.rs"
    - "crates/kernel-math/src/erf.rs"
    - "crates/kernel-math/src/spin.rs"
    - "crates/kernel-math/src/dft_quantities.rs"
    - "crates/kernel-math/src/bspline.rs"
    - "crates/kernel-math/src/lambert_w.rs"
    - "crates/kernel-math/src/expint_e1.rs"
    - "crates/kernel-math/src/special.rs"
    - "crates/kernel-math/src/integrate.rs"
    - "crates/kernel-lda/Cargo.toml"
    - "crates/kernel-lda/src/lib.rs"
    - "crates/kernel-gga/Cargo.toml"
    - "crates/kernel-gga/src/lib.rs"
    - "crates/kernel-mgga/Cargo.toml"
    - "crates/kernel-mgga/src/lib.rs"
  modified:
    - "Cargo.toml"
decisions:
  - "kernel-math crate holds all 12 shared math building blocks so kernel family crates have no circular dep on main libxc_rs crate"
  - "kernel-lda gets bytemuck dep because BufArg/CubeCL handle patterns need it; kernel-gga and kernel-mgga do not yet"
  - "pre-existing libxc_rs main crate stack overflow (SIGSEGV from CubeCL macro expansion on large kernel count) is out of scope for this plan"
metrics:
  duration: "~15 minutes"
  completed_date: "2026-04-13"
---

# Phase 08 Plan 01: Create Kernel Crate Scaffolds Summary

Four new workspace crates created under `crates/` — kernel-math fully populated with all 12 math submodules from `src/math/`, and kernel-lda/gga/mgga scaffolded with placeholder lib.rs ready for Plan 02 kernel migration.

## What Was Built

- **crates/kernel-math**: Independent math crate (`libxc-kernel-math`) containing all 12 shared numerical building blocks (constants, powers, piecewise, polynomials, erf, spin, dft_quantities, bspline, lambert_w, expint_e1, special, integrate). Verified with `cargo check -p libxc-kernel-math` passing cleanly.

- **crates/kernel-lda, kernel-gga, kernel-mgga**: Scaffold crates (`libxc-kernel-lda/gga/mgga`) with correct Cargo.toml dependencies (cubecl + libxc-kernel-math) and placeholder lib.rs files. All 3 verified with individual `cargo check -p` passes.

- **Cargo.toml (workspace root)**: Updated to include all 4 new crates as workspace members, added path dependencies from main crate to all 4, and added `[profile.dev/test.package.*]` overrides to prevent OOM during dev builds.

## Commits

| Task | Commit | Description |
|------|--------|-------------|
| 1 | e68beea | feat(08-01): create kernel-math crate with math module sources |
| 2 | b297ec3 | feat(08-01): create kernel-lda/gga/mgga scaffolds and update workspace |

## Deviations from Plan

### Pre-existing Issues (Out of Scope)

The main `libxc_rs` crate fails `cargo check` with a rustc SIGSEGV (stack overflow from CubeCL macro expansion for the large kernel codebase). This failure exists on the base commit `f71e5b0` before any changes in this plan. It is not introduced by Plan 01 and is a known risk documented in the project (Phase 08 extraction is the solution to this exact problem). The plan's success criteria only requires `cargo check -p libxc-kernel-math` and the new crates to pass — all verified.

### Worktree Branch State

The worktree was initialized with the correct base commit `f71e5b0`. The initial `reset --soft` during branch verification left some files staged from prior commits; `git checkout HEAD -- <files>` was used to restore them to the correct working tree state before creating new files.

## Known Stubs

None — the placeholder `lib.rs` files in kernel-lda/gga/mgga are intentional empty stubs per the plan specification. Kernel module migration is the explicit scope of Plan 02.

## Threat Flags

None — pure structural crate scaffolding with no new input handling, APIs, or behavioral changes.

## Self-Check: PASSED

- crates/kernel-math/Cargo.toml: FOUND
- crates/kernel-math/src/lib.rs: FOUND
- crates/kernel-lda/Cargo.toml: FOUND
- crates/kernel-gga/Cargo.toml: FOUND
- crates/kernel-mgga/Cargo.toml: FOUND
- Commit e68beea: FOUND
- Commit b297ec3: FOUND
