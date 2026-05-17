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

---

# Discussion: 2026-05-15 (second pause — D-14..D-17)

**Trigger:** Phase 11 paused mid-execution at plan 11-04 Task 1A. `.continue-here.md` documented a four-layer architectural blocker, with layer 4 (math/src/ helper concreteness vs `<F: Float>` chunks) identified as a planner-level reset event. User invoked `/gsd-discuss-phase 11` to lock D-02 disposition before replanning.

**Areas discussed:** D-02 ABI fate, Compile-first entry gate, Translator emit surface vs IR pass, Replan boundary, In-flight artifact disposition.

---

## Pre-discussion routing

| Option | Description | Selected |
|--------|-------------|----------|
| Update CONTEXT.md focused on D-02 + replan blockers | Re-open the gray areas raised by the blocker; carry forward unchanged decisions from existing CONTEXT.md | ✓ |
| View existing CONTEXT.md first | Display current decisions, then choose Update / Skip | |
| Full re-discuss from scratch | Treat as fresh phase | |

| Option | Description | Selected |
|--------|-------------|----------|
| Continue and replan after | Capture new context, then `/gsd-plan-phase 11` to regenerate 11-04..11-06 against the new D-02 disposition | ✓ |
| View existing plans first | List/skim 11-04, 11-05, 11-06 PLAN.md before deciding scope | |
| Cancel — context is fine, just replan | Skip discuss-phase entirely | |

---

## D-02 ABI fate

| Option | Description | Selected |
|--------|-------------|----------|
| Option C — generic chunks, cast at helper call sites | Chunks stay <F: Float>; translator wraps every helper call as `F::new(pow_1_3(F::cast_into(x)))`. ~581k call-site wraps. Zero math/src/ change. | |
| Option B — concrete-f64 chunk bodies, generic launch wrappers | Drop F-generic from chunk bodies; cast at launch wrapper. Loses chunk-level genericity. | |
| Option A — make 38 math helpers generic over <F: Float> | Refactor all 38 + propagate F::new() wraps internally. Manual §6 says yes. | |
| Spike first — build A and C on canary, pick after measuring | Lower-risk lock-in. ~2 days spike budget. Option B excluded from race. | ✓ |

**User's choice:** Spike first — A vs C race, B excluded.
**Notes:** Establishes D-14. The 11-01 D-02 spike was insufficient (never exercised helper calls); the new spike's pass criteria force compile + parity + idempotency together.

| Option | Description | Selected |
|--------|-------------|----------|
| Regex/textual pass in per_functional.py | Extend q01's `_wrap_f64_literals` regex | |
| AST-level pass in cse.py | F-coercion visitor in cse.py; for C wraps helper calls, for A barely changes | ✓ |
| Defer to planner | Decide once D-02 is locked | |

**User's choice:** AST-level cse.py.
**Notes:** D-16. q01's regex retires; one AST pass handles literal-wrap (Family A residual subsumed) and helper-call-wrap (under Option C) or stays minimal (under Option A).

| Option | Description | Selected |
|--------|-------------|----------|
| lda_x | Simplest baseline; ~1-2 helper calls; doesn't exercise CSE/5K cap/wide-tuple | |
| gga_x_pbe | Canonical mid-complexity; multi-output; exercises F↔f64 boundary | |
| mgga_c_b94 (deferred) | 16,703-line kxc_pol.rs stress; CSE + wide-tuple + helper-call all at once. ~20+ min compile under jobs=1. Deferred → parity needs one-shot bypass. | ✓ |
| Both lda_x AND gga_x_pbe in sequence | Two-tier spike | |

**User's choice:** mgga_c_b94 (deferred).
**Notes:** Most aggressive stress-test choice. Implication flagged for planner: parity step needs one-shot `is_deferred` bypass (NOT a permanent unfilter — D-11 stays).

| Option | Description | Selected |
|--------|-------------|----------|
| Compile + parity + lines-touched | Objective metrics; lines-touched delta as tiebreaker | |
| Compile + parity only | Simpler; lacks cost comparison | |
| Compile + parity + idempotency | Strictest gate; idempotency = D-LOCK-D anyway | ✓ |

**User's choice:** Compile + parity + idempotency.

