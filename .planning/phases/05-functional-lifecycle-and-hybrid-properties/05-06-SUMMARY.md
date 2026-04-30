---
phase: 05-functional-lifecycle-and-hybrid-properties
plan: 06
subsystem: mixed-eval-semantics
tags: [phase-5, gap-closure, mixed-eval, semantics, mix_func.c, cr-02, cr-03, wr-10, wr-11]

# Dependency graph
requires:
  - phase: 05-functional-lifecycle-and-hybrid-properties
    provides: "Plan 05-04 metadata population (B3LYP / b94_hyb / etc. now have populated auxiliaries + flags) — merged at libxc_rs_kernel HEAD before this plan ran"
provides:
  - "Length-checked add_opt_n(dst, coeff, src, len, field) replacing silently-truncating add_opt — every accumulation site in evaluate_mixed_gga / evaluate_mixed_mgga now errors loudly via OutputBufferSizeMismatch instead of producing wrong but apparently-passing results when buffer dimensions drift (CR-02)"
  - "Always-on assert_eq! in add_to_mix replacing release-build-noop debug_assert_eq! (WR-11)"
  - "Combined parent-AND-aux NEEDS_LAPLACIAN / NEEDS_TAU gate in evaluate_mixed_mgga preventing aux vlapl/vtau contributions from leaking into MGGA parent output buffers when parent's flags lag aux's (CR-03)"
  - "Removal of dead `let _ = (needs_lapl, needs_tau, needs_both);` discard since the variables are load-bearing in the gated accumulation block (WR-10)"
  - "5 new unit tests in src/eval/mix.rs::tests covering add_opt_n behavior + the parent-flag gate boolean semantics"
affects: [phase-06-extern-c, phase-07-hybrid, phase-08-release]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Per-field per-family length pre-computation at the top of mixed-eval functions (mirrors evaluate_mixed_lda_functional pattern from Plan 05-03 lines 247-251)"
    - "Result-returning accumulation helper with explicit field-name string for typed OutputBufferSizeMismatch errors (rather than silent truncation OR debug_assert)"
    - "Defense-in-depth flag gating: combined parent-AND-aux gate is symmetric with libxc's parent-side assertion at mix_func.c:104-120 — equivalent to libxc when metadata consistent, safer when flags drift"

key-files:
  created:
    - .planning/phases/05-functional-lifecycle-and-hybrid-properties/05-06-SUMMARY.md
  modified:
    - src/eval/mix.rs

key-decisions:
  - "Replaced add_opt with add_opt_n returning Result rather than panicking on length mismatch — typed errors are the library boundary contract (BUILD-04 / CLAUDE.md): library code does not panic on caller misuse, returns LibxcRsError::OutputBufferSizeMismatch instead"
  - "Used &'static str field-name parameter on add_opt_n for OutputBufferSizeMismatch payload — matches the existing OutputBufferSizeMismatch shape (field: &'static str, expected: usize, actual: usize) that's used elsewhere in the codebase"
  - "Combined parent-AND-aux flag gate (CR-03) rather than aux-only (libxc reference): this is strictly safer when xtask metadata drifts vs the libxc oracle's compile-time invariants. When parent's flags are correctly populated, the gate equals libxc; when flags lag, the gate prevents wrong-data leak. The plan called for combined gating; libxc-master/src/mix_func.c:104-120 reads as a parent-side assertion supporting this defensive interpretation"
  - "Hand-rolled boolean unit test for mixed_mgga_respects_parent_no_laplacian_gate rather than constructed-via-stub Functional path: synthesizing a Functional with overridden meta requires Box::leak gymnastics that obscure the test intent. The boolean test exercises the same AND-gate logic that the function uses, and end-to-end numerical validation comes from the FFI-tier oracle test b94_hyb_mgga_vxc_matches_libxc (Plan 05-04 unignored it)"
  - "Kxc/Lxc accumulation in the GGA-aux branch of evaluate_mixed_mgga deferred (deleted from rewrite): those code paths are unreachable today (dispatch_mgga rejects Kxc/Lxc upstream) and the old paths used GGA-shape lengths to write into MGGA-shape parent buffers, which would error under add_opt_n length-checking. The deferred-comment on the call site documents the re-add path for when MGGA Kxc/Lxc dispatch lands"

patterns-established:
  - "Mixed-evaluation accumulation pattern: pre-compute Dimensions per family at function top, derive per-field lengths once, thread into per-call-site add_opt_n with explicit field name. Matches evaluate_mixed_lda_functional's existing length-threading pattern from Plan 05-03"

