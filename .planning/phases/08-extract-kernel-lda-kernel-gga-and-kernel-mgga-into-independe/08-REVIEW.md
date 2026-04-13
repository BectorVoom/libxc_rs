---
phase: 08-extract-kernel-lda-kernel-gga-and-kernel-mgga-into-independe
reviewed: 2026-04-13T00:00:00Z
depth: standard
files_reviewed: 13
files_reviewed_list:
  - Cargo.toml
  - crates/kernel-math/Cargo.toml
  - crates/kernel-math/src/lib.rs
  - crates/kernel-lda/Cargo.toml
  - crates/kernel-lda/src/lib.rs
  - crates/kernel-lda/src/lda_x.rs
  - crates/kernel-lda/src/launch_lda_x.rs
  - crates/kernel-gga/Cargo.toml
  - crates/kernel-gga/src/lib.rs
  - crates/kernel-mgga/Cargo.toml
  - crates/kernel-mgga/src/lib.rs
  - src/kernel/mod.rs
  - src/math/mod.rs
findings:
  critical: 0
  warning: 3
  info: 4
  total: 7
status: issues_found
---

# Phase 08: Code Review Report

**Reviewed:** 2026-04-13
**Depth:** standard
**Files Reviewed:** 13
**Status:** issues_found

## Summary

Phase 08 extracted `libxc-kernel-math`, `libxc-kernel-lda`, `libxc-kernel-gga`, and `libxc-kernel-mgga` into independent workspace crates under `crates/`. The scaffold structure is sound: `src/math/mod.rs` and `src/kernel/mod.rs` correctly re-export the new crates, the workspace membership in the root `Cargo.toml` is complete, and the per-file module declarations in `kernel-lda/src/lib.rs` match the files present on disk.

Three warnings and four informational items were found. None are correctness or security issues at this scope, but two of the warnings represent real build-time or API-surface risks that should be resolved before the workspace is considered clean.

---

## Warnings

### WR-01: `bytemuck` declared as dependency in `kernel-lda` but not used

**File:** `crates/kernel-lda/Cargo.toml:9`
**Issue:** `bytemuck = { version = "1.25.0", features = ["derive"] }` is listed as a production dependency of `libxc-kernel-lda`. A full search of all `*.rs` files under `crates/kernel-lda/src/` finds zero references to `bytemuck`, `Pod`, or `Zeroable`. The dependency is unused dead weight that adds compile surface and may cause `cargo deny` or `cargo machete` failures in future audits.
**Fix:** Remove the `bytemuck` line from `crates/kernel-lda/Cargo.toml`. If `bytemuck` is needed for future kernel types (e.g., a `KernelParams` struct that must be cast to a byte buffer), add it back at that point with a comment explaining the use.

```toml
# Before (crates/kernel-lda/Cargo.toml)
[dependencies]
cubecl = { version = "0.9.0", default-features = false, features = ["cpu"] }
libxc-kernel-math = { path = "../kernel-math" }
bytemuck = { version = "1.25.0", features = ["derive"] }   # <-- unused

# After
[dependencies]
cubecl = { version = "0.9.0", default-features = false, features = ["cpu"] }
libxc-kernel-math = { path = "../kernel-math" }
```

---

### WR-02: `[profile.*]` sections in workspace member `Cargo.toml` files are silently ignored

**File:** `crates/kernel-math/Cargo.toml:9-15`, `crates/kernel-lda/Cargo.toml:14-20`, `crates/kernel-gga/Cargo.toml:10-16`, `crates/kernel-mgga/Cargo.toml:10-16`
**Issue:** All four member crates define their own `[profile.dev]` and `[profile.test]` sections. Per the Cargo reference, profile settings are only respected in the workspace root `Cargo.toml`; member crate profiles are silently ignored when the crate is built as part of a workspace. This means the memory-reduction intent (`debug = 0`, `codegen-units = 16`) does not apply to these crates during workspace builds. The root `Cargo.toml` already carries per-package profile overrides for `libxc-kernel-lda/gga/mgga` (lines 40-62), but is missing a `[profile.dev.package.libxc-kernel-math]` override, and the member-crate `[profile.*]` sections provide a false sense of coverage.
**Fix:** Remove the `[profile.*]` blocks from all four member `Cargo.toml` files (they have no effect). Then add the missing `libxc-kernel-math` override to the root `Cargo.toml` to keep all four crates consistent:

```toml
# Root Cargo.toml — add these two blocks alongside the existing kernel-lda/gga/mgga blocks:
[profile.dev.package.libxc-kernel-math]
debug = 0
codegen-units = 16

[profile.test.package.libxc-kernel-math]
debug = 0
codegen-units = 16
```

