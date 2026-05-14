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

---

# Re-discussion — 2026-05-14 (update session)

**Date:** 2026-05-14
**Trigger:** User re-ran `/gsd-discuss-phase 11` and chose "Update it." Plan 11-01 (Wave 0) was already executed; plans 11-02..06 existed. Two quick tasks (260514-q01, 260514-q02) had touched the same kernel area since the 2026-05-13 discussion.
**Areas discussed:** Unification target (new requirement from user), q01/q02 reconciliation, internal subcrate layout, verify-gate feasibility, deferred-kernel handling.

---

## New requirement — unification target (user-initiated)

The user rejected the initial gray-area batch to add a requirement directly: subcrates must be named by **functional id** (`gga_c_acgga`, `gga_c_gapc`), not family number (`gga-1`, `gga-2`). Crate structure `kernels/ > {lda,gga,mgga}/ > <func>/`, subcrates directly under the family directory. No build verification required now — `cargo build --workspace` is OOMing.

Two structural clarifications were asked:

| Question | Options | Selected |
|----------|---------|----------|
| Subcrate granularity | One per functional (all ~264) / Per-functional only for oversized | **One per functional (all ~264)** |
| Family level | Façade crate re-exporting subcrates / Plain directory, no crate | **Plain directory, no crate** |

**Effect on CONTEXT.md:** D-10 / D-10a / D-10b rewritten (per-functional subcrates, not per-family crates); D-LOCK-A revised; family directories lose their `Cargo.toml`/`lib.rs`.

---

## 5K-line cap status under the new structure

| Option | Description | Selected |
|--------|-------------|----------|
| Still a hard cap | Per-functional subcrates AND ≤5K-line files both required; CSE chunking (D-01) stays in scope | ✓ |
| Relax to soft/deferred | Subcrate isolation is the real OOM fix; 5K cap becomes soft / moves to follow-up | |
| Discuss it | — | |

**User's choice:** Still a hard cap.
**Notes:** Per-functional subcrates fix the OOM at the compilation-unit boundary; the 5K cap fixes per-file proc-macro fan-out. Both invariants hold. D-LOCK-B and D-01 unchanged.

---

## Reconciling q01/q02 hand-tuned output

| Option | Description | Selected |
|--------|-------------|----------|
| Clean-slate regenerate all (Recommended) | Splitter v2 regenerates all ~264 functionals from Maple; q01/q02 output discarded; splitter must reproduce ≤5K | ✓ |
| Preserve q01/q02, restructure only | Keep q01's mgga-2 files + q02's mgga_c_b94 nesting; only regenerate functionals still >5K | |

**User's choice:** Clean-slate regenerate all.
**Notes:** Consistent with D-10a clean-slate philosophy and D-LOCK-D idempotency (the splitter must reproduce everything anyway). Captured in revised D-10a.

---

## Internal layout within a per-functional subcrate

| Option | Description | Selected |
|--------|-------------|----------|
| Nested by output — q02 style (Recommended) | `src/kxc_pol/part01.rs`, `src/kxc_unpol/part01.rs` — group parts under the derivative they compute; supersedes flat `_partNN` | ✓ |
| Flat `_partNN` (keep D-04) | Keep flat files in `src/`: `kxc_pol_part01.rs` etc. | |

**User's choice:** Nested by output (q02 style).
**Notes:** Reverses the original D-04 (`_partNN` flat). q02's `mgga_c_b94` refactor (commit `504d8560`) is the concrete precedent. Within an isolated subcrate namespace the output-grouped nesting is clean.

---

## D-05 verify-gate feasibility

| Option | Description | Selected |
|--------|-------------|----------|
| Narrow deps + smoke per iteration (Recommended) | verify/ depends on individual functional subcrates, not umbrellas; per-iteration gate = representative smoke parity at 1e-12; full per-subcrate sweep at phase end | ✓ |
| Full sweep every iteration | Keep D-05 as written — complete regression sweep after every iteration | |
| Discuss it | — | |

**User's choice:** Narrow deps + smoke per iteration.
**Notes:** Structural fix for the verify/ OOM confirmed in Wave 0 deviation D1. Tolerance (1e-12, energy + routed derivatives, f64) unchanged. Captured in revised D-05.

---

## Deferred-kernel handling

| Option | Description | Selected |
|--------|-------------|----------|
| Exclude from default-members (Recommended) | 6 deferred kernels + mgga_x_br89_explicit are normal subcrates omitted from `[workspace] default-members`; built only via `-p` | ✓ |
| Feature-gate them | Port q02's reverted feature-gate approach to the subcrate structure | |
| Discuss it | — | |

**User's choice:** Exclude from default-members.
**Notes:** New D-11. Supersedes q02's feature-gate approach (committed `1eec03e2`, reverted `59b11dcd`). Per-functional subcrates make `default-members` exclusion the natural mechanism.

---

## Re-discussion process notes

- User rejected the first gray-area AskUserQuestion batch to inject a new requirement, then rejected the reformulated questions once for clarification. After the clarification was given, two structural questions + the 5K-cap question + the gray-area multiselect were batched into one 4-question turn; the four gray-area deep-dives were batched into a second 4-question turn. All recommended options were selected.
- The plans-exist gate was not asked as a separate question — given the magnitude of the D-10 rewrite, replanning 11-02..06 is a stated consequence (see CONTEXT.md re-plan note), surfaced as the primary next step.
- New decisions added: D-11 (deferred-kernel handling), D-12 (build verification not a phase gate). Revised: D-04, D-05, D-10, D-10a, D-10b, D-LOCK-A.
