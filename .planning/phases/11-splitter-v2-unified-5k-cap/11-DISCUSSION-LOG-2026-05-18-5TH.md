# Phase 11: Splitter v2 — Unified Kernels with 5K Line Cap - Discussion Log (5th Session)

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-05-18 (fifth discuss-phase session)
**Phase:** 11-splitter-v2-unified-5k-cap
**Triggered by:** 4th-iter HALT at D-22 Gate 2 (commit `3494c80fc`)
**Areas discussed:** Architectural direction (A/B/C), D-19 dual-precision test scope, Phase-2 commit disposition + tooling preservation, Anti-pattern AP-8 codification

---

## Pre-discussion: Context governance

| Option | Description | Selected |
|--------|-------------|----------|
| Update it (5th-iter) | Revise CONTEXT.md to lock the 5th-iter architectural direction and capture lessons from the 4th-iter HALT | ✓ |
| View it first | Show CONTEXT.md before deciding | |
| Skip | Use existing CONTEXT.md as-is (NOT recommended — locked A1 path is empirically disproven) | |

**User's choice:** Update it (5th-iter)

| Option | Description | Selected |
|--------|-------------|----------|
| Continue, replan 06-08 after | Capture 5th-iter context now, then /gsd-plan-phase 11 to regenerate 11-06/07/08 | ✓ |
| View existing plans first | Show stale 11-07 and 11-08 PLAN.md before deciding | |
| Cancel | Stop the discuss-phase here | |

**User's choice:** Continue, replan 06-08 after

---

## Gray Area Selection (multiSelect)

| Option | Description | Selected |
|--------|-------------|----------|
| Architectural direction (A/B/C) | REQUIRED. Pick A (manual Phase-2 redo, ~8-12h, definitive), B (Option C revival: translator emits casts, helpers stay concrete f64), or C (Hybrid) | ✓ |
| D-19 dual-precision test scope | Direction B kills helper-level f32+f64 dual-precision tests; Direction C keeps them only on Phase-1 helpers. Reaffirm D-19 as hard requirement, or relax | ✓ |
| Phase-2 commit disposition + tooling preservation | Direction A reverts 7a65f3bc6 + dcb7d57d + 233a8890d; cast_from policy is sound; D-22 Gate-1 fixture is valid. Keep tooling in tree, archive, or delete | ✓ |
| Anti-pattern codification (AP-8 candidate) | 4th-iter HALT empirically demonstrated 'automation-extension as architectural rescue'. Codify as AP-8 (blocking) | ✓ |

**User's choice:** All 4 selected

---

## Area 1: Architectural direction (A/B/C)

### Q1.1 — Which 5th-iter architectural direction to lock?

| Option | Description | Selected |
|--------|-------------|----------|
| A — Manual Phase-2 redo (recommended) | Revert 3 Phase-2 commits. Hand-convert 11 helper files using the proven Phase-1 pattern. ~8-12h manual. Preserves D-19 helper-level dual-precision tests. Definitive. | ✓ |
| B — Option C revival (translator cast) | Helpers stay concrete f64. Translator emits casts at call sites. ~581K regen. Sacrifices D-19 helper-level dual-precision tests. Reverses session-2 Option-C-rejection | |
| C — Hybrid (Phase-1 generic + Phase-2 concrete + translator cast) | Keep 5 Phase-1 clean files generic. Revert Phase-2 to concrete. Translator casts only for Phase-2 helper calls. Smaller translator surface; partial D-19 | |

**User's choice:** A — Manual Phase-2 redo
**Notes:** Locked as D-25. Rejected B (sacrifices D-19) and C (partial D-19 coverage; complexity of "some generic, some not"). Manual conversion of 11 files mirroring proven Phase-1 pattern is definitive.

### Q1.2 — Direction A revert scope

| Option | Description | Selected |
|--------|-------------|----------|
| Revert exactly 3 Phase-2 commits + keep D-23 surgical + keep Phase-1 | Standard scope. D-23 fixes are correct work; Phase-1 files are proven clean; only the Phase-2 batch-script outputs need to go | ✓ |
| Revert Phase-2 AND D-23 surgical (clean slate from pre-7a65f3bc6) | Fully cold-restart. Discards D-23 work even though correct | |
| Revert Phase-2 AND review D-23 file-by-file | Revert Phase-2; treat D-23 case-by-case. For partially-touched files, full revert might be cleaner | |

