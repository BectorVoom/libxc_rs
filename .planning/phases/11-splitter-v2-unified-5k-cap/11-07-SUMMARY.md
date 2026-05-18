---
phase: 11-splitter-v2-unified-5k-cap
plan: 07
subsystem: batched-compile-sweep, sweep-tool-authoring, partial-sweep-evidence
tags: [batched-compile-sweep, jobs3-invocation-policy, halt-and-surface, peak-rss, D-31, D-32, D-33, D-34, AP-2-narrowed, AP-8-clarified, translator-rule-3-gap, partial-sweep]
status: PARTIAL — Task 1 (tool authoring) COMPLETE; Task 2 (full sweep) PARTIAL — single-family (LDA) run executed and halted at lda_c_pk09 per design; remaining families (GGA, MGGA) deferred to a phase-11.1 translator-fix follow-up.
completed: 2026-05-18
---

# Phase 11 Plan 07: Batched compile sweep — PARTIAL

## Outcome

**Task 1 GREEN, Task 2 PARTIAL by user decision.** `tools/batched_compile_sweep.py`
was authored to spec (D-31 pure orchestrator, D-32 `--jobs 3` per-invocation,
D-33 bounded 2-pass, D-34 peak-RSS measurement) and committed at `6e2a793fb8`.
Per-family LDA sweep was dispatched as the diagnostic instrument for the
translator Rule 3 emission defect surfaced during 11-06 Leg 2.

**Halt outcome — design-intended.** The LDA sweep ran for 17m 22s, completed
12 LDA subcrates successfully in pass 2, then halted at `libxc-kernel-lda_c_pk09`
with a confirmed compile failure (789 errors). The sweep tool's HALT-and-surface
behavior worked exactly as specified.

**User disposition (2026-05-18):** Skip the remaining LDA + full GGA + full MGGA
sweep runs — the two existing failure samples (LDA tuple-return, GGA inline-named-const)
already prove the translator Rule 3 emit gap is graph-wide; additional sweep
passes would re-confirm the same root cause without adding actionable
information. Move to 11-08 audits with partial-sweep evidence; scope the
translator fix in a follow-up phase 11.1.

## Tasks

| # | Task | Status | Notes |
|---|---|---|---|
| 1 | Author `tools/batched_compile_sweep.py` | COMPLETE | commit `6e2a793fb8` — 515 lines, all 9 verification gates pass (forbidden-grep, required-grep, dry-run-all, dry-run-lda, dry-run-resume) |
| 2 | Execute the sweep across 266 on-disk subcrates | PARTIAL | LDA family swept (12 OK + 1 HALT at `lda_c_pk09`); GGA + MGGA deferred to phase 11.1 |

## Sweep tool — implementation deviations from plan

Documented inline in `tools/batched_compile_sweep.py` module docstring + commit body.

1. **Roster source: on-disk dirs, not `cached_routed_funcnames` UNION carve-outs.**
   Plan said LDA 43 + GGA 131 + MGGA 92 + 7 carve-outs = 273 packages, but
   `cached_routed_funcnames` actually returns the **routed-in-model** set
   (36 LDA + 105 GGA + 25 MGGA = 166 routed), while the plan's "43/131/92"
   matches the **on-disk** subcrate dir count (43+131+92 = 266 total; the 7
   carve-outs are already in the 92 MGGA on-disk count, all on-disk-but-unrouted).
   Per plan's escape hatch (`total_packages=N where N is the actual count`),
   the tool sweeps the on-disk superset (266 packages) — strictly broader
   coverage than routed-only, and aligns with the planner's intended scale.

2. **Per-package cargo invocation within batches, not combined `-p pkg1 -p pkg2`.**
   Plan offered both forms as planner's discretion. Chose per-package to avoid
   cargo registry/index lock contention and to give clean per-package wall-clock
   and peak-RSS attribution. Batch parallelism is still `--jobs N` rustc subjobs
   per package.

3. **No `cargo workspace` invocation anywhere** (D-12 honored).

## Sweep evidence (durable; under phase dir)

- `11-07-SWEEP-MANIFEST-lda.ndjson` (13 records: 12 ok pass=2, 1 fail pass=-1) — D-31 manifest evidence
- `11-07-SWEEP-SUMMARY-lda.md` — D-33 summary, VERDICT line, peak-RSS table
- `11-07-SWEEP-HALT-lda.md` — D-33 trimmed halt artifact (frontmatter + first 150 + last 50 of 789 errors; full log at `/tmp/sweep-lda-halt.md` in session scratch)
- Full sweep ndjson at `/tmp/sweep-lda.ndjson` (session scratch)

