---
phase: 08-extract-kernel-lda-kernel-gga-and-kernel-mgga-into-independe
verified: 2026-04-13T12:30:00Z
status: gaps_found
score: 8/13
overrides_applied: 0
gaps:
  - truth: "All GGA kernel source files exist under crates/kernel-gga/src/ and no longer under src/kernel/gga/"
    status: failed
    reason: "131 GGA functional kernel directories (1,443 .rs files) were present in src/kernel/gga/ at the phase 8 base commit (f71e5b0). Commit e68beea (Plan 01 Task 1: create kernel-math crate) deleted all 1,443 GGA source files. The migration in d10082d (Plan 02 Task 1) then only moved 5 stub files (order0-4.rs). As a result, crates/kernel-gga/src/ contains only 6 files (order0-4.rs + lib.rs) instead of ~1,443 translated kernel files. The deleted files are recoverable from git history at commit 24dbfbf."
    artifacts:
      - path: "crates/kernel-gga/src/"
        issue: "Contains only 6 files (order0-4.rs + lib.rs). Should contain ~1,443 files across 131 functional directories."
    missing:
      - "Restore 131 GGA functional kernel directories from git at 24dbfbf (e.g., git checkout 24dbfbf -- src/kernel/gga/)"
      - "Move restored files from src/kernel/gga/ to crates/kernel-gga/src/ with math import rewrite"
      - "Update crates/kernel-gga/src/lib.rs to declare all 131+ pub mod entries (replacing stub order0-4.rs entries)"
  - truth: "crates/kernel-gga/src/lib.rs contains pub mod gga_c_acgga (real kernel declarations)"
    status: failed
    reason: "crates/kernel-gga/src/lib.rs contains only pub mod order0 through order4 (stub placeholder modules). The required gga_c_acgga and all other 130+ GGA functional modules are absent because the source files were deleted in e68beea."
    artifacts:
      - path: "crates/kernel-gga/src/lib.rs"
        issue: "Contains only 5 stub module declarations (order0-4). Should contain 131+ pub mod declarations for translated GGA functionals."
    missing:
      - "After restoring GGA files, replace order0-4 stubs with real module declarations matching the original src/kernel/gga/mod.rs"
  - truth: "cargo test --workspace passes with zero failures (Plan 03)"
    status: failed
    reason: "cargo test --workspace fails without RUST_MIN_STACK environment variable. Even with RUST_MIN_STACK=67108864, cargo test -p libxc-kernel-lda times out (compilation of test binary exceeds 5 minutes without completing). The SUMMARY documents only cargo check -p libxc-kernel-lda passing — full test runs were blocked by concurrent build contention and did not complete. Individual crates that can compile (kernel-math: 51 tests pass; kernel-gga/mgga: 0 tests, pass trivially)."
    artifacts:
      - path: "crates/kernel-lda/src/"
        issue: "Test binary compilation exceeds available memory/time even with RUST_MIN_STACK=67108864. cargo test -p libxc-kernel-lda cannot be run to completion."
    missing:
      - "Verify cargo test -p libxc-kernel-lda actually completes and passes (likely requires build environment with more RAM or alternative test strategy)"
      - "Confirm whether the SIGSEGV is from test binary compilation or from run time; document minimum system requirements for running kernel-lda tests"
  - truth: "cargo clippy --workspace passes with no errors"
    status: failed
    reason: "cargo clippy -p libxc-kernel-math produces 145 warnings (13 in check mode, 145 in clippy mode including assign_op_pattern fixes). The main crate has #![deny(warnings)] so if clippy warnings are treated as errors across the workspace, this would fail. Cannot run clippy on kernel-lda or main libxc_rs due to SIGSEGV. Cannot verify workspace-wide clippy status."
    artifacts:
      - path: "crates/kernel-math/src/"
        issue: "145 clippy warnings generated; status of kernel-lda and main crate clippy unknown due to SIGSEGV."
    missing:
      - "Run clippy --fix on kernel-math to resolve the 145 warnings"
      - "Verify main crate's deny(warnings) does not propagate to make kernel-math warnings into errors"
---

# Phase 08: Extract kernel/lda, kernel/gga, kernel/mgga — Verification Report

**Phase Goal:** Extract kernel/lda, kernel/gga, and kernel/mgga into independent workspace crates under crates/, with a shared kernel-math crate. The main crate re-exports from sub-crates so the public API surface is unchanged.
**Verified:** 2026-04-13T12:30:00Z
**Status:** GAPS FOUND
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

#### Plan 01 Must-Haves

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | kernel-math crate compiles independently with `cargo check -p libxc-kernel-math` | VERIFIED | `cargo check -p libxc-kernel-math` exits 0 (58.9s, warnings only, pre-existing) |
| 2 | All 4 new crate directories exist under crates/ with valid Cargo.toml | VERIFIED | crates/kernel-math, kernel-lda, kernel-gga, kernel-mgga all present with correct Cargo.toml |
| 3 | Workspace Cargo.toml includes all 4 new members | VERIFIED | Lines 24-27 of Cargo.toml contain all 4 workspace members |

