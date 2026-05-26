---
phase: 10-workspace-level-modular-split
plan: 00
subsystem: infra
tags: [cargo, workspace, baseline, oracle, refactor-anchor]

requires:
  - phase: 11-splitter-v2-unified-5k-cap
    provides: the 306-crate kernel topology + cubecl-0.10 umbrella green
  - phase: 12-mgga-f64-parity
    provides: the σ-down regularization that makes the 6 routed MGGA exc functionals pass the f64 oracle
provides:
  - "Pre-refactor public-surface snapshot (SC-5 reference): log/10-surface-before.log"
  - "Monolithic dep-closure snapshot: log/10-tree-libxc_rs-before.log"
  - "Byte-snapshot of the 6 xtask-generated files + SHA256SUMS (D-03 idempotency reference): log/10-generated-snapshot/"
  - "Per-family oracle pass/fail baseline (SC-6/SC-7 reference): log/10-baseline-parity-{lda,gga,mgga}.log — ALL PASS"
  - "Phase-start green confirmation (kernel-free umbrella check EXIT 0)"
affects: [10-01-libxc-core, 10-02-libxc-eval, 10-03-libxc-compat-facade]

tech-stack:
  added: []
  patterns:
    - "Kernel-free + per-family compile gates for a kernel-invariant module-move refactor (full `cargo check -p libxc_rs --lib` OOMs on cold-cache borderline kernels)"

key-files:
  created:
    - log/10-surface-before.log
    - log/10-tree-libxc_rs-before.log
    - log/10-generated-snapshot/ (6 .rs + SHA256SUMS.txt)
    - log/10-baseline-parity-lda.log
    - log/10-baseline-parity-gga.log
    - log/10-baseline-parity-mgga.log
  modified: []

key-decisions:
  - "Green gate uses the kernel-free umbrella check (`cargo check -p libxc_rs --no-default-features --lib`, EXIT 0 0.31s) instead of the full `cargo check -p libxc_rs --lib` — the full check OOM-killed borderline-unsharded mgga_c_kcis on cold-cache re-expansion. User-confirmed: Phase 10 is a kernel-invariant module-move refactor, so kernel-free + per-family gates are correct; full-tree green-ness is anchored by 11-14/12-04. No kernel sharding (kcis compiles standalone in 9m41s)."
  - "SC-6 baseline is LDA pass / GGA pass / MGGA pass at 1e-12 (zero failures) — NOT the plan's anticipated 6-MGGA-fail set. Phase 12 (completed after this plan was written) fixed those 6. The recorded ACTUAL set is the SC-6 reference per the plan's escape hatch."

patterns-established:
  - "Refactor baseline anchoring: snapshot public surface + generated-file bytes + per-family oracle pass/fail BEFORE any move, so SC-5/SC-6 are diffable."

requirements-completed: []

duration: ~30min (assistant) + USER-RUN oracle (LDA 288s / GGA 869s / MGGA <1s warm)
completed: 2026-05-26
---

# Phase 10 / Plan 00: Pre-Refactor Baseline Snapshot Summary

**Captured the SC-5 surface, D-03 generated-file byte, and SC-6/SC-7 per-family oracle baselines for the workspace split; established kernel-free + per-family gates after the full-umbrella green gate OOM'd on a cold-cache borderline kernel.**

## Performance

- **Duration:** ~30 min assistant work + USER-RUN oracle sweeps (LDA 288s, GGA 869s, MGGA <1s warm-cached)
- **Completed:** 2026-05-26
- **Tasks:** 2 (Task 1 auto; Task 2 USER-RUN human-action checkpoint)
- **Files modified:** 0 source files (snapshot-only plan); 12 log artifacts created

## Accomplishments
- Public-surface snapshot: `log/10-surface-before.log` (28 `use libxc_rs::` paths) — the SC-5 reference the facade is diffed against in 10-03
- Generated-file byte-snapshot: `log/10-generated-snapshot/` (6 xtask outputs + SHA256SUMS) — the D-03 idempotency reference for 10-01
- Monolithic dep-closure snapshot: `log/10-tree-libxc_rs-before.log` (1647 lines)
- Per-family oracle baseline (SC-6/SC-7 reference): **LDA 2/2 pass, GGA 2/2 pass, MGGA 2/2 pass** at 1e-12 — zero failures
- Phase-start green confirmed via the kernel-free umbrella check (EXIT 0)

