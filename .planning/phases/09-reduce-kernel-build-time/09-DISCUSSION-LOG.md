# Phase 9: Reduce Kernel Build Time — Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions captured in CONTEXT.md (`09-CONTEXT.md`) — this log preserves the discussion shape.

**Date:** 2026-04-29
**Phase:** 09-reduce-kernel-build-time
**Mode:** discuss (interactive, default mode)
**Existing artifacts at session start:** SPEC.md (locked, 6 reqs), RESEARCH.md, plans 09-01 through 09-04 (09-01/02/03 complete, 09-04 pending), VALIDATION.md
**Existing artifacts at session end:** SPEC.md (revised, 3 reqs post-Round 4), CONTEXT.md (new), DISCUSSION-LOG.md (this file)

## Pre-discussion Decisions (from existing artifacts)

The following were already locked when the session began:
- **6 SPEC requirements** locked at ambiguity 0.16 (≤180s default build, family feature gates, cfg-gated re-exports, 25 deferred GGAs at full orders, ≤5K-line file cap, no profile drift)
- **Plans 09-01, 09-02, 09-03 are committed** (translator annotations, kernel re-translation, GGA bin-packing into ~60 sub-crates + MGGA into ~109)
- **Plan 09-04 is pending** (originally scoped to add `[features]` + cfg-gates; obsolete after Round 4)
- **Phase has 4 plans total**, of which 3 have SUMMARY.md files

## Pre-flight User Question

| Q | Header | Question | User answer |
|---|--------|----------|-------------|
| 0 | Plans exist | Phase 9 has 4 plans (09-01–04), 09-04 pending. Continue and replan after, view existing, or cancel? | "Continue and replan after (Recommended)" |

## Gray Area Selection

| Q | Header | Question | User answer |
|---|--------|----------|-------------|
| 1 | Discuss | Which implementation areas to discuss? (multi-select) | All four selected: File-splitting mechanism, Unguarded mgga import in src/, Plan 09-04 revision vs new plans, Build-time verification & sequencing |

## Round 3 (line-count cap)

User intervened on Gray Area 1 with two consecutive directives raising the per-file cap.

**Directive 3a** — "Since we have enough memory headroom, please set the line-count cap to **10K lines**."
- Effect: file violation count dropped from 201 → 16 (3 LDA, 3 GGA, 10 MGGA).
- SPEC.md Goal, Requirement 5, Constraints, Acceptance Criterion all updated 5,000 → 10,000.
- Interview Log row 3 added.

**Directive 3b** — "Since we have enough memory headroom, please set the line-count cap to **20K lines**."
- Effect: file violation count dropped from 16 → 0 (largest existing file is 16,703).
- SPEC.md updated again to 20,000 in all four places.
- Acceptance criterion gained `$2 != "total"` filter to skip the multi-file `wc -l` total row.
- Gray Area 1 (file-splitting mechanism) collapsed to "no remediation needed; cap retained as forward guard."

## Round 4 (feature-gating removal)

User issued two directives that fundamentally narrowed the phase scope.

**Directive 4a** — "Do **not** make `mgga` optional."
- Conflict surfaced: SPEC's Requirement 1 (≤180s default build) unreachable if MGGA is always-on. I asked for reconciliation guidance (4 options).

**Directive 4b** — "Drop the ≤180s default-build target and default-build is lda,gga,mgga."
- Combined with 4a, this removed Original Requirements 1, 2, 3 entirely (default-build family scope, family feature gates, cfg-gated re-exports).
- SPEC.md restructured: 6 requirements → 3 requirements, with "Removed Requirements (Round 4)" section noting the removed ones.
- Gray Areas 2, 3 (unguarded mgga import; Plan 09-04 disposition) recharacterized: mgga gating is no longer in scope at all; Plan 09-04 became fully obsolete (it was about feature-gating).
- Roadmap requirements `BUILD-OPT-02` and `BUILD-OPT-03` deferred to a future phase.
- Phase narrowed to: unblock 25 deferred GGAs at full orders + maintain forward-guard caps.

## Post-narrowing Discussion Questions

| Q | Header | Options | User answer |
|---|--------|---------|-------------|
| 2 | Residual | Verify-then-sweep / Single comprehensive plan / Dig deeper / Archive 09-04 no replacement | **Verify-then-sweep (Recommended)** |
| 3 | Sweep scope | Full sweep 1e-12 / Full relaxed / Smoke test / Defer to separate phase | **Full sweep, strict 1e-12 (Recommended)** |
| 4 | Audit method | Generated audit report / Manual table / cargo check + grep / Truth table from libxc | **Generated audit report (Recommended)** |

