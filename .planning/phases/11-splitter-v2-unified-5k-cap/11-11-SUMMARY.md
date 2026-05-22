---
phase: 11-splitter-v2-unified-5k-cap
plan: 11
subsystem: testing
tags: [g4, d-lock-d, idempotency, translator, sharded, tpssloc, revtpss, hier-cse]

requires:
  - phase: 11.1-translator-rule-3-emit-fix-sweep-to-green
    provides: "deferred D-LOCK-D idempotency proof + the tpssloc/revtpss hier-CSE+shard recipe (260520-eem/k1q)"
provides:
  - "D-LOCK-D idempotency proof: 264 non-sharded functionals byte-stable under re-translate (zero diff)"
  - "Sharded pair (tpssloc/revtpss) dispositioned Option A: deterministic split post-process, OOM-fix layout preserved"
affects: [11-13, translator, codegen-determinism]

tech-stack:
  added: []
  patterns:
    - "Idempotency proof via direct translate + git diff (incl. untracked-file cross-checks), not the stale test_idempotency.sh"
    - "Sharded functionals proven via split-tool selftest (double-run byte-identical) + idempotency-guard no-op, not whole-tree flat diff"

key-files:
  created:
    - .planning/phases/11-splitter-v2-unified-5k-cap/11-11-IDEMPOTENCY-PROOF.md
  modified: []

key-decisions:
  - "Sharded-pair disposition = Option A (exclude-and-document deterministic post-process), not Option B full pipeline replay — avoids regenerating the two heaviest functionals on a RAM-constrained box"

patterns-established:
  - "Translator codegen is deterministic; the only non-vanilla layout (tpssloc/revtpss shards) is a deterministic, idempotent post-process"

requirements-completed: []

duration: ~15min
completed: 2026-05-22
---

# Phase 11 · Plan 11 Summary

**D-LOCK-D SATISFIED: a fresh `translate --family all` reproduces the 264 non-sharded functionals byte-for-byte (zero diff incl. untracked cross-checks); the 2 sharded functionals are a deterministic split post-process (selftest double-run byte-identical + committed-layout idempotency-guard no-op).**

## Performance
- **Duration:** ~15 min (translator-only; no cargo)
- **Completed:** 2026-05-22
- **Tasks:** 2 (both produce 11-11-IDEMPOTENCY-PROOF.md)
- **Files modified:** 1 created (proof doc); no source changes

## Accomplishments
- **G-4 closed.** Vanilla `translate --family all` (rc=0; lda 43 + gga 131 + mgga 92 = 266 emitted, 0 failed) produced **zero diff** for the 264 non-sharded functionals — no tracked-modified files AND no new untracked files outside the sharded pair. No codegen non-determinism found.
- **Sharded pair handled (Option A).** mgga_c_tpssloc + mgga_c_revtpss are excluded-and-documented as a deterministic two-stage post-process (hier-CSE regen + `split_per_functional_subcrate.py --budget 10000`). Demonstrated split-stage determinism: tool `--selftest` PASS (*"double-run byte-identical"*) + the `_already_split` idempotency guard no-ops on the actual committed layout. The OOM-fix facade + 7 `_pK` shard layout was preserved verbatim (the transient flat re-emit was discarded by `git checkout`/`git clean`).

## Task Commits
1. **Task 1 + Task 2: D-LOCK-D proof** — `8a2640a98c` (docs) — 11-11-IDEMPOTENCY-PROOF.md
2. **SUMMARY** — `0206ace97a` (docs)

## Files Created/Modified
- `.planning/phases/11-splitter-v2-unified-5k-cap/11-11-IDEMPOTENCY-PROOF.md` (new) — the SATISFIED proof.

## Decisions Made
- **Option A over Option B** for the sharded pair: the split tool's built-in selftest (double-run byte-identical) + idempotency guard + the proven 2/2 recipe replays (260520-eem/k1q) are sufficient evidence of determinism without regenerating tpssloc/revtpss (the two heaviest functionals) on a RAM-constrained machine.
- Used direct `translate` + `git diff` (not the stale `test_idempotency.sh`, which references the pre-D-10a `crates/kernels/lda/src` path deleted in 11-03).

## Deviations from Plan
None — plan executed as written (Option A was the plan's RECOMMENDED disposition).

## Issues Encountered
- `git diff --stat` alone would have hidden the 235 untracked files the flat re-emit creates; added explicit untracked-file cross-checks (`git status --porcelain | grep '^??'`) to confirm the 264 are truly zero-churn, not just zero-modified.

## Next Phase Readiness
- **G-4 (11-11): DONE & verified.** D-LOCK-D evidence is ready for 11-13 (G-5 closure).
- Independent of the cubecl-0.10 umbrella launch-ABI blocker (memory `project_umbrella_cubecl010_launch_abi_drift`) — this plan touched no umbrella code.

---
*Phase: 11-splitter-v2-unified-5k-cap*
*Completed: 2026-05-22*
