---
phase: 11-splitter-v2-unified-5k-cap
plan: 12
subsystem: testing
tags: [g2, oracle, f64, family-chunked, feature-gate, memory-safe, cargo-features, cfg-gating]

requires:
  - phase: 11 (11-09, G-1)
    provides: production von Weizsäcker τ-clamp in mgga_dispatch/prepare.rs (MGGA oracle inherits it)
  - phase: 11 (11-14, G-6)
    provides: umbrella libxc_rs compiles under cubecl 0.10 (the oracle routes through eval::dispatch_*)
provides:
  - Memory-safe family-chunked oracle path — per-family Cargo features oracle-{lda,gga,mgga} make a
    single-family build pull only that family's kernels (no OOMing all-281 umbrella build)
  - Umbrella source cfg-gating (kernel/dispatch modules per family + stub dispatch_* fns)
  - launch.rs buffer helpers generic over the element type (f32-ready foundation)
  - Repaired Phase-05 oracle harnesses ({lda,gga,mgga}_oracle.rs run again, family-gated)
  - A working f64 oracle that surfaced 6 real MGGA parity bugs (attributed, routed to roadmap)
affects: [11-13, G-5 closure, MGGA-parity follow-up]

tech-stack:
  added: []
  patterns:
    - "Per-family Cargo feature gating (optional deps + oracle-<fam> features, default=all) for memory-safe chunked builds"
    - "Stub dispatch_<fam> under #[cfg(not(feature))] keeps the public symbol defined so callers need no cfg cascade"

key-files:
  created:
    - .planning/phases/11-splitter-v2-unified-5k-cap/11-12-ORACLE-F32-LOG.md
    - tools/make_kernel_deps_optional.py
  modified:
    - Cargo.toml
    - verify/Cargo.toml
    - src/kernel/mod.rs
    - src/eval/mod.rs
    - src/kernel/launch.rs
    - src/functional/evaluate.rs
    - src/eval/mix.rs
    - verify/tests/lda_oracle.rs
    - verify/tests/gga_oracle.rs
    - verify/tests/mgga_oracle.rs

key-decisions:
  - "Path A (feature-gate the umbrella by family) over Path B (per-family verify sub-crate) — clean 3-file family boundary made it feasible"
  - "f32/G4 RE-DEFERRED as milestone-scale: kernels are f64-concrete by design (2491 files, 0 generic); real f32 needs a translator re-arch + full regen, against the f64-only/1e-12 core value"
  - "G-2 re-scoped to the memory-safe family-chunked f64 oracle (the on-design deliverable)"
  - "6/12 MGGA exc parity failures ATTRIBUTED as residual MGGA f64 correctness gaps (not silently passed); routed to a dedicated roadmap effort, not fixed in G-2"

patterns-established:
  - "tools/make_kernel_deps_optional.py: scripted Cargo dep→optional + per-family feature emission"
  - "cargo tree -e no-dev as the cheap (no-compile) proof of per-family kernel isolation"

requirements-completed: []

duration: 2 sessions (2026-05-22 + 2026-05-23)
completed: 2026-05-23
---

# Phase 11 Plan 12: G-2 Family-chunked f64 Oracle Summary

**Memory-safe family-chunked f64 oracle via per-family Cargo features + umbrella cfg-gating: a single-family build pulls only that family's kernels (no all-281 OOM). LDA & GGA pass at f64 tiers; the now-runnable MGGA oracle surfaced 6/12 real exc parity bugs (attributed). f32/G4 re-deferred — kernels are f64-concrete by design.**

## Performance
- **Duration:** 2 sessions (2026-05-22 mechanism + 2026-05-23 source/validation)
- **Completed:** 2026-05-23
- **Tasks:** Re-scoped — original Task 1 (memory-safe path) delivered; Task 2/3 (f32 sweep + tolerance checkpoint) re-deferred (see Deviations)
- **Files modified:** 10 (+ 2 created)