## Failure pattern catalog (combined Leg 2 + LDA sweep evidence)

| pattern | first observed in | exemplar | error shape |
|---|---|---|---|
| P1: Named-const ref inline in F-typed arithmetic | `gga_c_gaploc::lxc_pol::part53::chunk804.rs:13` (11-06 Leg 2) | `let t40620 = t9105 * t5337 * M_PI * t1691 * ...;` | `expected type parameter F, found f64` on `M_PI` |
| P2: Tuple-return chunk with bare f64 tuple member | `lda_c_pk09::fxc_pol::part2::chunk5.rs:15` (11-07 sweep) | `pub fn ...<F: Float>(...) -> (F, F, F) { ...; (t6, t7, t8) }` where `t8` (or similar) is bound to a bare f64 literal | `expected type parameter F, found f64` on tuple member |

Both patterns share the same root cause: **the translator's chunk-body emit path does not apply Rule 3 (`F::cast_from(<f64-literal>)` or `F::new(<f32-safe-literal>)`) to f64-literal positions inside the function body**. Deviation F (commits `4aaaaa7739`/`8a9f32091e`/`d26efabda6`) extended Rule 10 turbofish to cross-fn calls but did not touch in-body f64 occurrences.

## Sweep metrics

- LDA sweep wall-clock: 1042.1 s (17m 22s)
- Peak-of-peak RSS: 17,292.8 MB (~17.3 GB — within the 24-GB-host / 21-GB-budget envelope per D-34; jobs=3 envelope confirmed adequate, no re-calibration to --jobs 2 needed)
- Pass-1 batch attempt: 0 successes (the first failing package short-circuited pass 1)
- Pass-2 sequential retry: 12 LDA subcrates compiled cleanly; 1 (lda_c_pk09) confirmed failed; 30 LDA packages remaining unrun

The 12 clean LDA subcrates (no Rule 3 violations in their chunk bodies) span small or trivially-typed functionals: `hyb_lda_xc_bn05`, `lda_c_1d_csc`, `lda_c_1d_loos`, `lda_c_2d_amgb`, `lda_c_2d_prm`, `lda_c_br`, `lda_c_cgnssc`, `lda_c_chachiyo_g`, `lda_c_chachiyo_mod`, `lda_c_gk72`, `lda_c_gombas`, `lda_c_hl`, `lda_c_lp96`, `lda_c_ml1`. (Last entries OK before `lda_c_pk09` halt.)

## Phase 11 follow-up scope (NOT this plan)

Translator fix required before sweep can complete cleanly. Recommended next plan (phase 11.1 or fresh phase 12):

1. Amend `tools/translate_v2/` to enforce Rule 3 across ALL f64-literal positions in chunk-body emit (not just cross-fn call arguments):
   - Named-const refs in arithmetic: emit `F::cast_from(M_PI)` (or hoisted `let pi = F::cast_from(M_PI)`)
   - Tuple-return members from bare numeric literals: emit `F::cast_from(<literal>)` or `F::new(<literal>)` per Rule 2/3 boundary
   - Any other `let x = <f64>` binding fed into an F expression downstream
2. Full-tree regen (supersedes Deviation F commits)
3. Re-run `python3 tools/batched_compile_sweep.py` end-to-end; iterate until VERDICT: ALL_OK
4. Resume 11-06 Task 6 Legs 2/3/4 + Task 8

## Why "PARTIAL" is the right closure for 11-07 (and not "FAILED")

Task 1 (sweep tool) is the deliverable that survives phase 11 — it's the entry-gate codification per memory `project_phase11_structural_without_compile`. The tool works as specified; its first run halted as designed. Task 2's partial scope is by user disposition, not tool failure. SPEC-11-R4 (per-`-p` compile evidence) is partially satisfied for the 12 OK LDA packages, with the partial manifest as durable evidence. The remaining sweep is deferred to a follow-up phase rather than re-attempted now (would re-confirm same root cause).

## Commits

| SHA | Message |
|---|---|
| `6e2a793fb8` | feat(11-07): tools/batched_compile_sweep.py — pure orchestrator (D-31/D-32/D-33) |