## Task Commits

1. **Task 1: Capture surface/dep-closure/generated-file/green-gate baselines** — `1bdc8e0f50` (chore)
2. **Task 2: USER-RUN per-family oracle baseline** — logs committed with this SUMMARY (docs)

## Files Created/Modified
- `log/10-surface-before.log` — pre-refactor public surface (SC-5 ref)
- `log/10-tree-libxc_rs-before.log` — monolithic dep closure
- `log/10-generated-snapshot/` — byte-snapshot of the 6 xtask outputs + SHA256SUMS (D-03 ref)
- `log/10-check-libxc_rs-baseline.log` — full-umbrella check (OOM'd on kcis — documents the gate decision)
- `log/10-check-libxc_rs-nodefault.log` — kernel-free green gate (EXIT 0)
- `log/10-kcis-standalone-check.log` — diagnostic: kcis compiles standalone (no sharding needed)
- `log/10-baseline-parity-{lda,gga,mgga}.log` — per-family oracle baselines (all pass)

## Decisions Made
- **Gate strategy (user-confirmed):** kernel-free `cargo check -p libxc_rs --no-default-features --lib` + per-family checks replace the full `cargo check -p libxc_rs --lib` throughout Phase 10. The full check OOM-killed the borderline-unsharded `mgga_c_kcis` (~60 parts) on cold-cache re-expansion under dep-graph memory pressure; kcis compiles fine standalone (9m41s, no OOM) so **no sharding is needed**. Kernel source is invariant under this module-move refactor; the full-tree green-ness + oracle parity are anchored by 11-14/12-04. See memory `reference_kernelfree_check_gate`.
- **SC-6 baseline correction:** the plan (written pre-Phase-12) anticipated 6 MGGA failures. Phase 12 fixed those 6, so the ACTUAL recorded baseline is all-families-pass. That recorded set is the SC-6 reference (plan escape hatch honored).

## Deviations from Plan

### 1. [Gate substitution] Full-umbrella green gate → kernel-free green gate
- **Found during:** Task 1 (green gate)
- **Issue:** `cargo check -p libxc_rs --lib` (the plan's literal gate) OOM-SIGKILLed `mgga_c_kcis` at dep #253/306 — cold-cache re-expansion of a borderline functional under the full dep-graph's memory overhead.
- **Fix:** Surfaced to user (plan says STOP on green-gate failure); user chose kernel-free + per-family gates. Confirmed kcis compiles standalone (no defect, no sharding). Kernel-free gate EXIT 0 (0.31s).
- **Verification:** `log/10-check-libxc_rs-nodefault.log` EXIT 0; `log/10-kcis-standalone-check.log` Finished 9m41s.
- **Impact:** No scope creep; covers exactly what the refactor changes. Full-tree green anchored by 11-14/12-04.

### 2. [Stale expectation] MGGA baseline is all-pass, not 6-fail
- **Found during:** Task 2 (oracle baseline)
- **Issue:** Plan expected 6 MGGA fails; Phase 12 (completed after this plan was written) fixed them.
- **Fix:** Recorded the actual all-pass set as the SC-6 reference per the plan's escape hatch.
- **Impact:** Cleaner baseline; 10-03's SC-6 diff target is "all pass, zero failures."

---

**Total deviations:** 2 (1 gate substitution, 1 stale-expectation correction). Both necessary; no scope creep; no source edits.

## Issues Encountered
- Borderline-kernel OOM during the full-umbrella check (kcis) — resolved by switching to kernel-free + per-family gates and confirming kcis builds standalone. Cache is now warm (LDA/MGGA fully cached, 25 small GGA rebuilt during the oracle), so subsequent per-family builds are smooth.

## User Setup Required
None.

## Next Phase Readiness
- All SC-5/SC-6/SC-7 baselines captured; 10-01 (extract libxc-core) is unblocked.
- Gate strategy fixed for the phase: kernel-free + per-family `cargo check`; `cargo tree` for boundary SCs; USER-RUN per-family oracle for the final parity gate (10-03 Task 3).

---
*Phase: 10-workspace-level-modular-split*
*Completed: 2026-05-26*
