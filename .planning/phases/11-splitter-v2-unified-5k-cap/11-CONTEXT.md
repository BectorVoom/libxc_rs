# Phase 11: Splitter v2 — Unified Kernels with 5K Line Cap - Context

**Gathered:** 2026-05-13
**Status:** Ready for planning

<domain>
## Phase Boundary

Re-engineer the Maple → CubeCL conversion pipeline (`tools/translate_{lda_v2,gga,mgga}.py` and helpers) so that two invariants hold simultaneously across every kernel emitted under `crates/kernels/`:

1. **Per-family subcrates collapsed** — no more `crates/kernels/{lda,gga,mgga}-N/` numbered children. One kernel-family crate per family (`crates/kernels/{lda,gga,mgga}/`).
2. **Hard 5,000-line cap per file** — every emitted `.rs` file under the kernel crates is ≤5,000 lines. The splitter is extended to subdivide single output expressions (the current 8–15K floor for r4scan, br89_explicit, mgga_c_b94 kxc_pol, etc.) into `#[cube]` helper functions following the CubeCL macro fan-out manual.

The pipeline must iterate until both invariants hold AND `cargo build --workspace` passes on the user's RAM-constrained machine AND oracle parity is preserved at the gate locked below.

**Not in scope:**
- Workspace boundary refactor into `libxc-core` / `libxc-eval` / `libxc-compat` — that is Phase 10 (see D-06 below for ordering).
- Adding new functionals or changing functional-level semantics.
- Promoting the f32 path to a verified-correct execution mode (see D-03 — f32 is plumbing-only on the verify side).

</domain>

<decisions>
## Implementation Decisions

### Expression-subdivision strategy
- **D-01:** CSE-aware subdivision. The splitter detects common subexpressions and multi-use temporaries in the Maple AST (or post-translation Rust AST) and lifts each into a free `#[cube]` helper. Aligns with `cubecl_macro_fanout_manual.md` §10 — "break apart meaningful algorithmic stages, not every expression-level helper." Per-statement banding and arbitrary AST-token chunking are explicitly rejected.

### Cross-file ABI for subdivided chunks
- **D-02:** Free `#[cube]` functions with explicit value args and tuple returns, generic over `<F: Float>`. Signature shape: `#[cube] fn chunk_NN<F: Float>(args: f64s as F) -> (F, F, ...)`. Each chunk's dependencies are visible in its parameter list. Helper structs with `#[cube] impl` blocks and bag-of-floats shared mutable state are explicitly rejected (per `cubecl_macro_fanout_manual.md` §9, §19, §4).

### Precision policy (overrides existing CLAUDE.md "f64 only" rule for kernel chunks)
- **D-03:** Kernel chunks are generic over `<F: Float>`. **f64 is the default and the sole correctness target** — the oracle verification gate (D-04) runs at f64 only. f32 is a launch-time opt-in for performance with no correctness guarantee; chunks compile against both but f32 is not oracle-gated. This relaxes the existing `CLAUDE.md` constraint ("f64 only; no silent f32 fallback") in a controlled way: f32 is no longer a *silent* fallback — it remains an explicit launch-time choice with documented "performance-only, no correctness gate" status. The typed-error-if-device-lacks-f64 rule still applies when the user *selects* f64.
- **D-03a:** `CLAUDE.md` must be updated as part of this phase to reflect the policy shift (move "f64 only" → "f64 by default and for oracle gating; f32 opt-in at launch with no correctness gate").

### File layout when one functional spans N files
- **D-04 (file layout):** Continue the existing `_partNN` suffix convention. r4scan's `lxc_pol` split across 4 files becomes `crates/kernels/mgga/src/mgga_c_r4scan/lxc_pol_part01.rs` … `_part04.rs`. The functional entry stub (`lxc_pol.rs` or `mod.rs`) re-exports the assembled function and dispatches into the parts. Already grep-friendly — the current splitter already emits `_part13`, `_part14`, etc., so this is continuity, not a new convention.

### Verification gate
- **D-05 (verify gate):** 1e-12 relative error on energy AND all routed derivatives, at f64. Matches the existing project standard (`CLAUDE.md` "energy relative error <= 10^-12"). The `verify/` regression sweep on representative LDA/GGA/MGGA functionals runs after every translation iteration. Bit-exact f64 was rejected as likely impossible — CSE-aware subdivision introduces named temporaries (sequence points) that may legitimately reorder accumulation. Energy-only at 1e-12 with relaxed-derivative gates was rejected — would have masked Phase 4-style derivative bugs.

