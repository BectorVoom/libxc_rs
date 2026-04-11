---
phase: 03-input-output-and-evaluation-framework
fixed_at: 2026-04-09T00:00:00Z
review_path: .planning/phases/03-input-output-and-evaluation-framework/03-REVIEW.md
iteration: 1
findings_in_scope: 5
fixed: 5
skipped: 0
status: all_fixed
---

# Phase 03: Code Review Fix Report

**Fixed at:** 2026-04-09
**Source review:** .planning/phases/03-input-output-and-evaluation-framework/03-REVIEW.md
**Iteration:** 1

**Summary:**
- Findings in scope: 5
- Fixed: 5
- Skipped: 0

## Fixed Issues

### CR-01: `unwrap()` on kernel launch result panics instead of propagating error

**Files modified:** `src/kernel/lda/launch_lda_x.rs`, `src/eval/dispatch.rs`, `src/error/mod.rs`
**Commit:** 06108ac
**Applied fix:** Changed all 10 kernel launch wrapper functions in `launch_lda_x.rs` from returning `()` to returning `Result<(), Box<dyn std::error::Error>>`, replacing `.unwrap()` with `?` inside each unsafe block. Added `KernelLaunchFailed { reason: String }` variant to `LibxcRsError` in `error/mod.rs`. Updated all 10 call sites in `dispatch.rs` to propagate errors via `.map_err(map_launch_err)?`. Test call sites use `.unwrap()` which is appropriate for tests.

### WR-01: `zk` readback silently discarded when output.zk is None

**Files modified:** `src/eval/dispatch.rs`
**Commit:** 0bf56ef
**Applied fix:** Added detailed documentation to the `dispatch_lda` function docstring explaining that `zk` is always computed by every LDA kernel variant, that passing `None` for `output.zk` wastes one buffer allocation but does not skip computation, and that higher-order derivative fields are truly optional.

### WR-02: `copy_from_slice` panics if output buffer is shorter than GPU result

**Files modified:** `src/eval/dispatch.rs`
**Commit:** c0fa983
**Applied fix:** Added explicit `buf.len() != result.len()` checks before each of the 5 `copy_from_slice` calls in the readback section. On mismatch, returns `LibxcRsError::OutputBufferSizeMismatch` with the field name, expected length, and actual length. The existing `OutputBufferSizeMismatch` error variant was already defined and reused here.

### WR-03: Double zeroing of output buffers in `evaluate_mixed_lda`

**Files modified:** `src/eval/dispatch.rs`, `src/eval/mix.rs`
**Commit:** 6b5d2f7
**Applied fix:** Added clarifying comments in both files documenting the zeroing invariant. In `dispatch.rs`, the comment explains that dispatch_lda is authoritative for zeroing its own output buffers, and that the double-zero is intentional to keep dispatch_lda self-contained. In `mix.rs`, the comment explains that `workspace.zero_scratch()` prevents cross-contamination between accumulation readback and the next dispatch call.

### WR-04: `add_to_mix` silently truncates when `dst` and `src` have different lengths

**Files modified:** `src/eval/mix.rs`
**Commit:** cf2f8ae
**Applied fix:** Added `debug_assert_eq!(dst.len(), src.len(), "add_to_mix: dst and src must have equal length")` at the top of the function. This catches length mismatches in debug/test builds without adding runtime overhead in release builds.

---

_Fixed: 2026-04-09_
_Fixer: Claude (gsd-code-fixer)_
_Iteration: 1_
