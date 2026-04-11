---
phase: 03-input-output-and-evaluation-framework
plan: 03
subsystem: eval
tags: [workspace, mixed-functional, accumulation, scratch-buffer]
dependency_graph:
  requires: [03-02]
  provides: [EvaluationWorkspace, add_to_mix, evaluate_mixed_lda, AuxiliaryConfig]
  affects: [eval]
tech_stack:
  added: []
  patterns: [split_at_mut-for-non-overlapping-slices, MGGA-superset-scratch-sizing, weighted-accumulation]
key_files:
  created:
    - src/eval/workspace.rs
    - src/eval/mix.rs
  modified:
    - src/eval/mod.rs
    - src/error/mod.rs
decisions:
  - Scratch buffer uses MGGA-superset sizing with offset-based field access for correct LDA slice extraction from MGGA-ordered layout
  - Separate dispatch-then-accumulate loop avoids borrow checker issues with simultaneous scratch write and read
key_metrics:
  duration_seconds: 371
  completed: "2026-04-09T11:48:29Z"
  tasks_completed: 2
  tasks_total: 2
  tests_added: 16
  files_created: 2
  files_modified: 2
---

# Phase 03 Plan 03: Mixed Functional Workspace and Accumulation Summary

EvaluationWorkspace with MGGA-superset contiguous scratch buffer and weighted mixed-functional accumulation loop matching libxc mix_func.c behavior, verified with synthetic LDA_X auxiliaries.

## What Was Done

### Task 1: EvaluationWorkspace scratch buffer management
- Created `src/eval/workspace.rs` with `EvaluationWorkspace` struct
- Single contiguous `Vec<f64>` allocation sized for MGGA superset (767 * np for polarized)
- `zero_scratch()` for cross-contamination prevention (T-03-07, T-03-08 mitigation)
- `lda_scratch_mut()` returns `LdaScratch` with non-overlapping slices via `split_at_mut`
- Offset calculation accounts for full MGGA field ordering (vsigma, vlapl, vtau between vrho and v2rho2)
- GGA/MGGA scratch accessors stubbed as `todo!()` for Phase 4
- 7 tests covering allocation sizing, zeroing, slice lengths for both spin modes
- **Commit:** `aa0c3db`

### Task 2: Mixed functional accumulation and integration test
- Created `src/eval/mix.rs` with `add_to_mix`, `evaluate_mixed_lda`, and `AuxiliaryConfig`
- `add_to_mix(dst, coeff, src)` matches libxc mix_func.c line 54 exactly
- `evaluate_mixed_lda` dispatches each auxiliary into scratch, zeros between auxiliaries, accumulates weighted results
- Added `WorkspaceMismatch` error variant to `LibxcRsError`
- 3 unit tests for `add_to_mix` (basic, complementary weights, accumulation on existing)
- 6 integration tests for `evaluate_mixed_lda` (weight=1.0 match, 0.7+0.3 match, half weight, Vxc order, None fields, Fxc order)
- **Commit:** `6d69c2b`

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed MGGA-ordered scratch offset calculation**
- **Found during:** Task 2 (integration test failure)
- **Issue:** `lda_scratch_mut()` used naive `split_at_mut` chaining that assumed LDA fields were contiguous in scratch. In MGGA-ordered layout, vsigma/vlapl/vtau fields sit between vrho and v2rho2, causing v2rho2 slice to point at wrong data.
- **Fix:** Implemented `lda_field_offsets()` helper that computes correct byte offsets for each LDA field within the MGGA layout, using skip-based `split_at_mut` to reach the right positions.
- **Files modified:** `src/eval/workspace.rs`
- **Commit:** `6d69c2b`

## Verification

```
cargo test --lib eval       -> 32 passed, 0 failed
cargo test --lib input      -> 14 passed, 0 failed
cargo test --lib output     -> 27 passed, 0 failed
cargo clippy --lib -- -D warnings -> 0 warnings
```

## Known Stubs

| Stub | File | Line | Reason |
|------|------|------|--------|
| `gga_scratch_mut()` | src/eval/workspace.rs | ~137 | todo!() -- Phase 4 will implement GGA scratch accessor |
| `mgga_scratch_mut()` | src/eval/workspace.rs | ~145 | todo!() -- Phase 4 will implement MGGA scratch accessor |

These stubs do not prevent this plan's goal (LDA mixed evaluation) from being achieved. They are intentional placeholders for Phase 4.