| Option | Description | Selected |
|--------|-------------|----------|
| 1 day per option (2 days total) | Default to other option if one fails within budget; escalate to third discuss-phase if both fail | ✓ |
| Half a day per option (1 day total) | Faster; risk on the trickier option (A's br89 Brent refactor) | |
| Open-ended | No cap; risks spike consuming the phase | |

**User's choice:** 2-day time-box.

---

## Compile-first entry gate

| Option | Description | Selected |
|--------|-------------|----------|
| Same canary (mgga_c_b94) for spike + gate | Spike outcome IS gate's first deliverable | ✓ |
| Different canary — spike on mgga_c_b94, gate on gga_x_pbe | Two-stage validation | |
| Three-stage — spike → lda_x → gga_x_pbe | Strongest pre-sweep confidence | |

**User's choice:** Same canary.
**Notes:** D-15. Trades the routed-functional re-validation for simplicity; planner notes the deferred-bypass requirement for the parity leg.

| Option | Description | Selected |
|--------|-------------|----------|
| Kernel subcrate + dispatch + verify integration | All three legs green | ✓ |
| Kernel subcrate + verify integration only | Skips standalone libxc_rs dispatch build | |
| Kernel subcrate compile only | Same granularity as 11-04 Task 1A; weak — wouldn't have caught the blocker | |

**User's choice:** Full three-leg gate.

| Option | Description | Selected |
|--------|-------------|----------|
| Stop and replan — third discuss-phase pass | Each gate failure is a planner-level reset event | ✓ |
| Iterate within spike plan — N retries | Lower context-switch overhead; risk: retries compound on the wrong ABI | |
| Hybrid — retries inside spike, force replan if both A and C fail | Middle path | |

**User's choice:** Stop and replan on failure.
**Notes:** Codifies AP-6: "don't grind on the same broken approach without a planner-level reset."

---

## Translator emit surface (refinement)

| Option | Description | Selected |
|--------|-------------|----------|
| All 38 helpers in one wave + 165-error test drift fix | Single coherent change; bigger diff | ✓ |
| Tiered — powers/piecewise → dft_quantities/spin/erf → br89/bessel/lambert_w | Smaller per-commit diffs; ~3x merge surface | |
| Defer hard helpers — only refactor canary-touched | Risk: per-`-p` sweep stalls on unrefactored helpers | |

**User's choice:** All 38 in one wave (if A wins).

| Option | Description | Selected |
|--------|-------------|----------|
| cse.py AST visitor matches helper-call CallExpr | Allowlist of known helpers; F::cast_into in, F::new out | ✓ |
| Hybrid — cse.py marks, emit.py renders | Cleaner separation of concerns | |
| Allowlist vs comprehensive scan | Sub-decision | |

**User's choice:** AST visitor with allowlist (if C wins).

| Option | Description | Selected |
|--------|-------------|----------|
| Subsume into AST F-coercion visitor | One pass handles literals + helper calls | ✓ |
| Keep regex; extend to integer-mantissa + named consts | Cheaper; brittle | |
| Defer to follow-up | Risk: sweep stalls on Family A sites not on canary | |

**User's choice:** Subsume into AST visitor. q01's regex retires.

---

## Replan boundary

| Option | Description | Selected |
|--------|-------------|----------|
| Expand to 11-04..08 (5 plans) | spike → translator → regen+gate → sweep → close | ✓ |
| Repurpose 11-04..06 (3 plans) | Each plan absorbs more work; coarser SUMMARYs | |
| Hybrid — 11-04a/b/c + 11-05/06 unchanged | Sub-plans for spike/translator/gate; rest preserved | |

**User's choice:** 5-plan expansion.

| Option | Description | Selected |
|--------|-------------|----------|
| Retroactive 11-04 SUMMARY for Task 1A, then renumber | 39eb75f93 documented standalone; forward work in 11-05.. | ✓ |
| Roll Task 1A into new 11-04 as completed prereq | Cleaner forward narrative; loses traceability | |
| Leave 11-04 PLAN.md as paused-partial; new plans use 11-04.1 etc. | Most faithful; confuses gsd-tools sequential expectation | |

**User's choice:** Retroactive 11-04 SUMMARY + forward 11-05..08.

| Option | Description | Selected |
|--------|-------------|----------|
| 11-06 (translator update) — co-locate with cse.py work | Math/src/ touches in one plan whether A or C wins | ✓ |
| 11-05 (spike) — unblock spike's parity test | Grows spike scope to 2.5 days | |
| 11-08 (close) — bundle with audit + ROADMAP edits | Math/src/ test suite stays red until close | |
| Defer to parallel quick task | Parallel + jobs=1 RAM ceiling don't compose | |

**User's choice:** 11-06 absorbs the 165 from_raw_parts API drift errors.

---

## In-flight artifact disposition (multi-select)

| Item | Decision |
|------|----------|
| Commit `5c379dc25` (q01 emit fixes) | KEEP. MAX_TUPLE_ARITY=12 stays; single-output scalar return stays. The regex `_wrap_f64_literals` retires (superseded by D-16). |
| Commit `39eb75f93` (verify dev-dep narrowing per D-05) | KEEP. Documented retroactively in 11-04 SUMMARY (D-17). |
| math/src/ #[cfg(test)] from_raw_parts drift (165 errors, predates session) | ADDRESS in 11-06 (translator update plan). |
| ROADMAP success criteria edits | EDIT in 11-08 close — criterion #1 per D-10, criterion #4 per D-12, NEW criterion per D-15 (compile-first entry gate). |

---

## Process notes (2026-05-15 second pause)

- Two routing questions batched as a 2-question turn at the start. Three planning gray-areas batched as 4-question turns (Area 1: D-02 fate + emit surface; Area 1 follow-up: canary + metric + time-box; Area 2: gate canary relationship + scope + failure recovery; Area 3: A-refactor scope + C-implementation + Family A subsumption; Area 4: replan structure + 11-04 disposition + math/src/ drift slot).
- All "Recommended" options were selected by the user except: (a) D-02 fate where the user picked "Spike first" over the recommended Option C — confirming the spike-first culture established by q01; (b) gate scope where the user picked the recommended three-leg gate.
- Empirical inputs from `260515-q01` SPIKE-FINDINGS.md were treated as locked facts, not re-litigated: MAX_TUPLE_ARITY=12 cap, literal-coercion E0277, 1-tuple E0308, 38 concrete-f64 helpers, ~581k call sites.
- New decisions added: D-11 (deferred-kernel handling), D-12 (build verification not a phase gate). Revised: D-04, D-05, D-10, D-10a, D-10b, D-LOCK-A.
