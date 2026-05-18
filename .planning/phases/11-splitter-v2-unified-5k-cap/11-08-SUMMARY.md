---
phase: 11-splitter-v2-unified-5k-cap
plan: 08
subsystem: sweep-manifest-ingest, audits, partial-phase-close
tags: [partial, audit-finalization, sweep-manifest-ingest, translator-fix-blocking, D-31, D-32, D-33, D-34, AP-2-narrowed, AP-8-clarified]
status: PARTIAL — Task 1 PARTIAL (audits 3/5 PASS, sweep ingest documents Rule 3 emit gap, F32 smoke deferred). Tasks 2 + 3 deferred to phase 11.1 follow-up after translator fix.
completed: 2026-05-18
---

# Phase 11 Plan 08: Audits + phase close — PARTIAL

## Outcome

**PARTIAL — phase 11 cannot close cleanly until translator Rule 3 emit gap is fixed.**

Task 1 (audits + manifest ingest + F32 smoke + write 11-FINAL-METRICS.md) ran the structural audit re-runs successfully but cannot satisfy SPEC-11-R4 due to the Rule 3 translator defect surfaced in 11-06 Leg 2 and re-confirmed in 11-07's LDA sweep. F32 smoke (Step 3) and Tasks 2 + 3 (config/cleanup + full-649 f32 oracle sweep) are deferred to a phase 11.1 follow-up that lands the translator fix first.

## Tasks

| # | Task | Status | Outcome |
|---|---|---|---|
| 1 | Ingest 11-07 manifest + LIBXC_RS_F32 smoke + 5 audits + write 11-FINAL-METRICS.md | PARTIAL | Steps 1 (manifest existence), 4 (cube_launch audit), 5 (5 audits re-run), 6 (write metrics) ran. Step 3 (F32 smoke) deferred (translator bug would block cargo test). SPEC-11-R4 gate not satisfied (1 confirmed FAIL + GGA/MGGA unrun) — recorded as `verdict: TRANSLATOR_FIX_BLOCKING` in 11-FINAL-METRICS.md instead of HALT-and-discuss-phase. |
| 2 | CLAUDE.md + ROADMAP edits + tool cleanup + LIBXC_RS_BYPASS_DEFERRED removal | DEFERRED | All cleanup actions assume phase is closing cleanly. Premature until 11.1 lands. |
| 3 | Full-649 f32 oracle sweep (D-24 phase-end deliverable) | DEFERRED | Cargo test on oracle_*.rs harness pulls full dispatch tree; same Rule 3 transitive-compile failure as 11-06 Leg 2. |

## Audit Re-run Results (Task 1 Step 5)

| # | Tool | Exit | Status |
|---|---|---|---|
| A1 | `tools/audit_cube_launch.sh` | 0 | PASS — 1654 routed (functional, output) pairs, 0 unrouted, 22 math/src/ |
| A2 | `tools/audit_subcrate_collapse.sh` | 0 | PASS — 0 numbered subcrates, 0 family-level crates |
| A3 | `tools/audit_kernel_size.py --strict` | 1 | FAIL — 92 files >5K total; 22 unexcepted violations (max 6,674); see `project_splitter_algorithm_floor` memory disposition |
| A4 | `tools/audit_dispatch_tree.sh` | 0 | PASS — 0 unresolved batchN references |
| A5 | `tools/test_idempotency.sh` | 1 | FAIL — `split_lda_subcrates.py` tool-staleness (references pre-11-03 path `crates/kernels/lda/src`); needs post-D-10a update |

## Structural achievements (vs 11-BASELINE — PASS)

| Metric | Baseline | Now | Delta |
|---|---|---|---|
| Numbered subcrates | 27 | 0 | -27 PASS |
| Family-level crates | 3 | 0 | -3 PASS |
| Per-functional subcrates | 0 | 266 | +266 PASS (D-10a clean-slate per 11-03) |
| Max line (unexcepted) | 16,703 | 6,674 | -10,029 (target 5,000 not met but order-of-magnitude improvement) |
| Files >5K (unexcepted) | 235 | 22 | -213 (single-output splitter floor per memory) |
| D-13 launch budget | N/A | 1654/0/22 PASS | New invariant per D-13 |
| Dispatch tree references | unresolved (B1) | 0 unresolved | PASS |

## Codegen-correctness blockers (FAIL — requires phase 11.1)

| Block | Evidence | Defect class |
|---|---|---|
| SPEC-11-R4 | 11-07 LDA partial sweep: 12 OK + 1 FAIL (`lda_c_pk09`, 789 errors, P2 pattern) | Translator Rule 3 emit gap |
| SPEC-11-R5 | 11-06 Leg 2 attempt: HALT at `gga_c_gaploc` (2920 errors, P1 pattern) — recorded in `.continue-here.md` (`92ddcebe90`) | Translator Rule 3 emit gap |
| D-24 full-649 f32 sweep | NOT RUN — same transitive-compile failure expected | Translator Rule 3 emit gap |

## Why Plan 11-08 stays PARTIAL (not FAILED)

The audits portion (Task 1 Steps 4 + 5) ran cleanly and documented the structural end state, which is the durable contribution of phase 11. The codegen blockers all resolve to a single translator defect family (Rule 3 emit gap in chunk bodies) — that's the precise, well-scoped follow-up. Marking the plan FAILED would imply the work was wasted; marking it COMPLETE would imply codegen correctness is achieved. PARTIAL with explicit "translator-fix-blocking" disposition is the honest closure.

## Phase 11.1 Follow-Up Scope (recommended)

See `11-FINAL-METRICS.md` § "Phase 11.1 Follow-Up Scope (translator fix)" for the precise plan:

1. Amend `tools/translate_v2/` chunk-body emit path to apply Rule 3 to ALL f64-literal positions (named consts in arithmetic, bare tuple-return members, let-binding right-hand sides feeding F expressions).
2. Full-tree regen across 266 subcrates.
3. Re-run `python3 tools/batched_compile_sweep.py` end-to-end; iterate until VERDICT: ALL_OK.
4. Resume 11-06 Task 6 Legs 2/3/4 (canary parity at both precisions + idempotency) + Task 8 (rewrite PARTIAL summary).
5. Resume 11-08 Task 2 (CLAUDE.md/ROADMAP edits + tool cleanup + LIBXC_RS_BYPASS_DEFERRED removal) + Task 3 (full-649 f32 oracle sweep).
6. Then phase.complete 11.

## Commits

| SHA | Message |
|---|---|
| `6e2a793fb8` | feat(11-07): tools/batched_compile_sweep.py — pure orchestrator (D-31/D-32/D-33) |
| `6667a0731b` | docs(11-07): close plan 07 PARTIAL — sweep tool authored, LDA sweep halted at lda_c_pk09 |
| `92ddcebe90` | docs(11-06): record HALT at Task 6 Leg 2 — translator Rule 3 emission defect |
| `a470529c8c` | metrics(11-08): phase-end metrics — structural goals met, codegen blocked on translator fix |
| (this commit) | docs(11-08): close plan 08 PARTIAL — phase 11 stays open pending phase 11.1 translator fix |
