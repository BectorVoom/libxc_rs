# Phase 11: Splitter v2 — Discussion Log (2026-05-18, 4th iteration)

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-05-18 (fourth session)
**Phase:** 11-splitter-v2-unified-5k-cap
**Session type:** Discuss-phase architectural replan triggered by plan 11-06 HALT (commit `75c0f5112`)
**Areas discussed:** Architectural path forward + Pre-bulk validation gate + (emergent) Test scope expansion

---

## Setup: Existing-CONTEXT handling

| Option | Description | Selected |
|--------|-------------|----------|
| Update it | Layer 4th-iteration architectural decisions on top of existing D-01..D-18 | ✓ |
| View it first | Display existing CONTEXT.md before deciding | |
| Skip | Use existing CONTEXT.md as-is and exit | |

**User's choice:** Update it
**Notes:** 18 D-XX decisions already locked from 3 prior sessions; new decisions layer on top without re-litigating locked ones.

---

## Gray area selection

| Option | Description | Selected |
|--------|-------------|----------|
| Architectural path forward (Recommended) | A1 / A2 / C / Hybrid resolving the F::new(val: f32) vs f64-const blocker | ✓ |
| Pre-bulk validation gate | How to prove a new script's policy works BEFORE bulk-applying on 11 helper files | ✓ |
| Anti-pattern catalog update (AP-7) | Codify the 11-06 failure mode | |
| Revert scope (decoupled) | What gets undone from commits 7a65f3bc6 / dcb7d517d / 233a8890d | |

**User's choice:** Architectural path forward + Pre-bulk validation gate
**Notes:** AP-7 emerged organically from the validation-gate discussion and was folded into D-22's structural mitigation; revert scope flowed from the architectural choice and is captured in D-23.

---

## Emergent area: Test scope (introduced by user directive "Please modify all test for f32 and f64")

After the first batched question, the user introduced a new directive: "Please modify all test for f32 and f64." Clarification round selected option 1: every test in the codebase is parameterized over both f32 and f64.

### Q1: Test surface scope

| Option | Description | Selected |
|--------|-------------|----------|
| Everything: helper + chunk + spike + parity + 649-functional oracle | All test files across all crates, both precisions | ✓ |
| Helper + chunk + spike + parity (no full 649-functional sweep) | Skip the 649-functional oracle sweep under f32 | |
| Helper + chunk + spike (no parity vs oracle) | F32 gated on compiles + finite output only | |
| Chunk + per-functional subcrate tests only (helpers stay concrete f64) | Preserves Hybrid/C viability | |

**User's choice:** Everything
**Notes:** Locks A1 — helper-level f32/f64 parametric tests require generic-over-F helpers.

### Q2: F32 oracle tolerance policy (revising D-03)

| Option | Description | Selected |
|--------|-------------|----------|
| 1e-6 relative (typical f32 mantissa) — Recommended | Energy + routed derivatives at 1e-6 vs f64 oracle | ✓ |
| 1e-5 relative (more permissive) | Broader margin without per-test exceptions | |
| Per-test/per-derivative-class configurable | Tolerance table per (functional, derivative order) | |
| Compile-only under f32 (no oracle comparison) | F32 gated on compiles + finite output only | |

**User's choice:** 1e-6 relative
**Notes:** D-19a amends D-03 — f32 is now a first-class correctness target at relaxed tolerance.

### Q3: F32 test execution mode

| Option | Description | Selected |
|--------|-------------|----------|
| Always-on (Recommended) | Every cargo test runs both precisions | |
| Env-gated (default f64, LIBXC_RS_F32=1 enables f32) | Local default f64; CI runs both | ✓ |
| Feature-gated (cargo test --features test-f32) | Cargo feature flag | |
| Separate test binaries (test_f64.rs / test_f32.rs as siblings) | Two side-by-side test files per surface | |

**User's choice:** Env-gated (`LIBXC_RS_F32=1`)
**Notes:** D-19b — avoids day-to-day test-time doubling while keeping f32 a first-class target.