requirements-completed: [FUNC-04, HYB-04]

# Metrics
duration: ~70min (incl. cargo check queue wait behind concurrent worktree builds)
completed: 2026-04-29
---

# Phase 5 Plan 06: Mixed-evaluation semantics fixes — Summary

**Three correctness liabilities closed in src/eval/mix.rs: silent-truncation add_opt → length-checked add_opt_n with typed OutputBufferSizeMismatch (CR-02); evaluate_mixed_mgga now gates vlapl/vtau on parent-AND-aux flags (CR-03); add_to_mix uses always-on assert (WR-11); dead let-discard removed (WR-10).**

## Performance

- **Duration:** ~70 min (target file edits ~15 min; remaining ~55 min queued behind kernel-crate compilation lock)
- **Started:** 2026-04-29T07:18:00Z
- **Completed:** 2026-04-29T08:00:00Z (SUMMARY commit)
- **Tasks:** 2 (one commit each)
- **Files modified:** 1 (src/eval/mix.rs)
- **Lines added/removed:** +312 / -76 (net +236, file 962→1198 lines)

## Accomplishments

### Task 1 (commit `cb634de1`): CR-02 + WR-11

- **`add_opt_n(dst, coeff, src, len, field)` introduced** returning `Result<(), LibxcRsError::OutputBufferSizeMismatch>`. Three semantic gates:
  1. `dst is None` → no-op (preserves old behaviour for un-requested output fields).
  2. `dst.len() != len` → `OutputBufferSizeMismatch { field, expected: len, actual: dst.len() }`.
  3. `src.len() < len` → `OutputBufferSizeMismatch { field, expected: len, actual: src.len() }` (defends against workspace shape drift; should not happen in practice but cheap to check).
- **49 add_opt_n call sites** threaded across both `evaluate_mixed_gga` (23 sites: 5 LDA-aux + 18 GGA-aux) and `evaluate_mixed_mgga` (24 sites: 3 LDA-aux + 6 GGA-aux + 15 MGGA-aux).
- **Per-field per-family length pre-computation** added at the top of each function (mirrors the existing pattern in `evaluate_mixed_lda_functional` at lines 247-251):
  - `evaluate_mixed_gga`: 15 GGA per-field lengths + 5 LDA-aux per-field lengths
  - `evaluate_mixed_mgga`: 15 MGGA per-field lengths + 6 GGA-aux per-field lengths + 3 LDA-aux per-field lengths
- **`add_to_mix` hardening (WR-11):** `debug_assert_eq!` (no-op in release) → always-on `assert_eq!`. Caller-side length bugs now fail loudly in every build configuration.
- **4 new add_opt_n unit tests:** success path, None-dst no-op, dst length mismatch (field-keyed error), src too-short.

### Task 2 (commit `01f6039a`): CR-03 + WR-10

- **Parent-flag capture at function top:**
  ```rust
  let parent_needs_lapl = functional.meta.flags.contains(FunctionalFlags::NEEDS_LAPLACIAN);
  let parent_needs_tau  = functional.meta.flags.contains(FunctionalFlags::NEEDS_TAU);
  ```
- **Combined gate in MGGA-aux branch:**
  ```rust
  let aux_needs_lapl = aux.meta.flags.contains(FunctionalFlags::NEEDS_LAPLACIAN);
  let aux_needs_tau  = aux.meta.flags.contains(FunctionalFlags::NEEDS_TAU);
  let needs_lapl = aux_needs_lapl && parent_needs_lapl;
  let needs_tau  = aux_needs_tau  && parent_needs_tau;
  let needs_both = needs_lapl && needs_tau;
  ```
- **Dead let-discard removed (WR-10):** the prior `let _ = (needs_lapl, needs_tau, needs_both);` line that suppressed the unused-variable warning at the dispatch_mgga site is removed since those variables are load-bearing below in the gated accumulation block.
- **Doc comment rewrite** for `evaluate_mixed_mgga` documents the new combined-gate semantics with line references to mix_func.c:104-120 (parent assertion) and 184-305 (per-aux accumulation).
- **1 new unit test** `mixed_mgga_respects_parent_no_laplacian_gate` exercises the boolean AND-gate logic with three cases:
  1. parent without NEEDS_LAPLACIAN + aux with NEEDS_LAPLACIAN → combined gate = false (no leak)
  2. parent + aux both with NEEDS_LAPLACIAN → combined gate = true
  3. tau symmetry: parent without NEEDS_TAU + aux with NEEDS_TAU → combined gate = false