#### Plan 02 Must-Haves

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 4 | All LDA kernel source files exist under crates/kernel-lda/src/ | VERIFIED | 43 entries in crates/kernel-lda/src/ including all expected LDA files (lda_x.rs, launch_lda_x.rs, all subdirectories) |
| 5 | All GGA kernel source files exist under crates/kernel-gga/src/ | FAILED | Only 6 files (order0-4.rs + lib.rs). 1,443 GGA source files deleted in commit e68beea, never migrated. |
| 6 | All MGGA kernel source files exist under crates/kernel-mgga/src/ | VERIFIED (limited) | MGGA was stubs-only at f71e5b0 (base commit). Only 5 order files existed; all moved correctly. No MGGA kernel translations existed to lose. |
| 7 | Every kernel file uses libxc_kernel_math:: instead of crate::math:: | VERIFIED | `grep -r "use crate::math::" crates/` returns 0 |
| 8 | src/kernel/mod.rs re-exports the 3 kernel crates so existing paths resolve | VERIFIED | Contains `pub use libxc_kernel_lda as lda`, `pub use libxc_kernel_gga as gga`, `pub use libxc_kernel_mgga as mgga` |
| 9 | src/math/mod.rs re-exports from libxc_kernel_math | VERIFIED | All 12 re-exports present: `pub use libxc_kernel_math::constants`, etc. |
| 10 | cargo check --workspace passes with zero errors | PARTIAL | kernel-math/gga/mgga pass `cargo check`. kernel-lda requires `RUST_MIN_STACK=67108864`. Main libxc_rs crate fails with SIGSEGV (pre-existing, documented in SUMMARY as known constraint). |

#### Plan 03 Must-Haves

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 11 | cargo test --workspace passes with zero failures | FAILED | Without RUST_MIN_STACK: SIGSEGV during kernel-lda compilation. With RUST_MIN_STACK=67108864: kernel-math 51/51 pass, gga/mgga 0 tests (trivially pass), kernel-lda test build times out. Full workspace test cannot be confirmed. |
| 12 | cargo clippy --workspace passes with no errors | FAILED | kernel-math generates 145 clippy warnings. Cannot run clippy on kernel-lda or main crate (SIGSEGV). Full workspace clippy status unverifiable. |
| 13 | Inline tests in kernel-lda crate use libxc_rs:: paths (not crate::) | VERIFIED | lda_x.rs line 1266: `use libxc_rs::kernel::launch::{...}`. launch_lda_x.rs line 355: `use libxc_rs::kernel::launch::{...}`. Zero `use crate::kernel::launch` references in crates/. |

**Score:** 8/13 truths verified

### Critical Data Loss Finding

Commit `e68beea` (Plan 01 Task 1, "create kernel-math crate with math module sources") deleted **1,443 GGA kernel source files** across **131 functional subdirectories** from `src/kernel/gga/`. These files represented fully-translated GGA kernel implementations added in commit `24dbfbf` ("implement lda and gga kernel").

The subsequent migration commit `d10082d` (Plan 02 Task 1) only found 5 stub files in `src/kernel/gga/` and moved only those. The Plan 02 SUMMARY explicitly states "GGA had only placeholder stub files" — but this was because Plan 01 had already deleted the real files. The SUMMARY's characterization of the pre-migration state was incorrect.