---

## Architectural path forward

### Q1: Architectural path (with test-scope constraint factored in)

| Option | Description | Selected |
|--------|-------------|----------|
| A1: cast_from script + surgical fixes — Recommended | Aligns with locked D-02 = Option A; required by D-19's helper-level dual-precision test scope | ✓ |
| Hybrid: Phase-1 generic + Phase-2 reverts + translator casts | Phase-2 reverts incompatible with D-19 helper-level tests | |
| C: Full Option C revival | Helpers concrete f64; incompatible with D-19 helper-level tests | |
| A2: f32 demote | Violates 1e-12 oracle gate (REQUIREMENTS.md:4 + CLAUDE.md) — non-starter | |

**User's choice:** A1
**Notes:** Once D-19 mandates helper-level parametric tests over F, A1 is the only viable path — Hybrid/C have concrete-f64 helpers and can't support that.

### Q2: Primary edit tool for A1

| Option | Description | Selected |
|--------|-------------|----------|
| Serena MCP (Recommended; D-18 already locked) | Semantic-aware via LSP; classifies identifiers, skips non-generic files | ✓ |
| Extended Python regex | Lightest investment; risks repeating 11-05 failure pattern | |
| Python + Rust AST parser | Heaviest setup; strongest correctness | |
| Mix: Serena for semantic + Python for bulk literal wrapping | Hybrid approach | |

**User's choice:** Serena MCP
**Notes:** Reaffirms D-18. Pure regex rejected after 11-06 HALT showed semantic awareness is required (f64 const vs f32 literal vs doc-comment vs string-literal vs range-op vs `_f64` suffix).

---

## Pre-bulk validation gate

### Q1: Gate structure

| Option | Description | Selected |
|--------|-------------|----------|
| All three in sequence (Recommended) | (1) Synthetic fixture, (2) bessel.rs canary, (3) chunk → helper spike on mgga_c_b94 at both precisions | ✓ |
| Canary file + spike harness only | Skip synthetic fixture | |
| Synthetic fixture + spike harness only | Skip per-real-file canary | |
| Spike harness only | Riskiest — closest to what 11-05 did | |

**User's choice:** All three in sequence
**Notes:** Captured in D-22. Strict sequencing required; AP-7 violation if Gate 3 runs before Gates 1+2 are green.

---

## Todo handling

| Option | Description | Selected |
|--------|-------------|----------|
| Defer to Phase 10 (Recommended) | The audit is workspace-modular-split prep, not Phase 11 architecture | ✓ |
| Fold into Phase 11 scope | Expand Phase 11 deliverables | |

**User's choice:** Defer to Phase 10
**Notes:** Captured in `<deferred>` section of CONTEXT.md so it doesn't resurface in future cross-phase todo matching.

---

## Claude's Discretion

- Specific path for the synthetic fixture file in D-22 Gate 1 (`tools/refactor_test_fixtures/symbol_class_matrix.rs` suggested; planner confirms after reading tool structure)
- Specific transformation of `special.rs:224` `F::F::new(MAX)` (could be `f64::MAX`, `F::cast_from(f64::MAX)`, or context-dependent — planner inspects pre-commit state)
- Implementation surface for the per-test f32 tolerance overrides in D-19c (separate table file / per-test attribute / runtime config — planner's call)
- Exact spelling of the env var `LIBXC_RS_F32` (could be `LIBXC_RS_F32=1`, `LIBXC_RS_TEST_F32=1`, or similar — planner confirms after reading existing env-var conventions in the codebase)
- Whether the env-gate is read once at suite startup or per-test (D-19b notes "single env-var check at startup" as the discovery; planner verifies feasibility)

---

## Deferred Ideas

- Audit error/ and math/ module placement before workspace-modular-split phase (deferred to Phase 10 per todo handling)

---

*Log date: 2026-05-18*
*Session: Phase 11 discuss-phase 4th iteration — post-11-06 HALT architectural decision*