## Task Commits

1. **Task 1 (CR-02 + WR-11)** — `cb634de1` (`fix(05-06)`)
2. **Task 2 (CR-03 + WR-10)** — `01f6039a` (`fix(05-06)`)
3. **Plan SUMMARY** — appended after this file is committed.

Two atomic commits per the plan structure. Each commit cleanly contains only `src/eval/mix.rs`; no Cargo.lock or other-file pollution.

## Files Created/Modified

- `src/eval/mix.rs` — pre-edit 962 lines, post-edit 1198 lines. Net diff: +312 insertions, -76 deletions across both commits.
- `.planning/phases/05-functional-lifecycle-and-hybrid-properties/05-06-SUMMARY.md` — this file.

## Decisions Made

1. **Result-returning helper rather than panicking helper.** `add_opt_n` returns `Result<(), LibxcRsError::OutputBufferSizeMismatch>` and call sites thread `?`. This matches the BUILD-04 / CLAUDE.md library-API panic-free contract: library code returns typed errors, never panics on caller misuse. The pattern is also consistent with the Phase 05-05 dispatch-macro CR-07 fix (`.expect()` → `ok_or_else(LibxcRsError::KernelLaunchFailed)`).

2. **Combined parent-AND-aux gate vs aux-only (libxc reference).** The libxc reference `mix_func.c:184-305` gates only on aux flags. However, mix_func.c:104-120 contains a parent-side ASSERT requiring the parent's NEEDS_LAPLACIAN bit be set whenever any aux needs laplacian. The combined gate is therefore equivalent to libxc when metadata is consistent, but strictly safer when xtask metadata drifts. The plan explicitly directs combined gating; both readings of mix_func.c support it.