**Recovery path:** The 1,443 GGA files are intact in git at commit `24dbfbf` and can be recovered with:
```
git checkout 24dbfbf -- src/kernel/gga/
```
Then apply the Plan 02 migration (copy to crates/kernel-gga/src/, rewrite imports, delete originals).

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `crates/kernel-math/Cargo.toml` | name = "libxc-kernel-math" | VERIFIED | Present, correct |
| `crates/kernel-math/src/lib.rs` | All 12 pub mod declarations | VERIFIED | All 12 present |
| `crates/kernel-lda/Cargo.toml` | name = "libxc-kernel-lda" | VERIFIED | Present, correct |
| `crates/kernel-gga/Cargo.toml` | name = "libxc-kernel-gga" | VERIFIED | Present, correct |
| `crates/kernel-mgga/Cargo.toml` | name = "libxc-kernel-mgga" | VERIFIED | Present, correct |
| `crates/kernel-lda/src/lib.rs` | pub mod lda_x | VERIFIED | Present |
| `crates/kernel-gga/src/lib.rs` | pub mod gga_c_acgga | FAILED | Contains only order0-4 stubs; gga_c_acgga was deleted in e68beea |
| `crates/kernel-mgga/src/lib.rs` | pub mod mgga_c_b88 | FAILED | MGGA had no mgga_c_b88 even before phase 8 (f71e5b0 only had stubs) |
| `crates/kernel-lda/src/lda_x.rs` | mod tests with libxc_rs paths | VERIFIED | Test imports use libxc_rs::kernel::launch:: |
| `crates/kernel-lda/src/launch_lda_x.rs` | mod tests with libxc_rs paths | VERIFIED | Test imports use libxc_rs::kernel::launch:: |
| `src/kernel/mod.rs` | libxc_kernel_lda as lda re-export | VERIFIED | `pub use libxc_kernel_lda as lda` present |
| `src/math/mod.rs` | libxc_kernel_math re-exports | VERIFIED | All 12 re-exports present |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `crates/kernel-math/src/lib.rs` | `crates/kernel-math/src/constants.rs` | `pub mod constants` | VERIFIED | All 12 pub mod declarations present |
| `Cargo.toml` | `crates/kernel-math` | workspace members array | VERIFIED | Line 24: `"crates/kernel-math"` |
| `src/kernel/mod.rs` | `crates/kernel-lda` | `pub use libxc_kernel_lda as lda` | VERIFIED | Exact pattern present at line 1 |
| `src/math/mod.rs` | `crates/kernel-math` | `pub use libxc_kernel_math::*` | VERIFIED | All 12 re-exports present |
| `crates/kernel-lda/src/lda_x.rs #[cfg(test)]` | `libxc_rs::kernel::launch` | dev-dependency on libxc_rs | VERIFIED | `use libxc_rs::kernel::launch::` at line 1266 |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|----------|---------|--------|--------|
| kernel-math compiles independently | `cargo check -p libxc-kernel-math` | Finished in 0.84s, warnings only | PASS |
| kernel-gga compiles independently | `cargo check -p libxc-kernel-gga` | Finished in 0.11s | PASS |
| kernel-mgga compiles independently | `cargo check -p libxc-kernel-mgga` | Finished in 0.11s | PASS |
| kernel-lda compiles independently | `RUST_MIN_STACK=67108864 cargo check -p libxc-kernel-lda` | Finished in 58.9s | PASS (requires env var) |
| kernel-math tests pass | `cargo test -p libxc-kernel-math` | 51 passed, 0 failed | PASS |
| kernel-lda test binary builds | `cargo test -p libxc-kernel-lda --no-run` | Times out at 120s even with RUST_MIN_STACK=67108864 | FAIL |
| No crate::math:: imports remain in crates/ | `grep -r "use crate::math::" crates/` | 0 results | PASS |
| No crate::kernel::launch imports remain in crates/ | `grep -r "use crate::kernel::launch" crates/` | 0 results | PASS |
| All 6 documented commits exist in git history | `git log e68beea b297ec3 d10082d 8b33e89 36624de eba1af8` | All found | PASS |

### Requirements Coverage

No requirement IDs were declared for this phase (structural refactoring).

### Anti-Patterns Found

| File | Pattern | Severity | Impact |
|------|---------|----------|--------|
| `crates/kernel-gga/src/lib.rs` | Only pub mod order0-4 (stubs) instead of 131 real kernel modules | BLOCKER | GGA kernels untranslatable from independent crate; entire GGA family lost from crates/kernel-gga/ |
| `crates/kernel-gga/src/order*.rs` | `#![allow(dead_code)]` placeholder stubs | BLOCKER | GGA crate has no real content; will not satisfy Phase 4 GGA bulk translation goal |

### Human Verification Required

None — all items were deterministically verifiable from the codebase state.

### Gaps Summary

**Two independent gaps blocking goal achievement:**

**Gap 1 — GGA kernel data loss (Critical):**
Commit `e68beea` (Plan 01, Task 1) deleted 1,443 GGA kernel source files that had been translated and committed in `24dbfbf`. The migration in Plan 02 only saw the leftover stubs (order0-4.rs) and correctly moved those, but the translated GGA kernel content was already gone. `crates/kernel-gga/src/` now contains 6 files instead of the expected ~1,443. The phase goal requires GGA kernel files to exist in `crates/kernel-gga/`. The files are fully recoverable from git at `24dbfbf`.

**Gap 2 — Test suite and clippy unverifiable for kernel-lda (Significant):**
Plan 03's must-haves require `cargo test --workspace` and `cargo clippy --workspace` to pass. The kernel-lda crate cannot compile its test binary in the available time (SIGSEGV without RUST_MIN_STACK, timeout with it). The SUMMARY documents that only `cargo check` was verified, not `cargo test`. This leaves the correctness of the test import fix (Plan 03 Task 1) unconfirmed at the test execution level.

**Deferred scope (not gaps):**
The `mgga_c_b88` must-have from Plan 02 artifacts expected a subdirectory structure that never existed in the codebase at any point — MGGA was stubs-only since before this phase. The plan artifact requirement was aspirational. The extraction of what actually existed (stubs) was performed correctly.

---

_Verified: 2026-04-13T12:30:00Z_
_Verifier: Claude (gsd-verifier)_