**User's choice:** Revert exactly 3 Phase-2 commits + keep D-23 surgical + keep Phase-1
**Notes:** Recorded in D-25. Preserved commits: `466e074d0` + `d8cc4da0c` (Phase-1) and `9df2880b3` (D-23 surgical). Reverted: `7a65f3bc6`, `dcb7d57d`, `233a8890d`.

### Q1.3 — Conversion cadence

| Option | Description | Selected |
|--------|-------------|----------|
| File-by-file with per-file compile gate (recommended) | One file at a time; cargo build -p libxc-kernel-math after each; commit atomically per file. Catches regressions immediately; per-file commits give clean revert points. Matches AP-6 entry-gate philosophy | ✓ |
| Small batches (3-4 files), gate per batch | Group conceptually-related files. Gate per batch. Fewer commits but coarser blast radius | |
| All 11 at once, then gate | Convert everything in one pass, then run compile gate. Closer to original Phase-2 auto-script's failure mode — NOT recommended given 4th-iter HALT evidence | |

**User's choice:** File-by-file with per-file compile gate
**Notes:** Recorded in D-26.

### Q1.4 — File order

| Option | Description | Selected |
|--------|-------------|----------|
| Hardest-first (bessel.rs as canary, then descending complexity) | Convert bessel.rs first. If a pattern issue surfaces, it surfaces on the canary; remaining 10 files benefit from the lessons. Matches AP-7 integration-boundary discipline | |
| Easiest-first (smallest/lowest-complexity files first) | Build confidence + establish the pattern cleanly on small files before tackling bessel.rs. Slower to surface architectural issues | ✓ |
| Original Phase-2 commit order (chronological from 7a65f3bc6) | Mirror the original conversion sequence. No particular signal advantage but matches existing git archaeology | |

**User's choice:** Easiest-first
**Notes:** Recorded in D-26. Reasoning: the pattern is proven on Phase-1 files; conversion is application, not exploration — easiest-first is correct discipline for pattern-application (vs hardest-first for novel architecture).

### Q1.5 — Per-file done-criterion

| Option | Description | Selected |
|--------|-------------|----------|
| Compile gate only (cargo build -p libxc-kernel-math green) | Per-file: file compiles cleanly within math crate. Minimum bar; fastest iteration | |
| Compile + spike_cse_emit_q01.rs + helper unit tests (recommended) | Per-file: compile + the existing q01 spike test passes + per-helper unit tests (where they exist). Catches semantic regression at conversion time, not at integration-time. Aligns with D-19 helper-level test mandate | ✓ |
| Compile + chunk→helper spike on mgga_c_b94 (full integration boundary) | Per-file: full chunk→helper integration test on mgga_c_b94 after each file. Strongest gate; slowest — mgga_c_b94 build is expensive on the 30GB machine | |

**User's choice:** Compile + spike_cse_emit_q01.rs + helper unit tests
**Notes:** Recorded in D-26. The mgga_c_b94 integration spike is the all-files-converted EXIT gate (D-22 amended Gate 3).

### Q1.6 — Canonical pattern reference

| Option | Description | Selected |
|--------|-------------|----------|
| polynomials.rs (cleanest — 0 F::new sites, simplest generic shape) | Smallest, most readable reference. Pure generic body; no f64 named constants | |
| lambert_w.rs (representative — 14 F::new sites, mix of patterns) | Mid-complexity reference. Shows standard literal-wrap + named-constant handling | |
| Spec-document derived from all 5 Phase-1 files (recommended) | Plan writes a brief PATTERN.md in .planning/phases/11/ extracted from all 5 Phase-1 files: literal-wrap rule, named-constant cast_from rule, doc-comment / string-literal handling, type-annotation handling, #[cube] pub-visibility rule. One canonical place; per-file conversion checks against it | ✓ |
| Planner's call — derive from Phase-1 during 11-06 replan | Don't pre-commit; let the planner pick or derive during plan-phase. Standard CONTEXT-vs-plan separation | |

**User's choice:** Spec-document derived from all 5 Phase-1 files
**Notes:** Recorded in D-27. PATTERN.md authored as task 3 of the 5th-iter 11-06 plan.

---

## Area 2: D-19 dual-precision test scope