## Accomplishments
- **Memory-safe family-chunked oracle mechanism** (G-2's core goal): root `Cargo.toml` makes all 280 per-functional kernel deps `optional` behind `oracle-{lda,gga,mgga}` features (`default = all`); `cargo tree` proves `--no-default-features -F oracle-<fam>` resolves ONLY that family's kernels (+ shared math). No OOMing all-281 umbrella build.
- **Umbrella source cfg-gating** so a single-family build compiles: `kernel/mod.rs` + `eval/mod.rs` gate family modules behind features, with `#[cfg(not(feature))]` stub `dispatch_*` keeping symbols defined. Verified clean via `cargo check -p libxc_rs --lib --no-default-features` (EXIT 0).
- **Repaired the stale Phase-05 oracle harnesses** (broken since the D-10a restructure): fixed the `is_deferred` import to `libxc_kernel_math::deferred::{lda,mgga}`, added per-family `#![cfg]`, fixed an `LdaInput::new(&rho)` owned-Vec drift.
- **`launch.rs` generic over the float type** (f32-ready foundation; f64 callers unchanged).
- **Ran the full f64 oracle per family** (user, jobs=1, memory-safe): **LDA ✓, GGA ✓** at f64 tiers; **MGGA 6/12 routed exc functionals fail** — surfaced for the first time by the now-runnable oracle.

## Per-family f64 oracle results (user-run 2026-05-23)
| Family | Result |
|--------|--------|
| LDA  | ✓ pass (no failures) |
| GGA  | ✓ pass (no failures) |
| MGGA | ✗ 6 of 12 routed exc functionals fail (test_all_mgga_oracle_unpol); pol test passed |

MGGA exc failures (rel_err vs C oracle, f64): `mgga_x_th` 2.0e-1 · `mgga_x_2d_js17` 1.1e-2 · `mgga_c_cs` 9.2e-3 · `mgga_x_pkzb` 3.7e-3 · `mgga_x_pbe_gx` 1.5e-3 · `mgga_x_tm` 9.2e-4.

**Attribution:** these are genuine pre-existing **MGGA f64 correctness gaps**, NOT a harness/f32/τ-clamp issue. The G-1 von Weizsäcker τ-clamp IS correctly applied (`mgga_dispatch/mod.rs:280-282`). `mgga_x_th` at 20% is almost certainly a per-functional translation bug; the smaller ones may be residual `work_mgga` regularization beyond the τ-clamp. They were always present; the now-runnable family-chunked oracle is the first thing to exercise them. **Routed to a dedicated MGGA-parity roadmap effort** (per-functional translation debug + work-driver regularization) — out of G-2's "build the oracle path" scope.

## Task Commits
1. **Path A mechanism (Cargo features)** — `f04e2095dc` (build)
2. **D1 umbrella source cfg-gating** — `af1c5e1c20` (feat)
3. **D2 harness repair** — `5566f99467` (fix) + `bf7c4b6eb3` (fix, LdaInput &rho)
4. **S1 launch.rs generic** — `86cf732e09` (feat)
5. **Re-scope/re-defer docs** — `9b8dea6302` (docs); cheap-check baseline `31aae07197`; STATE `9e0e4dfb46`

## Decisions Made
See key-decisions frontmatter. Headline: G-2 re-scoped from the f32 oracle to the **f64** oracle after discovering kernels are f64-concrete.

## Deviations from Plan

### [Rule 4 — Architectural] f32/G4 re-deferred; G-2 re-scoped to the f64 oracle
- **Found during:** Task 2 prep (wiring the f32 path). The plan + the 11.1-03 G4 deferral both assumed `LIBXC_RS_F32=1` produced f32 results via a dispatch/launch switch.
- **Issue:** f32 evaluation is unimplemented at the kernel layer — kernels are f64-concrete (2491 files `&Array<f64>`, 0 generic). `LIBXC_RS_F32`'s only reader (`parity_phase11.rs`) flips a tolerance but computes in f64 (placeholder). Wiring the f32 oracle as planned would falsely "pass" G4 by comparing f64-vs-f64 at a relaxed tolerance (threat T-11-12-01).
- **Fix:** STOPPED, presented to user → user chose to re-defer f32/G4 as milestone-scale (translator re-arch + full regen, against the f64-only/1e-12 design) and re-scope G-2 to the memory-safe family-chunked **f64** oracle. `launch.rs` generic kept as a harmless foundation.
- **Verification:** repo-wide grep (2491 `&Array<f64>`, 0 `&Array<F>`); recorded in `11-12-ORACLE-F32-LOG.md` + memory `project_kernels_f64_concrete_f32_milestone`.

**Total deviations:** 1 Rule-4 architectural (user-approved). **Impact:** G-2's memory-safe path (the hard part) is delivered; the f32 sweep was an over-specified, off-design aspiration now correctly routed to a milestone.

## Issues Encountered
- **MGGA 6/12 f64 parity failures** — real, attributed, routed to roadmap (see above). Not a blocker for G-2's re-scoped deliverable (oracle runs to completion; residual failures attributed, not silently passed).
- Phase-05 oracle harnesses didn't compile (deleted family-crate imports); repaired in D2.

## User Setup Required
None.

## Next Phase Readiness
- **G-2 (f64) delivered:** memory-safe family-chunked oracle built, validated, and run per family with attributed residuals.
- **For 11-13 (G-5 closure):** (1) correct ROADMAP SC-#5/G4 wording — "full-649 f32 oracle" is a milestone-scale follow-up, not a Phase-11 gate; (2) add a ROADMAP gap for **MGGA f64 parity** (6 functionals; per-functional translation + work_mgga regularization).
- f32/G4 and the MGGA-parity effort are the documented follow-ups.

---
*Phase: 11-splitter-v2-unified-5k-cap*
*Completed: 2026-05-23*