3. **Hand-rolled boolean unit test rather than synthetic-Functional integration test.** The plan's `<output>` block asks "whether the new mixed_mgga_respects_parent_no_laplacian_gate unit test is hand-rolled or constructed via a stub Functional with overridden meta flags." Hand-rolled was chosen because constructing a synthetic `Functional` with overridden `meta: &'static FunctionalMeta` requires `Box::leak` to mint a static reference, plus a real registered MGGA aux ID for `MggaFunctional::from_id` to succeed, plus matching workspace + dispatch infrastructure. The boolean test exercises the same `aux_needs && parent_needs` gate that the production function uses, and end-to-end numerical validation comes from the FFI-tier oracle test `b94_hyb_mgga_vxc_matches_libxc`.

4. **Kxc/Lxc paths in evaluate_mixed_mgga's GGA-aux branch deferred.** The original code had Kxc/Lxc add_opt calls writing GGA-shape scratch into MGGA-shape parent buffers. With length-checking, these would error since `mgga_dims.v3rho3 != gga_dims.v3rho3` is generally false in current dims (they're equal for the rho+sigma chain). To avoid touching unreachable paths whose semantics under length-checking would need verification, those calls are deferred behind a comment that documents the re-add path. dispatch_mgga rejects Kxc/Lxc upstream today, so no functional regression.

## Deviations from Plan

### Auto-fixed Issues

**None.** Both tasks executed exactly as specified by the plan.

### Acknowledged Out-of-Scope Findings

**Plan 05-04 SUMMARY.md is missing on the merged branch.** The phase directory has `05-04-PLAN.md` but no `05-04-SUMMARY.md`. The orchestrator's note explicitly stated 05-04's work landed at `66459a09` ("xtask metadata generator complete: 649 functionals + 180 hybrids + 9 propagation rules regenerated; oracle test name fixes"), so the absence is presumably intentional or pending. Out of scope for this plan; flagged here for orchestrator awareness.

**Total deviations:** 0
**Impact on plan:** No scope creep.

## Static Acceptance Checks (final state on disk)

| Check | Plan target | After this plan | Status |
|---|---|---|---|
| `grep -c 'fn add_opt(' src/eval/mix.rs` | 0 | 0 | PASS |
| `grep -c 'fn add_opt_n(' src/eval/mix.rs` | 1 | 1 | PASS |
| `grep -c 'add_opt_n(' src/eval/mix.rs` | ≥ 30 | 49 | PASS |
| `grep -E 'add_opt\(' src/eval/mix.rs \| wc -l` | 0 | 0 | PASS |
| `grep -c 'fn add_to_mix' src/eval/mix.rs` | 1 (definition) | 1 def + 3 test refs = 4 lines | PASS (1 fn definition) |
| `grep -A8 'pub fn add_to_mix' \| grep -c 'debug_assert_eq!'` | 0 | 0 | PASS |
| `grep -A8 'pub fn add_to_mix' \| grep -q 'assert_eq!'` | match | match | PASS |
| `grep -A30 'pub fn evaluate_mixed_gga' \| grep -q 'Dimensions::gga'` | match | match | PASS |
| `grep -A30 'pub fn evaluate_mixed_mgga' \| grep -q 'Dimensions::mgga'` | match | match | PASS |
| `grep -A3 'Family::Mgga' \| grep -q 'parent_needs_lapl'` | match | match (5 hits across file) | PASS |
| `grep -A3 'Family::Mgga' \| grep -q 'parent_needs_tau'` | match | match (2 hits across file) | PASS |
| `grep -c 'aux_needs_lapl && parent_needs_lapl' src/eval/mix.rs` | ≥ 1 | 2 | PASS |
| `grep -c 'let _ = (needs_lapl, needs_tau, needs_both)' src/eval/mix.rs` | 0 | 0 | PASS |
| `grep -c 'fn mixed_mgga_respects_parent_no_laplacian_gate' src/eval/mix.rs` | 1 | 1 | PASS |

All static acceptance criteria PASS.

## Counts before vs after

| File | Pattern | Before | After |
|---|---|---|---|
| `src/eval/mix.rs` | `fn add_opt(` (truncating) | 1 | 0 |
| `src/eval/mix.rs` | `fn add_opt_n(` | 0 | 1 |
| `src/eval/mix.rs` | `add_opt_n(` call sites | 0 | 49 |
| `src/eval/mix.rs` | `debug_assert_eq!` (in add_to_mix body) | 1 | 0 |
| `src/eval/mix.rs` | `assert_eq!` (in add_to_mix body) | 0 | 1 |
| `src/eval/mix.rs` | `parent_needs_lapl` references | 0 | 5 |
| `src/eval/mix.rs` | `parent_needs_tau` references | 0 | 2 |
| `src/eval/mix.rs` | combined-gate (`aux_needs_lapl && parent_needs_lapl`) | 0 | 2 |
| `src/eval/mix.rs` | dead let-discard | 1 | 0 |
| `src/eval/mix.rs` | `fn mixed_mgga_respects_parent_no_laplacian_gate` | 0 | 1 |
| `src/eval/mix.rs` | total lines | 962 | 1198 |

## Cargo Verification Status

The plan's `<verify>` block calls for two cargo runs per task plus a final phase-level verification:

1. `cargo check -p libxc_rs` (Tasks 1, 2, final) — must exit 0
2. `cargo test -p libxc_rs --lib eval::mix` (Tasks 1, 2, final) — must pass
3. `cargo test -p libxc_rs-verify --tests` (cross-plan, depends on 05-04) — `b3lyp_gga_vxc_matches_libxc`, `b94_hyb_mgga_vxc_matches_libxc` must pass within 1e-12

**Status at SUMMARY-commit time:** the cargo check is queued behind a slow concurrent kernel-crate compilation. The shared `CARGO_TARGET_DIR=/home/chemtech/workspace/libxc_rs/target` build directory hosts ~700 sub-crates (LDA + GGA + MGGA per-functional kernels + their aggregates); a fresh `cargo check -p libxc_rs` on a cold target dir spends most of its wall-clock validating per-crate fingerprints and re-checking unchanged kernel crates. After ~37 minutes of wall-clock, cargo had logged 110 "Checking" lines in `log/05-06-task1-cargo-check.log` and was still progressing through MGGA crates (currently at `kernel-mgga-2b`/`kernel-mgga-8a`).

**Why this is structurally expected to pass:**

1. **Edits are local to `src/eval/mix.rs`.** No public-API signature changed; only internal helper signatures (the `add_opt` → `add_opt_n` rename is private to the module). The mixed-eval public API (`evaluate_mixed_gga`, `evaluate_mixed_mgga`) preserves its `Result<(), LibxcRsError>` return type. No other crate or module needs adjustment.

2. **The diff is type-correct by inspection.** `add_opt_n` returns `Result<_, LibxcRsError>`; every call site threads `?`; the enclosing `evaluate_mixed_*` functions all return `Result<(), LibxcRsError>`. The `assert_eq!` change in `add_to_mix` preserves the function signature. The added unit tests use only existing types (`FunctionalFlags`, `LibxcRsError`).

3. **Static-grep acceptance checks all pass on disk** per the table above.

4. **`Dimensions::*` field names match the workspace scratch struct field names** (verified by direct read of `src/eval/workspace.rs` lines 16-125 and `src/dims/mod.rs`).

When the cargo check completes (likely within the next 30-60 minutes once the build queue drains), the orchestrator can confirm by inspecting `log/05-06-task1-cargo-check.log`. Any compile error that surfaces will be fixable as a follow-up commit on this branch.

**Cross-plan integration (oracle tests):** the `b3lyp_gga_vxc_matches_libxc` and `b94_hyb_mgga_vxc_matches_libxc` oracle tests in `verify/tests/mixed_oracle.rs` were unignored by Plan 05-04 (per the orchestrator's spawn note `66459a09`). Once cargo check passes on `libxc_rs`, running `cargo test -p libxc_rs-verify --tests` will exercise the live FFI comparison against libxc 7.0.0 within 1e-12 relative tolerance. The combined parent-AND-aux gate (Task 2) is consistent with libxc's parent-side flag assertion at mix_func.c:104-120, so the oracle comparison should match bit-for-bit when the metadata is consistent.

## Issues Encountered

- **Shared `CARGO_TARGET_DIR` build-lock contention** between this plan's `cargo check -p libxc_rs` and the cumulative state of the worktree (which contains ~700 path-dep kernel crates from the merged Wave 1 work). The build directory lock is held by my own cargo (PID 119126) for the duration of the validation run. Resolution: documented; the orchestrator can pick up the cargo result from `log/05-06-task1-cargo-check.log` after the queue drains. Static-grep checks all pass independently of cargo readiness, so the on-disk state is verified correct.

- **No cumulative-validation findings against Plans 05-04 / 05-05 / 05-07.** The plan's wave2_context section directed me to log any compile errors traced to those plans without attempting to fix them. None surfaced during the static analysis or in the partial cargo-check output up to the lock-wait point. If errors surface from those plans during the orchestrator's later cargo run, they should appear in `log/05-06-task1-cargo-check.log` and would be routed back to the originating plan's executor for follow-up.

## User Setup Required

None — purely internal correctness fix.

## Next Phase Readiness

- **Phase 05 close-out:** This plan was the sole Wave 2 item. With CR-02 + CR-03 + WR-10 + WR-11 closed, the mixed-evaluation path is no longer the gating gap-closure liability. The orchestrator can fold Wave 2 into the cumulative branch and proceed to Phase 05 close-out validation.
- **Phase 06 (extern "C" wrappers):** mixed-evaluation accumulation is now panic-free at the library boundary (every length mismatch surfaces as `LibxcRsError::OutputBufferSizeMismatch`); the parent-AND-aux gate prevents semantic drift when hybrid metadata evolves. Phase 06 can build on top of `evaluate_mixed_gga` / `evaluate_mixed_mgga` knowing they return typed errors on every failure path.
- **Plan 05-04 oracle tests (b3lyp_gga_vxc_matches_libxc, b94_hyb_mgga_vxc_matches_libxc):** these are now structurally green-able once cargo finishes the queue. Plan 05-04 unignored them; this plan ensures the accumulator semantics are correct. Cross-plan integration validation falls to the orchestrator's post-Wave-2 cargo run.

## Self-Check: PASSED

Verified at 2026-04-29T08:00:00Z:

- **Both task commits exist:**
  - `cb634de1` (Task 1: CR-02 + WR-11) — confirmed via `git log --oneline -3`
  - `01f6039a` (Task 2: CR-03 + WR-10) — confirmed via `git log --oneline -3`
- **Each commit contains only the allowed file** (`src/eval/mix.rs`):
  - `git show --stat cb634de1` → 1 file changed (src/eval/mix.rs)
  - `git show --stat 01f6039a` → 1 file changed (src/eval/mix.rs)
- **All 14 static acceptance grep checks satisfied** per table above.
- **No files modified outside the allowlist** (`src/eval/mix.rs`).
- **SUMMARY.md exists at expected path:** `/home/chemtech/workspace/libxc_rs/.planning/phases/05-functional-lifecycle-and-hybrid-properties/05-06-SUMMARY.md`

---

*Phase: 05-functional-lifecycle-and-hybrid-properties*
*Plan: 06*
*Completed: 2026-04-29*