### Q2.1 — D-19 scope under Direction A

| Option | Description | Selected |
|--------|-------------|----------|
| Reaffirm D-19 fully — dual-precision everywhere | All tests parameterize over both precisions. F32 env-gated via LIBXC_RS_F32=1 per D-19b. 1e-6 f32 tolerance per D-19a. This is the architectural intent that ruled out Direction B/C; reaffirming it locks the architectural value of Direction A | ✓ |
| Reaffirm D-19 architecturally, scope tests phase-end | Helpers + chunks compile against both F=f32 and F=f64 (compile-gate unconditional). But f32 TEST execution is deferred to phase-end (11-08). Per-file iteration (11-06) only runs f64 tests. Reduces per-file cycle time | |
| Relax D-19 to opt-in per-helper | Helpers are generic but f32 tests only enabled where they're known-cheap. Avoids the Brent root-finder convergence issues by deferring those entirely | |

**User's choice:** Reaffirm D-19 fully — dual-precision everywhere
**Notes:** Rules out any temptation to slip toward Direction B/C during execution.

### Q2.2 — F32 test execution timing during 11-06

| Option | Description | Selected |
|--------|-------------|----------|
| Per-file, immediately after f64 passes (recommended for D-19 full lock) | Each converted file: f64 test pass first, then LIBXC_RS_F32=1 re-run on the same tests. F32 failure on a file blocks moving to the next file. Maximum signal; longest per-file cycle | ✓ |
| Per-file f64, plus periodic f32 sweep (every N files) | Per-file f64 gate stays strict. F32 sweep runs after every 3-4 files. Catches drift without doubling per-file time | |
| F32 deferred to phase-end (11-08) full sweep | F64-only during 11-06. F32 first runs in 11-08 across all converted helpers + the 649-functional oracle. Lowest per-file cost; highest blast radius if f32 issues are file-localized | |

**User's choice:** Per-file, immediately after f64 passes
**Notes:** Recorded in D-26.

### Q2.3 — F32 tolerance policy for Brent-class helpers

| Option | Description | Selected |
|--------|-------------|----------|
| 1e-6 default + per-test override table (D-19c as-locked) | Default tolerance 1e-6 relative. Brent helpers + similar may relax per-test with documented justification in a per-(functional, derivative-order) override table | ✓ |
| 1e-6 default + 1e-4 fallback for iterative algorithms (Brent class) | Pre-commit a coarser fallback (e.g., 1e-4) for any helper containing a fixed-point or root-finding loop. Less per-test bureaucracy; more permissive than ideal | |
| 1e-6 hard for everything — iterative algorithms either converge or get excluded from f32 tests | No special tolerance. If a Brent helper can't hit 1e-6 in f32, that helper is skipped from f32 test surface (documented exclusion). Strictest policy | |

**User's choice:** 1e-6 default + per-test override table (D-19c as-locked)
**Notes:** Recorded in D-26. Planner specifies table location during 11-06.

---

## Area 3: Phase-2 commit disposition + tooling preservation

### Q3.1 — cast_from classifier disposition

| Option | Description | Selected |
|--------|-------------|----------|
| Keep in tree, document as 'fallback / future use' | Leave the classifier code in tools/. Add a header comment + 11-CONTEXT.md note that it's proven-correct for the Phase-2 corruption pattern; not used by Direction A but available if helper crate ever drifts into that state again. Zero cost to keep | ✓ |
| Archive to .planning/archived-tools/ | Move the classifier + Gate-1 fixture to .planning/archived-tools/ with a README. Keeps tools/ clean | |
| Delete (cleanest) | Remove the classifier code; git history preserves it. Tools/ stays minimal | |
| Keep classifier, delete Gate-1 fixture | Split disposition | |

**User's choice:** Keep in tree, document as 'fallback / future use'
**Notes:** Recorded in D-28. Required header comments to be added during 11-06 execution.

### Q3.2 — D-22 revision under Direction A

