# Phase 11: Splitter v2 — Unified Kernels with 5K Line Cap - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in 11-CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-05-13
**Phase:** 11-splitter-v2-unified-5k-cap
**Areas discussed:** Expression-subdivision strategy, Cross-file ABI, Precision policy, File layout, Verification gate, Phase ordering, RAM ceiling

---

## Pre-discuss locked decisions (carried in from /gsd-quick promotion)

The originating /gsd-quick session locked four decisions before promotion to phase status. Captured here for the audit trail; they appear in 11-CONTEXT.md as D-LOCK-A through D-LOCK-D.

| Decision | Locked value |
|---|---|
| Unification scope | Collapse per-family subcrates only (`mgga-N/`, `gga-N/`, `lda-N/`). Multiple files per functional permitted. |
| 5,000-line cap | HARD cap; splitter must subdivide single output expressions. Not aspirational. |
| Existing 8nv quick task | Superseded; .planning/quick/260513-8nv-update-splitter-tool-enforce-3000-line-c abandoned in favor of this phase. |
| Iteration requirement | Pipeline must be re-run until both invariants hold AND cargo build --workspace passes. |

The /gsd-quick orchestrator surfaced these as gray areas the user pre-committed; the discuss-phase below addressed the remaining six.

---

## Expression-subdivision strategy

| Option | Description | Selected |
|--------|-------------|----------|
| CSE-aware (Recommended) | Detect common subexpressions / multi-use temporaries in the Maple AST, lift each into a `#[cube] fn`. Aligns with cubecl_macro_fanout_manual §10. Highest engineering cost; produces cleanest split. | ✓ |
| AST-level token-budget chunking | Walk the Maple AST, peel off subtrees once a token/line budget is hit, emit each subtree as a `#[cube] fn`. No semantic awareness. | |
| Per-statement banding | Group N consecutive Rust statements into a helper. Risks violating manual §10. | |

**User's choice:** CSE-aware (Recommended)
**Notes:** Selected on first pass, no clarification needed.

---

## Cross-file ABI for subdivided expressions

| Option | Description | Selected |
|--------|-------------|----------|
| Free `#[cube]` functions, value args/returns (Recommended) | Each chunk is `#[cube] fn chunk_NN(...) -> ...` taking explicit f64 args, returning explicit f64 tuples. Aligns with manual §10/§19/§4. | ✓ |
| Helper struct with `#[cube] impl` block | Group per-kernel chunks as methods on a state struct. Manual §9 warns against this. | |
| Cube-shared mutable state bag | One mutable params/temps struct passed through every chunk. Risk: ABI brittleness. | |

**User's choice:** Free `#[cube]` functions, value args/returns
**Notes:** User added constraint — "but user can select f64 or f32." This is a precision-policy change. Surfaced as a follow-up question (see Precision policy below) because it conflicts with CLAUDE.md's existing "f64 only; no silent f32 fallback" rule.

---

## Precision policy (follow-up to Cross-file ABI clarification)

| Option | Description | Selected |
|--------|-------------|----------|
| Generic `<F: Float>` kernels, f64 the default + correctness target (Recommended) | Helpers are `#[cube] fn chunk_NN<F: Float>(...)`. f64 stays the oracle-verified target (1e-12 gate). f32 is an opt-in launch-time choice with no correctness guarantee. CLAUDE.md updated to reflect. Aligns with manual §6/§13. | ✓ |
| Generic `<F: Float>`, both f32 and f64 verified | Same plus f32 oracle parity gate at relaxed tolerance. Doubles verify-time cost. | |
| Keep f64-only, generic is plumbing-only | Helpers carry `<F: Float>` but only `F = f64` wired at launch boundary. f32 path stays dead code. | |

**User's choice:** Generic `<F: Float>` kernels, f64 the default + correctness target
**Notes:** CLAUDE.md update is in scope for Phase 11 execution. f32 oracle gating deferred to future phase.

---

## File layout when one functional spans N files

| Option | Description | Selected |
|--------|-------------|----------|
| `_partNN` suffix files (Recommended) | Continue current convention: `mgga_c_kcis/lxc_pol_part01.rs`, `_part02.rs`, ... Functional entry stub re-exports the assembled function. | ✓ |
| Directory per output field, files per chunk | `mgga_c_kcis/lxc_pol/chunk_01.rs`, `chunk_02.rs`. More structure; deeper nesting. | |
| Single file with internal `mod chunk_NN` | Keep `lxc_pol.rs` as one file with `mod chunk_01 { ... }`. Does NOT reduce file line count. | |

