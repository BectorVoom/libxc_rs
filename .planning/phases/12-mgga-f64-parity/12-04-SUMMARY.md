---
phase: 12-mgga-f64-parity
plan: 04
subsystem: eval
tags: [mgga, oracle, regression, needs-tau, regularization, idempotency, parity, d-13]

# Dependency graph
requires:
  - phase: 12-mgga-f64-parity
    provides: "12-01 D-01 sigma-down regularization; 12-02 6 canaries; 12-03 mgga_x_th confirmation"
provides:
  - "Authoritative per-family f64 oracle verdicts: MGGA failures=0 (SC-1+SC-2 MGGA), LDA failures=0 (SC-2 LDA)"
  - "NEEDS_TAU gating fix: tau-floor + sigma-down clamp now gated on XC_FLAGS_NEEDS_TAU (matches libxc work_mgga_inc.c:62)"
  - "mgga_k_gea2 non-NEEDS_TAU regression canary (permanent guard)"
  - "D-13 resolution: mgga_x_2d_js17 closed by D-01, remains routed (CASE A)"
affects: []

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Input regularization gated on functional flags (NEEDS_TAU) at the dispatch chokepoint via functional.to_id().meta().flags"
    - "The family oracle (D-06) is the authoritative regression gate — it caught a regression the single-kernel canaries (per-functional) could not"

key-files:
  created:
    - "verify-canary/tests/mgga_k_gea2_parity.rs (non-NEEDS_TAU regression guard)"
  modified:
    - "src/eval/mgga_dispatch/prepare.rs (regularize_inputs gains needs_tau gate)"
    - "src/eval/mgga_dispatch/mod.rs (chokepoint computes needs_tau from functional flags)"
    - "verify-canary/Cargo.toml (mgga_k_gea2 dep)"

key-decisions:
  - "Full-tree regen (Task 1) NOT run: Phase 12 changed zero kernel/translator files (driver-only fix), so there is no codegen drift to catch; a literal `translate --family all` would un-shard rmggac/kcisk (the separate Phase-11 `split` step) and reintroduce the OOM ceiling. D-LOCK-D idempotency was proven in Phase 11 (11-11 G-4) and is unperturbed."
  - "D-13 CASE A: mgga_x_2d_js17 (XC_FLAGS_2D + NEEDS_TAU) is closed by D-01 and passes the family oracle — remains routed, no de-route."
  - "NEEDS_TAU gating: the tau-floor + sigma-down clamp apply only when the functional declares XC_FLAGS_NEEDS_TAU (libxc work_mgga_inc.c:62); the sigma/rho floors stay unconditional."
  - "Corrected oracle command: -p libxc_rs-verify --no-default-features -F oracle-<fam> (the plan's -p libxc_rs --features oracle-<fam> is wrong: test lives in verify, and default-features pulls all 306 -> OOM)."

patterns-established:
  - "A shared input-regularization change MUST be gated on the per-functional flags libxc gates on — applying it uniformly regresses functionals that don't share the flag"

requirements-completed: [SC-1, SC-2]

# Metrics
duration: ~50 min (incl. regression diagnosis + fix + USER-RUN oracle gate)
completed: 2026-05-25
---

# Phase 12 Plan 04: Final Per-Family Oracle Regression Gate Summary

**The authoritative per-family f64 oracle gate passes — MGGA failures=0 (all 6 targets + the 4 functionals a D-01 regression had broken), LDA failures=0 — after fixing a regression the gate caught: the σ-down clamp must be gated on `XC_FLAGS_NEEDS_TAU` exactly like libxc, not applied to every routed MGGA functional.**

## Performance

- **Duration:** ~50 min (incl. regression diagnosis, fix, and the USER-RUN oracle re-run)
- **Completed:** 2026-05-25
- **Tasks:** 3 (Task 1 deviation; Task 2 CASE A; Task 3 blocking human-verify — failed, fixed, re-passed)
- **Files modified:** 4

## Accomplishments

### SC-1 + SC-2 evidence (USER-RUN, jobs=1, memory-safe family-chunked)
```
MGGA unpol summary: tested=12 skipped_no_exc=1 skipped_not_compiled=117 skipped_pending_params=10 skipped_deferred=6 failures=0
  test result: ok. 2 passed; 0 failed     (test_all_mgga_oracle_pol + test_all_mgga_oracle_unpol)
LDA  unpol summary: tested=38 skipped_no_exc=0 skipped_deferred=4 skipped_not_compiled=26 failures=0
  test result: ok. 2 passed; 0 failed
```
- **SC-1:** all 6 target MGGA functionals (mgga_x_th, mgga_x_2d_js17, mgga_c_cs, mgga_x_pkzb, mgga_x_pbe_gx, mgga_x_tm) pass the family oracle exc at 1e-12.
- **SC-2 (MGGA):** zero MGGA failures — the 4 functionals a D-01 regression had broken (mgga_k_gea2/gea4, mgga_xc_zlp/lp90) now pass; no other-MGGA regression; vxc-unpol within TOL_VXC=1e-10.
- **SC-2 (LDA):** zero LDA failures — no cross-family regression.
- **SC-2 (GGA):** not re-run, **unaffected by construction**: the only Phase-12 source change (`src/eval/mgga_dispatch/`) is `#[cfg(feature="oracle-mgga")]`-gated, so the `oracle-gga` build does not compile it (uses the stub `dispatch_gga`). LDA empirically confirms the cfg-isolation. Command to confirm if desired:
  `cargo test -p libxc_rs-verify --no-default-features -F oracle-gga --test gga_oracle --jobs 1 -- --test-threads=1 --nocapture`