| Option | Description | Selected |
|--------|-------------|----------|
| Amend D-22: Gate 1 retire, Gate 2 replaced by per-file, Gate 3 kept as exit gate (recommended) | Gate 1 (synthetic fixture for classifier) retires. Gate 2 (bessel.rs canary) is subsumed by per-file gate. Gate 3 (mgga_c_b94 chunk→helper integration spike at f64 1e-12 AND f32 1e-6) becomes the all-files-converted EXIT gate before 11-07's full-tree regen | ✓ |
| Retire D-22 entirely; replace with per-file gate + 11-07 entry gate | D-22 was strategy-specific. Direction A has its own discipline. The chunk→helper spike rolls into 11-07's D-15 gate | |
| Keep D-22 as-is for record; planner decides applicability in 11-06 replan | Don't amend in CONTEXT.md; let the planner reframe during plan-phase | |

**User's choice:** Amend D-22 (Gate 1 retire, Gate 2 per-file, Gate 3 exit)
**Notes:** Recorded as amended D-22 in CONTEXT.md.

### Q3.3 — Partial-credit commits in git history

| Option | Description | Selected |
|--------|-------------|----------|
| Keep all in main's history as-is (recommended) | Commits are technically clean and document the 4th-iter attempt. Standard 'commits document attempts, don't get rewritten' policy | ✓ |
| Squash the 4 partial-credit commits into one 'archived' commit | Compress into a single archived commit. Reduces git log noise; preserves work. Requires interactive rebase | |
| Cherry-pick the classifier into a separate branch, drop from main | Move to phase-11-cast-from-archive branch and remove from main via revert. Cleanest main history | |

**User's choice:** Keep all in main's history as-is
**Notes:** Recorded in D-29.

---

## Area 4: Anti-pattern AP-8 codification

### Q4.1 — Codify AP-8?

| Option | Description | Selected |
|--------|-------------|----------|
| Codify AP-8 as BLOCKING (recommended) | Add AP-8 to Critical Anti-Patterns table at blocking severity. Triggers when an automation pass partially clears errors but error count fails to monotonically converge after 2 extension iterations. Same enforcement weight as AP-1/AP-2/AP-3 | ✓ |
| Codify AP-8 as WARNING | Add as warning-tier (like AP-4/AP-5). Documented but not gate-blocking | |
| Don't codify — covered implicitly by AP-1 | AP-1 (no in-plan retry-grinding) already covers the structural prohibition; AP-8 would be redundant | |

**User's choice:** Codify AP-8 as BLOCKING

### Q4.2 — AP-8 abort threshold

| Option | Description | Selected |
|--------|-------------|----------|
| Non-monotonic decrease across 2 consecutive extensions (recommended) | Threshold: if extension N+1 INCREASES total errors vs extension N (even when it 'fixes' one class), STOP. The 4th-iter signature: 121→507 (+386) trips this. Computer-checkable; empirically matches the observed failure | ✓ |
| Any extension that introduces a new error class (E0xxx not seen before) | Threshold: if extension N+1 surfaces an error code not present in extension N, STOP — uncovering structural-baseline issues, not finishing the original problem | |
| Hard cap: 2 extension passes after initial automation, then HALT regardless | Counter-based: initial pass + 2 extensions = 3 total. After that, no more extensions — HALT for /gsd-discuss-phase or pivot to manual. Strictest; least judgment-dependent | |
| Combine 'non-monotonic decrease' AND 'new error class introduced' (either trips it) | Either signature trips the abort. Tighter than option 1 or 2 alone | |

**User's choice:** Non-monotonic decrease across 2 consecutive extensions
**Notes:** Recorded in D-30 + AP-8 entry in anti-patterns table.

---

## Close-out

| Option | Description | Selected |
|--------|-------------|----------|
| Ready for context (write the 5th-iter revision) | Lock the decisions above. Write CONTEXT.md fifth-session revision + DISCUSSION-LOG. Commit. Then user runs /gsd-plan-phase 11 to regenerate 11-06/07/08 | ✓ |
| Explore more gray areas | Surface additional gray areas | |

**User's choice:** Ready for context

## Claude's Discretion

Carried forward from prior sessions — see CONTEXT.md `<decisions>` / Claude's Discretion section. No new Claude's Discretion items added in this session.

## Deferred Ideas

No new deferrals from this session. Prior deferrals carry forward — see CONTEXT.md `<deferred>` section.

---

*Session: 2026-05-18 fifth discuss-phase*
*Triggered by: 11-06 4th-iter HALT (commit 3494c80fc) at D-22 Gate 2*
*Outcome: D-25 Direction A locked; D-26..D-30 added; D-22 amended; D-24 amended; AP-8 codified BLOCKING*