### Phase ordering
- **D-06:** Phase 11 lands before Phase 10 (workspace modular split). Rationale: collapsing 22 numbered kernel subcrates into 3 family crates first means Phase 10 inherits a clean kernel layer, rather than Phase 10 having to absorb the current sprawl AND the workspace split simultaneously. Phase 10's ROADMAP entry already commits to `cargo tree -p libxc-eval` cleanliness — Phase 11 makes that cheaper to achieve, not harder. Risk acknowledged: Phase 11 is research-grade and slow; Phase 10 waits.

### RAM ceiling (Phase 11 operating envelope)
- **D-07:** Hard rule for ALL Phase 11 iteration runs:
  - Executor runs **inline** (no `isolation="worktree"` subagent dispatch for cargo-touching work).
  - `CARGO_BUILD_JOBS=1` exported in the orchestrator's environment AND in every subagent prompt that may compile.
  - This is **tighter** than the existing memory `feedback_ram_constraints.md` which says `jobs ≤ 2`. Phase 11 specifically tightens to `jobs=1` because the splitter iteration loop touches the macro-heaviest crates and previously OOM'd at higher concurrency. Decision is scoped to Phase 11; other phases keep the `≤ 2` rule.
  - Read-only researcher / scout subagents are still permitted (they don't compile).

### Locked from prior discussion (carried in from quick-task promotion)
- **D-LOCK-A:** Unification scope = collapse per-family subcrates ONLY. Multiple files per functional are permitted; `_partNN` per D-04 is the convention. (Re-stating from pre-discuss context for downstream agents.)
- **D-LOCK-B:** 5,000-line cap is HARD, not aspirational. The splitter is extended (D-01) until it can hit the cap on every functional, including the current 8–15K single-output leaves (r4scan, br89_explicit, kcis/kcisk lxc_pol_partNN, mgga_c_ccalda, mgga_c_rppscan, mgga_c_b94 kxc_pol, mgga_c_revtpss lxc_pol_part20).
- **D-LOCK-C:** Supersedes in-progress quick task `.planning/quick/260513-8nv-update-splitter-tool-enforce-3000-line-c`. That task targeted a 3,000-line cap and is abandoned. If it produced uncommitted artifacts, they are discarded; if it created commits, they are reviewed during planning and either kept (compatible foundation) or reverted (incompatible).
- **D-LOCK-D:** Iteration is required. The pipeline must be re-run until BOTH invariants (collapsed subcrates AND 5K cap) hold AND `cargo build --workspace` passes AND the D-05 oracle gate passes. Idempotency is a Phase 11 success criterion: running the pipeline twice must produce no diff.

### Claude's Discretion
- Internal structure of the CSE pass (Maple AST walker vs post-translation Rust AST walker vs Python-side intermediate IR). The decision is "CSE-aware" — implementation surface is left to the planner + phase researcher.
- Whether to extend the existing `tools/translate_*.py` family in place or fork a `tools/translate_v2/` tree. Planner's call after reading the current splitter implementation.
- Exact migration path for the existing 22 numbered subcrates (in-place rename + content merge vs new tree + cutover). Planner's call.
- Whether to add a `tools/audit_kernel_size.py` that fails CI when a kernel file exceeds 5K. Recommended but not locked.
- Whether to retain the existing `tools/split_oversized_{kernel,mgga}.py` / `tools/rebatch_mgga.py` / `tools/split_mgga_7_kcis.py` helpers as scaffolding or fold them into the unified pipeline. Planner's call.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### CubeCL design constraints (load-bearing)
- `docs/manual/Cubecl/cubecl_macro_fanout_manual.md` — THE authoritative reference for how to subdivide kernels under CubeCL. Key sections: §3 ("Keep the CubeCL expansion surface as small as possible"), §6 (Prefer Generic Numeric Kernels — supports D-03 generic `<F: Float>`), §10 ("Break apart meaningful algorithmic stages, NOT every expression-level helper" — supports D-01 CSE-aware over per-statement), §13 (Reduce Element-Type Generic Explosion — caveats D-03), §19 (Recommended low-fan-out architecture), §21 (Refactoring Checklist). Read end-to-end before planning.

### Current splitter implementation (the thing being re-engineered)
- `tools/translate_lda_v2.py` — current LDA translator. `SPLIT_THRESHOLD = 6000` at line 362; sole authoritative knob.
- `tools/translate_gga.py` — current GGA translator. `SPLIT_THRESHOLD = 6000` at line 483.
- `tools/translate_mgga.py` — current MGGA translator. `SPLIT_THRESHOLD = 6000` at line 553.
- `tools/maple_to_kernels.py` — unified driver. `DEFAULT_SPLIT_THRESHOLD = 100_000` / `DEFAULT_TARGET_MAX = 500_000` (lines 89-90) — these defaults will need re-tuning for the 5K hard cap.
- `tools/split_oversized_kernel.py`, `tools/split_oversized_mgga.py`, `tools/split_mgga_7_kcis.py`, `tools/rebatch_mgga.py` — post-split sub-crate helpers. `TARGET_MAX = 500_000` lines per sub-crate (irrelevant after D-LOCK-A subcrate collapse).
- `tools/batch_translate_{lda,gga,mgga}.py` — batch drivers.
- `tools/split_lda_subcrates.py`, `tools/audit_deferred_gga.py`, `tools/demote_deferred_lda_fanout.py`, `tools/demote_unrouted_kernels.py` — supporting helpers.
- `tools/translators/` directory — directory exists but appears empty; investigate during planning.

### Maple source
- `libxc-master/maple/` — 48 Maple input files. Includes `gga_exc`, `gga_vxc`, top-level `.mpl` files (attenuation, b97, b97mv, gvt4, ...) — the splitter's input.

### Current kernel layout (the thing being collapsed)
- `crates/kernels/lda/`, `crates/kernels/gga/`, `crates/kernels/mgga/` — family-level façade crates (small `lib.rs` re-exports). Target end-state layout.
- `crates/kernels/lda-{1,2}/`, `crates/kernels/gga-8/`, `crates/kernels/mgga-{1..14, 8a, 8b, 9a, 9b, 11a, 11b}/` — 22 numbered subcrates to be collapsed.
- `crates/kernels/math/` — shared math primitives (out of scope for subcrate collapse).
- `crates/kernels/shared/` (under `src/kernel/shared/`) — shared kernel utilities.

### Existing project policy that this phase touches
- `CLAUDE.md` — § "Constraints": specifies "f64 only; no silent f32 fallback; typed error if device lacks f64 support" AND "Maple2c formula translations must preserve floating-point operation order for bit-level equivalence". Phase 11 amends both (D-03, D-05). The amendment must land in this phase's executor commits, not as a separate doc change.

### Project memory references (must read before planning)
- `~/.claude/projects/-home-user-Documents-workspace-libxc-rs/memory/project_splitter_algorithm_floor.md` — "Splitter algorithm bottoms out at one output component; 8–15K-line single-output leaves are unavoidable today." Phase 11 D-01 explicitly attacks this floor.
- `~/.claude/projects/-home-user-Documents-workspace-libxc-rs/memory/project_split_threshold_history.md` — "SPLIT_THRESHOLD history 5K→18K→50K→100K→6K; lda-2 OOM'd at 100K; don't go below 4500 without recalibrating." Phase 11 targets 5K, which is at the edge of the historical OOM zone — recalibration is part of the phase.
- `~/.claude/projects/-home-user-Documents-workspace-libxc-rs/memory/feedback_ram_constraints.md` — "inline sequential, jobs≤2". Phase 11 D-07 tightens further to jobs=1.
- `~/.claude/projects/-home-user-Documents-workspace-libxc-rs/memory/feedback_splitting_terminology.md` — "decrease splitting criteria ⇒ FEWER files ⇒ RAISE SPLIT_THRESHOLD/TARGET_MAX". Phase 11 inverts this: the goal IS more files, smaller files. Confirm directional intent before editing thresholds.
- `~/.claude/projects/-home-user-Documents-workspace-libxc-rs/memory/feedback_kernel_build_failure.md` — "kernel build/test failure → refactor per cubecl_macro_fanout_manual." Directly applicable.

### Superseded work
- `.planning/quick/260513-8nv-update-splitter-tool-enforce-3000-line-c/` — superseded by this phase (D-LOCK-C). Review any commits referencing `260513-8nv` during planning; revert if incompatible.

### Phase-adjacent ROADMAP context
- `.planning/ROADMAP.md` § Phase 10 ("Workspace-Level Modular Split") — Phase 11 sequences BEFORE Phase 10 (D-06). Phase 10's success criteria (clean `cargo tree`) are easier to hit after Phase 11.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `tools/maple_to_kernels.py` — unified driver pattern (commit 37820e2d, quick task 260509-q03). Provides translate+split orchestration with knob-style configuration. Splitter v2 can extend this rather than starting from scratch.
- `tools/translate_lda_v2.py`, `tools/translate_gga.py`, `tools/translate_mgga.py` — the three translator families. All share the `SPLIT_THRESHOLD` constant pattern and accept it via argv override. Extension surface is well-defined.
- `crates/kernels/{lda,gga,mgga}/src/lib.rs` — family façade crates already exist (lda: 54 lines, gga: 19 lines, mgga: 30 lines). They currently re-export from numbered children. Collapse target is to fold child content into these crates' `src/`.
- `tools/audit_deferred_gga.py` and the pattern of post-emit audit scripts — Splitter v2 should add a `tools/audit_kernel_size.py` (Claude's Discretion in D) following this pattern.

### Established Patterns
- **Quick-task pipeline iteration:** Phases 9 quick tasks (q01–q08, q01-investigate-kernel-oom) show the project's iteration style — adjust thresholds, regenerate, audit, repeat. Phase 11 will follow this style but with the splitter implementation itself as the moving target, not just the thresholds.
- **OOM mitigation precedent:** Quick task 260510-q01 ("Investigate cargo build OOM root cause: RUST_MIN_STACK 1.87 GiB → 64 MiB typo") and 260512-q02 (mgga-14 OOM unblock at 21,679 → 5,352 lines) show the project has fought this OOM cliff before. Phase 11 must not regress.
- **Routing-aware emission:** Quick task 260512-q01 added routing-aware translator emission (`#[cube]` vs `#[cube(launch)]` based on dispatch). Splitter v2 chunks (D-02) follow this — chunks are `#[cube]`, never `#[cube(launch)]`. Per cubecl_macro_fanout_manual §4 and §19.

### Integration Points
- `Cargo.toml` workspace `members` list — must be updated when the 22 numbered subcrates collapse into 3 family crates.
- `crates/eval/gga_dispatch/`, `crates/eval/mgga_dispatch/` (and the LDA analog) — these dispatch trees currently import from `libxc_kernel_{lda,gga,mgga}_NN` numbered crates. They must be retargeted to import from the unified family crates.
- `verify/` — the regression sweep harness. D-05 says it gates each iteration; the harness must still work after dispatch retargeting.
- The `xtask` codegen flow (referenced in Phase 10's pre-planning blockers) — touches kernel emission. Confirm during planning whether xtask interacts with the splitter.

</code_context>

<specifics>
## Specific Ideas

- The user explicitly referenced `cubecl_macro_fanout_manual.md` in the originating task. That manual is the source of truth for the chunk-shape decisions (D-01, D-02, D-03) — when a planning question is between two readings of the manual, the manual wins, not Claude's discretion.
- The "no `mgga-1`/`mgga-2` numbered parts" phrasing in the original request means PARENT subcrates, not per-functional `_partNN` files. The user confirmed `_partNN` continues (D-04).
- The 8–15K examples to attack (concrete evidence captured during discuss): `mgga_c_b94/kxc_pol.rs` (16,703 lines), `mgga_c_kcisk/lxc_pol_part15.rs` (16,138), `mgga_c_ccalda/lxc_pol.rs` (15,378), `mgga_c_kcis/lxc_pol_part13.rs` (14,127), `mgga_c_kcisk/lxc_pol_part{14,16}.rs` (13,719–13,913), `mgga_c_rppscan/lxc_pol.rs` (13,238), `mgga_c_revtpss/lxc_pol_part20.rs` (12,648). The splitter v2 must hit ≤5K on every one of these.

</specifics>

<deferred>
## Deferred Ideas

- **f32 oracle gate at relaxed tolerance.** The user's Q1-precision answer rejected this for now (D-03), but it's a natural future capability once the f64 path is stable.
- **CI gate enforcing the 5K cap.** A `tools/audit_kernel_size.py` that fails CI when any kernel file exceeds 5K is a natural complement to this phase but is not strictly required for phase completion. Belongs in a follow-up phase or a quick task post-Phase-11.
- **Workspace boundary refactor.** Phase 10 — sequenced AFTER Phase 11 (D-06). Not in scope here.
- **Promoting `#[cube]` traits in kernel chunks.** Discussed and rejected at D-02 (manual §9 warns). If a future phase finds a true trait-shaped abstraction in the chunks, that's its own decision.
- **Bessel I0/I1 implementation for `mgga_x_2d_prp10`** — pre-existing deferral (libxc id 211) from quick task 260510-q02. Phase 11 must not regress this deferral.

</deferred>

---

*Phase: 11-splitter-v2-unified-5k-cap*
*Context gathered: 2026-05-13*