### The regression the gate caught (and the fix)
The first MGGA gate run failed with 4 failures (mgga_k_gea2 4.4e-3, mgga_k_gea4 4.5e-3, mgga_xc_zlp 3.0e-3, mgga_xc_lp90 4.3e-4) — **none of them Phase-12 targets**. Root cause: D-01's `regularize_inputs` applied the τ-floor + σ-DOWN clamp **unconditionally**, but libxc gates both inside `if(flags & XC_FLAGS_NEEDS_TAU)` (work_mgga_inc.c:62). These 4 carry `XC_FLAGS_NEEDS_LAPLACIAN`, **not** `NEEDS_TAU` — they read σ but never τ. The old τ-up clamp was invisible to them (they ignore τ); the σ-down clamp lowered σ they *do* read → divergence. **Regression introduced by D-01, caught by the D-06 family-oracle gate** (the single-kernel canaries could not catch it — they only cover targeted functionals).

Fix (commit `cd9b9691b4`): `regularize_inputs` gains a `needs_tau: bool`; ρ/σ floors stay unconditional, the τ-floor + σ-down clamp are gated on it. The chokepoint computes `needs_tau = functional.to_id().meta().flags.contains(FunctionalFlags::NEEDS_TAU)`. No-op for the 6 NEEDS_TAU targets; restores raw σ for the laplacian-only functionals.

### Task 1 — full-tree regen deviation
Not run (see Decisions): Phase 12 changed **zero** files under `crates/kernels/` and `tools/` (verified `git diff b5a1113eda..HEAD`), so there is no codegen drift to catch. A literal `translate --family all` would re-emit base crates and un-shard rmggac/kcisk (the separate Phase-11 `split` step) → reintroduce the OOM ceiling, for zero benefit. D-LOCK-D idempotency stands from Phase 11 (11-11 G-4: 264 zero-diff); the `--dry-run` confirmed the translator still runs clean (mgga ok=92, 0 failed).

### Task 2 — D-13 CASE A
mgga_x_2d_js17 (`XC_FLAGS_2D | XC_FLAGS_NEEDS_TAU`) is closed by D-01 (canary 0.0; family oracle pass), so it **remains routed** — no de-route needed. The 2D-dimensionality escape hatch is not exercised.

## Task Commits

1. **Regression fix (NEEDS_TAU gating) + mgga_k_gea2 canary** — `cd9b9691b4` (fix)
   - (Tasks 1 & 2 made no code changes — deviation/no-op, documented above.)

**Plan metadata:** this SUMMARY commit.

## Files Created/Modified
- `src/eval/mgga_dispatch/prepare.rs` — `regularize_inputs(..., needs_tau: bool)`; gates τ-floor + σ-down.
- `src/eval/mgga_dispatch/mod.rs` — chokepoint computes `needs_tau` from `functional.to_id().meta().flags`.
- `verify-canary/Cargo.toml` — `libxc-kernel-mgga_k_gea2` dep.
- `verify-canary/tests/mgga_k_gea2_parity.rs` — non-NEEDS_TAU regression guard (passes 6.3e-16).

## Decisions Made
- See key-decisions. The headline: a uniform input-regularization change must respect the per-functional flags libxc gates on; the family oracle (not per-target canaries) is what surfaced this.

## Deviations from Plan

### 1. [Rule 1 - Bug found during plan] D-01 σ-down clamp regressed non-NEEDS_TAU functionals
- **Found during:** Task 3 (family-oracle gate)
- **Issue:** σ-down clamp applied unconditionally; libxc gates it on NEEDS_TAU. 4 laplacian-only MGGA functionals regressed.
- **Fix:** `needs_tau` gate in `regularize_inputs` + chokepoint flag lookup; mgga_k_gea2 regression canary.
- **Verification:** gea2 canary 6.3e-16; MGGA family oracle failures=0 on re-run.
- **Committed in:** `cd9b9691b4`

### 2. [Rule 4 - Architectural judgment] Task 1 full-tree regen not run
- **Found during:** Task 1
- **Issue:** Plan mandates `translate --family all`, but it is destructive vs the sharded tree and unnecessary (zero Phase-12 codegen change).
- **Fix:** Proved non-perturbation via `git diff` (zero kernel/tools changes) + `--dry-run`; relied on Phase-11's established D-LOCK-D idempotency. Surfaced to the user at the checkpoint.
- **Verification:** `git diff b5a1113eda..HEAD -- crates/kernels/ tools/` empty.

**Total deviations:** 2 (1 Rule-1 regression fix, 1 Rule-4 judgment). **Impact:** The Rule-1 fix was essential (SC-2 correctness). The Rule-4 deviation avoided a destructive, OOM-reintroducing no-op. No scope creep.

## Issues Encountered
- The first oracle run failed (4 non-target regressions). Diagnosed to the NEEDS_TAU gate, fixed, re-run clean. This is exactly why the D-06 all-family gate exists — the per-functional canaries (passing) would have declared victory prematurely (the b94-hollow-gate lesson, generalized).

## User Setup Required
None.

## Next Phase Readiness
- **Phase 12 goal achieved:** all 6 target MGGA functionals reach f64 oracle parity at 1e-12, with no LDA/MGGA regression and GGA unaffected by construction.
- Optional: run the GGA oracle for a fully-explicit SC-2 triple (command above) — expected `ok`, unchanged.
- The MGGA dispatch input regularization now mirrors libxc work_mgga_inc.c exactly (ρ/σ floors unconditional; τ-floor + σ-down gated on NEEDS_TAU).

---
*Phase: 12-mgga-f64-parity*
*Completed: 2026-05-25*