## Round 5 (translator threshold raise — fold into Plan 09-04)

User directive — "Since we have enough memory headroom, it is possible to increase the splitting threshold. Please increase the splitting threshold and generate the kernels."

I did not execute this directly (we are in `/gsd-discuss-phase`, decision capture only). Instead I asked two parameter-locking follow-ups so the directive could be captured cleanly as a plan.

| Q | Header | Options | User answer |
|---|--------|---------|-------------|
| 5 | Threshold | 18K (Recommended) / 15K / 20K = SPEC cap / 10K | **18,000 lines (Recommended)** |
| 6 | Regen scope | All three families (Recommended) / GGA only / GGA + MGGA / Only the 25 deferred | **All three families (Recommended)** |

This recast the new Plan 09-04 from "audit script" into "translator threshold raise + regenerate all three families." The audit script moved to a new Plan 09-05; build verification became 09-06; oracle parity sweep became 09-07.

## Mid-Discussion Build-Command Question

| Q | Header | Options | User answer |
|---|--------|---------|-------------|
| 7 | Build cmd | cargo check (Recommended) / cargo build / Both | **cargo check (Recommended)** |

(Per user's project memory: prefer `cargo check -p <crate>` for compile-error verification.)

## Wrap-up Question (asked but not answered before user issued Round 5 directive)

| Q | Header | Options | User answer |
|---|--------|---------|-------------|
| 8 | Done | I'm ready for CONTEXT / Explore more gray areas | (no explicit answer — user proceeded with Round 5 directive instead, which I treated as substantive enough to wrap discussion after threshold/scope locked) |

## Deferred Ideas Captured

See CONTEXT.md `<deferred>` section. High-level items mentioned during discussion but moved out of Phase 9 scope:

- Family feature gates (`gga`, `mgga`, `all-kernels`)
- Default-build wall-clock targets
- Cfg-gating mgga/gga refs in src/
- Sub-crate re-bin-packing post-regen
- 6 deferred MGGAs (Brent root-finders)
- RESEARCH.md refresh (stale post-09-03)
- Phase directory rename (`09-reduce-kernel-build-time` no longer matches scope)
- Full MGGA oracle re-sweep (Plan 09-07 only does spot-checks)

## SPEC Revisions Made During This Session

Two cumulative revisions to `09-SPEC.md`:

| Round | Locations touched | Net effect |
|-------|-------------------|------------|
| Round 3 | Goal; Requirement 5 (now 2); Constraints (per-file cap); Acceptance Criterion (file-cap awk); Interview Log row 3 | File cap 5K → 10K → 20K. 0 violations today. Forward-guard semantics. |
| Round 4 | Goal (rewritten); Requirements (3 deleted, 4/5/6 renumbered to 1/2/3 + "Removed Requirements" section added); Boundaries (rewritten in/out of scope); Constraints (≤180s removed; "No family feature gates" added); Acceptance Criteria (rewritten — feature/wall-clock criteria removed, audit/parity/log criteria added); Interview Log row 4; Spec front-matter ("Requirements: 3 locked"); Footer (Next Step note rewritten) | Phase narrowed from 6 reqs to 3 reqs. Build-time-reduction work deferred to a future phase. |

## Claude's Discretion Items

Items where the user didn't specify and Claude has flexibility during planning/execution:
- Exact `_partN`/`_partN_subM` numbering under the new 18K threshold
- Audit report format (JSON vs markdown)
- Whether the audit script also covers MGGA + LDA non-regression as a freebie
- Exact rayon parallelism level for the parity sweep
- Whether the parity sweep uses one or all four oracle test systems (must at minimum include both spin-zero and spin-polarized)

## Notes for Future Sessions

- The user's pattern is **directive-first, parameters-second**: they assert the design decision ("don't make mgga optional", "increase threshold") before the implementation parameters get nailed down. Best to acknowledge the directive, capture implications, then ask focused parameter questions rather than relitigating the directive.
- The user prefers `cargo check` over `cargo build` and always wants logs in `log/<descriptive>.log` (recorded in feedback memories `feedback_cargo_check.md` and `feedback_cargo_logs.md`).
- The user's memory headroom claim ("we have enough memory headroom") was used twice to relax constraints; this is a stable signal for the dev machine's capacity rather than a one-off.

---

*Phase: 09-reduce-kernel-build-time*
*Discussion captured: 2026-04-29*
*Mode: discuss (default, interactive)*