---

### WR-03: `#[allow(unused_variables)]` applied to `zeta_threshold` parameters that are actually used

**File:** `crates/kernel-lda/src/lda_x.rs` — all 10 kernel functions (lines 32, 60, 96, 138, 187, 241, and their polarized counterparts)
**Issue:** Every kernel function in `lda_x.rs` annotates its `zeta_threshold: f64` parameter with `#[allow(unused_variables)]`, yet every function body actively reads `zeta_threshold` (e.g., `pow_1_3(zeta_threshold)`, `1.0 <= zeta_threshold`). The attribute is incorrectly applied and creates a misleading code contract: readers (and future translators following this file as a template) will assume the parameter is vestigial. If CubeCL's `#[cube]` macro expansion causes the compiler to flag `zeta_threshold` as unused in some intermediate form, the correct fix is a CubeCL-specific suppression comment, not a blanket lint override on a used variable.
**Fix:** Remove the `#[allow(unused_variables)]` attribute from the `zeta_threshold` parameter in all 10 functions. If the CubeCL macro expansion genuinely causes a lint warning on this specific parameter, add a targeted file-level or function-level comment explaining why:

```rust
// Before
#[allow(unused_variables)] zeta_threshold: f64,

// After — remove the attribute entirely:
zeta_threshold: f64,
```

If a CubeCL macro issue forces a workaround, document it explicitly rather than using a generic lint suppression.

---

## Info

### IN-01: Root `Cargo.toml` missing `resolver = "2"` in `[workspace]`

**File:** `Cargo.toml:20-28`
**Issue:** The `[workspace]` section has no explicit `resolver = "2"` key. Cargo edition 2021+ workspaces default to resolver v2, so this is not a functional defect with edition 2024 crates. However, the omission is non-obvious and could confuse contributors using older Cargo versions or comparing against workspace templates that include the explicit key.
**Fix:** Add `resolver = "2"` for clarity:

```toml
[workspace]
resolver = "2"
members = [
    "xtask",
    "verify",
    ...
]
```

---

### IN-02: `libxc_rs` crate is missing from its own workspace `[members]`

**File:** `Cargo.toml:20-28`
**Issue:** The root package is `libxc_rs` (the `[package]` section at line 1), and it also declares `[workspace]`. The root package is implicitly a workspace member, so this is not broken — Cargo auto-includes the root. However, not listing it explicitly in `members` makes it easy to overlook when reading the membership roster. Workspace tooling (e.g., `cargo workspaces`, `cargo release`) sometimes behaves differently when the root is implicit vs. explicit.
**Fix:** Either leave as-is (acceptable), or add `"."` to `members` for clarity. Low priority.

---

### IN-03: `kernel-gga` and `kernel-mgga` contain only empty placeholder stubs

**File:** `crates/kernel-gga/src/lib.rs:9-13`, `crates/kernel-mgga/src/lib.rs:9-13`
**Issue:** Both crates declare five `pub mod orderN` modules (`order0`–`order4`), and each of those files contains only an `#![allow(dead_code)]` stub. The `lib.rs` doc comment says "GGA/MGGA kernel translations are added here in later plans," which is consistent with the phase scope. This is noted for awareness: these crates currently export nothing functional and any downstream code importing from them will compile but will produce empty results. Tests that depend on these crates being populated will fail silently (return no output rather than erroring) if the placeholder status is forgotten.
**Fix:** No immediate action required for this phase. In the phase that populates these crates, ensure at least one integration test is added per crate that asserts actual kernel output (non-zero values at a known grid point).

---

### IN-04: `src/lib.rs` exports `dispatch_lda` but no corresponding `dispatch_gga` or `dispatch_mgga`

**File:** `src/lib.rs:30`
**Issue:** The public API surface exports `dispatch_lda` (line 30), and the `GgaInput`/`MggaInput` types are also exported (lines 28-29), but there is no `dispatch_gga` or `dispatch_mgga` export. This is appropriate for the current phase if GGA/MGGA dispatch is not yet implemented, but the asymmetry means callers can construct `GgaInput` from the public API with no way to evaluate it. This creates a confusing partial API surface.
**Fix:** No action required in this phase. When GGA/MGGA dispatch is implemented, add the corresponding exports. If the exports of `GgaInput`/`MggaInput` from `src/lib.rs` are premature (i.e., callers cannot yet use them end-to-end), consider deferring those exports to the same phase that introduces the dispatch functions.

---

_Reviewed: 2026-04-13_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