**User's choice:** `_partNN` suffix files (Recommended)
**Notes:** Continuity over churn — the existing splitter already emits `_partNN` for some functionals, so this avoids a tree-wide rename in addition to the subcrate collapse.

---

## Verification gate

| Option | Description | Selected |
|--------|-------------|----------|
| 1e-12 relative error on energy + all routed derivatives at f64 (Recommended) | Match existing project standard. Run verify/ regression sweep on representative LDA/GGA/MGGA at f64 after each iteration. | ✓ |
| Bit-exact f64 vs pre-phase baseline | Identical f64 bits. Risk: CSE-aware subdivision introduces sequence points that re-order accumulation — likely impossible. | |
| Energy-only at 1e-12 + derivatives at 1e-9 | Energy strict, derivatives relaxed. Faster iteration; would not have caught Phase 4 vrho 33% bug (relative error was far above 1e-9). | |

**User's choice:** 1e-12 relative error on energy + all routed derivatives at f64 (Recommended)
**Notes:** Anchors to the existing project-wide correctness standard. Iteration cost is acceptable given the phase's "iterate until clean" framing.

---

## Phase 11 vs Phase 10 ordering

| Option | Description | Selected |
|--------|-------------|----------|
| Phase 11 first (Recommended) | Land subcrate collapse + 5K cap before Phase 10. Phase 10 inherits a clean kernel layer. | ✓ |
| Phase 10 first | Land workspace boundary split first; Phase 11 operates inside the new layers. | |
| Parallel / interleaved | Both progress with explicit touchpoint coordination. Risk: merge conflicts. | |

**User's choice:** Phase 11 first (Recommended)
**Notes:** Risk acknowledged — Phase 11 is research-grade and slow; Phase 10 waits. Net win because Phase 10's clean-tree success criteria are cheaper to hit post-collapse.

---

## RAM ceiling

| Option | Description | Selected |
|--------|-------------|----------|
| Encode as phase constraint, hard rule (Recommended) | Phase 11 CONTEXT.md locks: inline executor, `cargo` jobs ≤ 2, `CARGO_BUILD_JOBS=2`. | ✓ (with override) |
| Encode as default, allow override per-task | Default the constraint but allow planner to relax for non-compile tasks. | |
| Don't encode — leave to executor judgment | Trust the executor's memory. | |

**User's choice:** Option 1, **with override**: `CARGO_BUILD_JOBS=1` (tighter than the memory's `≤ 2`).
**Notes:** User chose to tighten further than the project memory's default. Phase-scoped tightening — does NOT update the global memory (which stays at `≤ 2` for other phases). Rationale: splitter iteration loop hits the macro-heaviest crates and previously OOM'd at higher concurrency.

---

## Claude's Discretion

The following implementation surfaces are explicitly left to downstream planning + research:

- Internal structure of the CSE pass (Maple AST walker vs post-translation Rust AST walker vs Python-side IR).
- Whether to extend `tools/translate_*.py` in place or fork a `tools/translate_v2/` tree.
- Exact migration path for the 22 numbered subcrates (in-place rename + merge vs new tree + cutover).
- Whether to add `tools/audit_kernel_size.py` (recommended but not locked).
- Whether to retain or fold `tools/split_oversized_*.py`, `tools/rebatch_mgga.py`, `tools/split_mgga_7_kcis.py`.

---

## Deferred Ideas

- f32 oracle gate at relaxed tolerance — possible future phase once f64 path stable.
- CI gate enforcing the 5K cap — natural follow-up to Phase 11.
- Workspace boundary refactor — that's Phase 10, sequenced after this one (D-06).
- `#[cube]` traits in kernel chunks — rejected at D-02; revisit only if a true trait-shaped abstraction emerges in a future phase.
- Bessel I0/I1 implementation for `mgga_x_2d_prp10` (libxc id 211) — pre-existing deferral from quick task 260510-q02. Phase 11 must not regress.

---

## Process notes

- Two questions had user clarification rounds before answering (Cross-file ABI → triggered the f32-vs-f64 precision question; RAM ceiling → triggered the jobs=1 tightening).
- All questions were batched per project memory `feedback_batch_questions` (2–4 questions per turn). One 4-question batch covered Q1–Q4; one 3-question batch covered the precision follow-up + Q3 + Q4; one 2-question batch covered Q5 + Q6.
- Underlying gsd-sdk lacks the `query` subcommand the workflow expects; orchestrator drove the workflow manually using direct file ops and Read/Edit/Write tools.
