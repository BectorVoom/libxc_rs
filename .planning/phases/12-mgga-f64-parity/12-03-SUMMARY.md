---
phase: 12-mgga-f64-parity
plan: 03
subsystem: testing
tags: [mgga, mgga_x_th, parity, root-cause, regularization, translator]

# Dependency graph
requires:
  - phase: 12-mgga-f64-parity
    provides: "12-01 D-01 sigma-down regularization; 12-02 mgga_x_th canary skeleton"
provides:
  - "Root-cause finding: mgga_x_th's ~20% error was the wrong-variable regularization (tau-up vs sigma-down), NOT a per-functional translation bug — closed by D-01"
  - "Confirmation that no translator change / kernel regen is needed for mgga_x_th"
affects: [12-04]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Empirical D-01-closure check BEFORE assuming a translation bug (D-09 decoupling resolved in favor of regularization root cause)"

key-files:
  created: []
  modified: []

key-decisions:
  - "mgga_x_th root cause = regularization (sigma-down), not translation — proven empirically; Task 2 (translator fix + regen) is a documented no-op"
  - "No file under crates/kernels/mgga/mgga_x_th/ or tools/translate_* changed (AP-3 honored; D-01 was the fix)"

patterns-established:
  - "A functional that reads sigma AND tau independently (mgga_x_th: t19=1/tau, t34 uses sigma*t23*t19) is sensitive to the sigma-down-vs-tau-up triple difference — the D-01/D-02 mechanism explains the entire residual"

requirements-completed: [SC-1]

# Metrics
duration: ~5 min
completed: 2026-05-25
---

# Phase 12 Plan 03: mgga_x_th Root Cause Summary

**mgga_x_th's ~20% error was the same wrong-variable regularization (τ-up instead of libxc's σ-down), NOT a per-functional translation bug — D-01 closed it; the canary passes at 1e-12 (6.6e-16) and no translator change or kernel regen was required.**

## Performance

- **Duration:** ~5 min (fast confirmation — Tasks 2 no-op, Task 3 deferred to USER-RUN 12-04)
- **Completed:** 2026-05-25
- **Tasks:** 3 (Task 1 confirmation; Task 2 no-op; Task 3 family-oracle deferred to 12-04)
- **Files modified:** 0 (no code change — D-01 in 12-01 was the fix)

## Accomplishments
- **Task 1 — D-01 closure confirmed empirically.** The mgga_x_th single-kernel canary (built in 12-02, applying the D-01 σ-down regularization) passes at **6.616e-16** vs the libxc id=225 oracle (`/tmp/12-03-th-baseline.log`). Per the plan's Task-1 STEP-1 branch, this means D-01 closed mgga_x_th → skip the translator-fix task.
- **Root cause documented.** mgga_x_th reads σ and τ **independently** — `t19 = 1.0/tau[ip]`, and `t34` uses `sigma[ip]*t23*t19`. So it is sensitive to *which* variable the work-driver clamps. Under the old τ-up clamp it received a different (ρ,σ,τ) triple than libxc evaluates internally (which clamps σ DOWN), producing the ~20% divergence. Under D-01's σ-down regularization it receives the identical triple → machine-precision parity. The exc translation itself is byte-faithful (no constant-wrap / coefficient / piecewise / op-order / missing-term fault).
- **Task 2 — no-op.** No translator change in `tools/translate_*`; no regen of `crates/kernels/mgga/mgga_x_th/`. AP-3 honored (kernel and translator git status clean).
- **Task 3 — deferred to USER-RUN.** The authoritative family-oracle confirmation is the heavy build deferred to 12-04 (user checkpoint decision in 12-02). Corrected memory-safe command recorded below.

## Task Commits

No source commits — this plan made no code changes (D-01 in 12-01 was the fix). Plan metadata only:

- **Plan metadata** — this SUMMARY commit.

## Files Created/Modified
None. (Confirmation-only plan.)

## Decisions Made
- **mgga_x_th was NOT a translation bug** (D-09 had decoupled it as a *possible* per-functional defect). The empirical D-01-closure check resolved it in favor of the regularization root cause — saving the translator-fix + regen + idempotency work that Tasks 2 would otherwise entail.

## Deviations from Plan

None — plan executed exactly as written. Task 1's STEP-1 branch ("if PASS → D-01 closed it → skip Task 2, no translator change") was taken; Task 3's family-oracle build is deferred to the USER-RUN 12-04 gate per the 12-02 checkpoint decision (consistent with this RAM-constrained box running heavy oracle builds user-side).

## Issues Encountered
None.

## User Setup Required
None.

## Next Phase Readiness
- All 6 MGGA targets (5 cluster + mgga_x_th) pass single-kernel parity at 1e-12. No outstanding per-functional fixes.
- **12-04 (authoritative gate, USER-RUN):** the family oracle is the remaining authoritative confirmation. Memory-safe command:
  `cargo test -p libxc_rs-verify --no-default-features -F oracle-mgga --test mgga_oracle -j1 -- --test-threads=1 --nocapture`
  Expectation: all 6 targets pass exc 1e-12; no regression in previously-passing MGGA functionals (D-06); mgga_x_th confirmed in the real dispatch path (not just the canary).

---
*Phase: 12-mgga-f64-parity*
*Completed: 2026-05-25*
