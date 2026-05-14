# Phase 11: Splitter v2 — Unified Kernels with 5K Line Cap - Context

**Gathered:** 2026-05-13
**Revised:** 2026-05-14 (unification target changed to per-functional subcrates; D-10 family, D-LOCK-A, D-04, D-05 revised; D-11/D-12 added)
**Status:** Ready for planning (re-plan required — see note at end)

<domain>
## Phase Boundary

Re-engineer the Maple → CubeCL conversion pipeline (`tools/translate_{lda_v2,gga,mgga}.py` and helpers) so that two invariants hold simultaneously across every kernel emitted under `crates/kernels/`:

1. **One subcrate per functional, named by functional id** — no more `crates/kernels/{lda,gga,mgga}-N/` numbered subcrates, AND no single fat per-family crate either. Every routed functional becomes its own independently-compilable Cargo crate (`crates/kernels/gga/gga_c_acgga/`, `crates/kernels/mgga/mgga_c_b94/`, `crates/kernels/lda/lda_x/`, …). The family level (`crates/kernels/{lda,gga,mgga}/`) is a **plain directory**, not a crate — it has no `Cargo.toml` and no `lib.rs`. ~264 functional subcrates total (~41 LDA + ~131 GGA + ~92 MGGA). This is the structural fix for the `cargo check`/`cargo build` OOM: rustc never expands ~290 `#[cube]` fns in one invocation.
2. **Hard 5,000-line cap per file** — every emitted `.rs` file under the kernel subcrates is ≤5,000 lines. The splitter is extended to subdivide single output expressions (the current 8–15K floor for r4scan, br89_explicit, mgga_c_b94 kxc_pol, etc.) into `#[cube]` helper functions following the CubeCL macro fan-out manual. **The 5K cap remains HARD even though per-functional subcrates already isolate compilation units** — both invariants hold (see D-LOCK-B).

The pipeline must iterate until both invariants hold AND oracle parity is preserved at the gate locked below (D-05). **`cargo build --workspace` is NOT a gate for this phase** — it is currently OOMing, and the per-functional subcrate restructure is the fix, verified per-subcrate (see D-12).

**Not in scope:**
- Workspace boundary refactor into `libxc-core` / `libxc-eval` / `libxc-compat` — that is Phase 10 (see D-06 below for ordering).
- Adding new functionals or changing functional-level semantics.
- Promoting the f32 path to a verified-correct execution mode (see D-03 — f32 is plumbing-only on the verify side).

</domain>

<decisions>
## Implementation Decisions

### Expression-subdivision strategy
- **D-01:** CSE-aware subdivision. The splitter detects common subexpressions and multi-use temporaries in the Maple AST (or post-translation Rust AST) and lifts each into a free `#[cube]` helper. Aligns with `cubecl_macro_fanout_manual.md` §10 — "break apart meaningful algorithmic stages, not every expression-level helper." Per-statement banding and arbitrary AST-token chunking are explicitly rejected. **Still in scope** — per-functional subcrates do not relax the 5K cap (D-LOCK-B), so CSE chunking is still required to break the 8–15K single-output floor.

### Cross-file ABI for subdivided chunks
- **D-02:** Free `#[cube]` functions with explicit value args and tuple returns, generic over `<F: Float>`. Signature shape: `#[cube] fn chunk_NN<F: Float>(args: f64s as F) -> (F, F, ...)`. Each chunk's dependencies are visible in its parameter list. Helper structs with `#[cube] impl` blocks and bag-of-floats shared mutable state are explicitly rejected (per `cubecl_macro_fanout_manual.md` §9, §19, §4).
  - **STATUS: empirically proven.** Plan 11-01 (Wave 0) executed; the D-02 spike (`crates/kernels/math/tests/spike_tuple_return_cube.rs`) PASSED on cubecl-cpu — tuple returns + `<F: Float>` generic round-trip cleanly through cubecl-macros 0.10 → IR → runtime. Pre-flight gate P11-INV-A1 is GREEN. The chunk ABI for the remaining plans is locked.

### Precision policy (overrides existing CLAUDE.md "f64 only" rule for kernel chunks)
- **D-03:** Kernel chunks are generic over `<F: Float>`. **f64 is the default and the sole correctness target** — the oracle verification gate (D-05) runs at f64 only. f32 is a launch-time opt-in for performance with no correctness guarantee; chunks compile against both but f32 is not oracle-gated. This relaxes the existing `CLAUDE.md` constraint ("f64 only; no silent f32 fallback") in a controlled way: f32 is no longer a *silent* fallback — it remains an explicit launch-time choice with documented "performance-only, no correctness gate" status. The typed-error-if-device-lacks-f64 rule still applies when the user *selects* f64.
- **D-03a:** `CLAUDE.md` must be updated as part of this phase to reflect the policy shift (move "f64 only" → "f64 by default and for oracle gating; f32 opt-in at launch with no correctness gate").

### File layout within a functional subcrate (REVISED 2026-05-14)
- **D-04 (REVISED — nested by output):** Inside a functional subcrate (`crates/kernels/gga/gga_c_acgga/src/`), multi-file functionals and CSE chunks are laid out **nested by the output derivative they compute** — the convention quick task 260514-q02 converged to for `mgga_c_b94`. Example: `gga_c_acgga/src/kxc_pol/part01.rs … part04.rs`, `gga_c_acgga/src/kxc_unpol/part01.rs …`. CSE chunk helpers (D-02) live alongside the parts in the same output subdir. The subcrate `src/lib.rs` enumerates the output modules (`pub mod kxc_pol;` …) and re-exports the assembled per-output functions. **The previous flat `_partNN`-in-`src/` convention is superseded** — flat `_partNN` was the pre-revision D-04; splitter v2 standardizes on the nested layout because within an isolated subcrate namespace the output-grouped nesting is clean and matches how the splitter already splits along output boundaries.

### Verification gate (REVISED 2026-05-14)
- **D-05 (REVISED — narrowed deps + smoke per iteration):** Verify gate tolerance is unchanged: **1e-12 relative error on energy AND all routed derivatives, at f64** (matches the `CLAUDE.md` project standard). What changes is *how* it runs:
  - `verify/` dev-dependencies are **narrowed to depend on individual functional subcrates**, never the umbrella kernel crates (which no longer exist under D-10). This is the structural fix for the verify/ OOM confirmed in Wave 0 (plan 11-01 deviation D1: the umbrella `libxc-kernel-{lda,mgga}` dev-deps OOM-killed the spike before its own test compiled).
  - **Per-iteration gate = representative smoke parity** — a few functionals per family (LDA/GGA/MGGA) run at strict 1e-12 after each translation iteration. This is the load-bearing per-iteration acceptance signal.
  - **Full per-subcrate parity sweep runs at phase end**, not every iteration — running all ~264 subcrates' parity every iteration is intractable on this hardware.
  - Bit-exact f64 was rejected (CSE-aware subdivision introduces named temporaries that may legitimately reorder accumulation). Energy-only at 1e-12 with relaxed-derivative gates was rejected (would mask Phase 4-style derivative bugs).

### Phase ordering
- **D-06 [informational]:** Phase 11 lands before Phase 10 (workspace modular split). Rationale: collapsing the numbered kernel subcrates into per-functional subcrates first means Phase 10 inherits a clean, granular kernel layer rather than absorbing the current sprawl AND the workspace split simultaneously. Phase 10's ROADMAP entry already commits to `cargo tree -p libxc-eval` cleanliness — Phase 11 makes that cheaper to achieve. Risk acknowledged: Phase 11 is research-grade and slow; Phase 10 waits.

### RAM ceiling (Phase 11 operating envelope)
- **D-07:** Hard rule for ALL Phase 11 iteration runs:
  - Executor runs **inline** (no `isolation="worktree"` subagent dispatch for cargo-touching work).
  - `cargo`'s `jobs = 1` is already enforced project-wide via `.cargo/config.toml` (see D-09). Phase 11 MUST NOT relax it — neither by overriding `CARGO_BUILD_JOBS`, nor by passing `--jobs N`, nor by editing `.cargo/config.toml`.
  - This is tighter than memory `feedback_ram_constraints.md` (`jobs ≤ 2`) — but the project's actual `.cargo/config.toml` default is already `jobs = 1`. Trust `.cargo/config.toml` over memory when they disagree.
  - Read-only researcher / scout subagents are still permitted (they don't compile).

### Build environment baseline (must be preserved verbatim)
- **D-08 (RUST_MIN_STACK):** `RUST_MIN_STACK = "67108864"` (64 MB) in `.cargo/config.toml` `[env]` is load-bearing for compilation of br89/mbrxc Brent-method root-finders. The default 8 MB rustc thread stack SIGSEGVs during deep `#[cube]` proc-macro expansion. CSE-aware subdivision (D-01) introduces MORE deeply nested `#[cube]` helpers, so the 64 MB stack is even more load-bearing after Phase 11. Phase 11 MUST NOT:
  - Remove or reduce `RUST_MIN_STACK` below 64 MB.
  - Set `RUST_MIN_STACK` to the prior buggy value `2_000_000_000` (≈1.87 GiB typo fixed in quick task 260510-q01).
  - Override the env value in subagent prompts.
  Phase 11 MAY raise `RUST_MIN_STACK` further if a specific chunk-graph still SIGSEGVs after splitting — document the new value and failing chunk in the iteration's SUMMARY.md.
- **D-09 (cargo config is the source of truth):** Phase 11 reads its build environment from `.cargo/config.toml`, not from agent prompts or memory:
  - `[build] jobs = 1` — single-job builds (D-07).
  - `[build] target-dir = "/home/user/Documents/workspace/libxc_rs/.cache/cargo-target"` — out-of-tree target dir. Iteration loops MUST NOT clean this directory between iterations; incremental builds against it are the design.
  - `[env] RUST_MIN_STACK = "67108864"` (D-08).
  - sccache is in use — Phase 11 MUST NOT disable sccache or enable incremental compilation in Cargo.toml profiles (incompatible per the config header).
  Any subagent prompt listing "build commands" MUST cite `.cargo/config.toml` as the authoritative env, not duplicate the values inline.

### Locked from prior discussion (carried in from quick-task promotion)
- **D-LOCK-A (REVISED 2026-05-14):** Unification scope = collapse the 27 numbered subcrates into **one subcrate per functional, named by functional id** — NOT into per-family crates, and NOT into a single fat family crate. The family level (`crates/kernels/{lda,gga,mgga}/`) is a plain directory. Multiple files per functional are permitted; the nested-by-output layout per D-04 is the within-subcrate convention.
- **D-LOCK-B:** 5,000-line cap is HARD, not aspirational — and it stays hard under the revised per-functional-subcrate structure. The splitter is extended (D-01) until it can hit the cap on every functional, including the current 8–15K single-output leaves (r4scan, br89_explicit, kcis/kcisk lxc_pol_partNN, mgga_c_ccalda, mgga_c_rppscan, mgga_c_b94 kxc_pol, mgga_c_revtpss lxc_pol_part20).
- **D-LOCK-C:** Supersedes in-progress quick task `.planning/quick/260513-8nv-update-splitter-tool-enforce-3000-line-c`. That task targeted a 3,000-line cap and is abandoned. If it produced uncommitted artifacts, they are discarded; if it created commits, they are reviewed during planning and either kept or reverted.
- **D-LOCK-D:** Iteration is required. The pipeline must be re-run until BOTH invariants (per-functional subcrates AND 5K cap) hold AND the D-05 oracle gate passes. Idempotency is a Phase 11 success criterion: running the pipeline twice must produce no diff.

### Splitter emission target (REVISED 2026-05-14 — per-functional subcrates)
- **D-10 (REVISED — splitter emits per-functional subcrates):** `tools/translate_{lda_v2,gga,mgga}.py` and all helpers emit DIRECTLY into per-functional subcrates: `crates/kernels/{family}/<func>/src/...`. Each `<func>` directory is a complete Cargo crate (its own `Cargo.toml` depending on `libxc-kernel-math`, its own `src/lib.rs`). **The splitter never emits numbered subcrates and never emits a per-family crate.** The family directory is a plain folder. This supersedes the original D-10 ("splitter emits into one crate per family with `pub mod <func>;` modules") — the change from per-family-modules to per-functional-subcrates was made on 2026-05-14 to attack the `cargo check`/`cargo build` OOM at the compilation-unit boundary.
- **D-10a (REVISED — clean slate):** The first substantive wave deletes, upfront:
  - All 27 numbered subcrates (`lda-1/2`, `gga-1..8`, `mgga-1..7, 8a, 8b, 9a, 9b, 10, 11a, 11b, 12, 13, 14`).
  - The three family façade crates as crates (`crates/kernels/{lda,gga,mgga}/Cargo.toml` + `lib.rs`) — the directories survive as plain folders.
  - All `libxc-kernel-{lda,lda-1,lda-2,gga,mgga,mgga-1..14}` path-deps from root `Cargo.toml` `[dependencies]`.
  - The numbered-subcrate entries in root `Cargo.toml` `[workspace] default-members`.
  The splitter then populates the ~264 per-functional subcrates from scratch via **clean-slate regen from Maple** — q01's mgga-2 split and q02's mgga_c_b94 nesting output are discarded; the splitter must be good enough to reproduce ≤5K (D-LOCK-D idempotency demands it reproduce everything anyway). There is NO state where both layouts coexist. Risk acknowledged: mid-flight regen failure leaves a half-broken tree until rollback (mitigated by per-wave atomic commits, per-subcrate `cargo build -p <subcrate>` verification, and `git checkout -- .` rollback).
- **D-10b (REVISED — dispatch imports per-functional subcrates):** The dispatch tree is regenerated to resolve against per-functional subcrates. The `src/kernel/{family}/` façade module tree — today `crate::kernel::mgga::batchN::<func>::...` — is regenerated so no `batchN::` segment and no numbered-subcrate segment survives; paths become `crate::kernel::{family}::<func>::...` re-exporting each functional subcrate (or dispatch imports `libxc_kernel_<func>::...` directly). RESEARCH.md "Strategy 1 (façade preserves dispatch paths)" remains REJECTED. The dispatch generator (`tools/generate_gga_dispatch.py` + MGGA equivalent) and its input rosters (`.planning/phases/04-bulk-kernel-translation/{gga,mgga}_roster.tsv`) are updated: the `batch` column is dropped or degenerate. **Recommended:** drop the per-batch submodule layer entirely; emit one `dispatch_<func>` helper per functional under `src/eval/{family}_dispatch/funcs/<func>.rs`. Planner's call on the exact façade shape.

### Deferred-kernel handling (NEW 2026-05-14)
- **D-11:** The 6 deferred kernels (`mgga_c_b94`, `mgga_x_br89`, `mgga_x_mbr`, `mgga_x_mbrxc_bg`, `mgga_x_mbrxh_bg`, `mgga_x_mggac`) AND the unrouted `mgga_x_br89_explicit` are each normal per-functional subcrates, but **omitted from root `Cargo.toml` `[workspace] default-members`**. `cargo build` / `cargo check` skip them by default; `cargo build -p libxc-kernel-<func>` still builds them on demand. This supersedes quick task 260514-q02's feature-gate approach (which was committed then reverted — see git `1eec03e2` → `59b11dcd`). The per-functional subcrate structure makes `default-members` exclusion the natural mechanism: no cargo feature, no `#[cfg]` attribute. Runtime routing is unchanged — `is_deferred(id)` still rejects these IDs in `MggaFunctional::from_id` and the oracle harness still SKIPs them.

### Build verification is not a phase gate (NEW 2026-05-14)
- **D-12:** `cargo build --workspace` is **currently OOMing and is NOT a gate** for the restructure work. The per-functional subcrate isolation (D-10) IS the structural fix for the OOM. During iteration, build verification is **per-subcrate** (`cargo build -p libxc-kernel-<func>`) and incremental against the cached target dir (D-09). A whole-workspace build is not expected to pass until enough subcrates are populated and the dispatch tree is regenerated; even then it is verified incrementally, not as a single-invocation gate. The original ROADMAP success criterion #4 (`cargo build --workspace` succeeds) is reinterpreted accordingly — see the re-plan note at the end of this file.

### Claude's Discretion
- **Subcrate package naming.** Recommended: follow the existing `libxc-kernel-*` convention — package `libxc-kernel-<func>` (e.g. `libxc-kernel-gga_c_acgga`), lib name `libxc_kernel_<func>`. Planner confirms the exact spelling (hyphen vs underscore handling in the package name) after reading the current numbered-subcrate `Cargo.toml` naming.
- Internal structure of the CSE pass (Maple AST walker vs post-translation Rust AST walker vs Python-side intermediate IR). The decision is "CSE-aware" — implementation surface is left to the planner + phase researcher.
- Whether to extend the existing `tools/translate_*.py` family in place or fork a `tools/translate_v2/` tree. Planner's call after reading the current splitter implementation.
- How the splitter generates ~264 per-subcrate `Cargo.toml` files and rebuilds the root `[workspace] default-members` array (template vs programmatic). Planner's call.
- Whether to add a `tools/audit_kernel_size.py` CI gate — already partially built in Wave 0 (`tools/audit_kernel_size.py` exists per 11-01-SUMMARY). Planner decides whether to extend it for the subcrate-count invariant too.
- Whether to retain the existing `tools/split_oversized_{kernel,mgga}.py` / `tools/rebatch_mgga.py` / `tools/split_mgga_7_kcis.py` helpers as scaffolding or delete them (they assume the numbered-subcrate layout, now obsolete). Planner's call.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### CubeCL design constraints (load-bearing)
- `docs/manual/Cubecl/cubecl_macro_fanout_manual.md` — THE authoritative reference for how to subdivide kernels under CubeCL. Key sections: §3 ("Keep the CubeCL expansion surface as small as possible"), §6 (Prefer Generic Numeric Kernels — supports D-03 generic `<F: Float>`), §10 ("Break apart meaningful algorithmic stages, NOT every expression-level helper" — supports D-01 CSE-aware over per-statement), §13 (Reduce Element-Type Generic Explosion — caveats D-03), §19 (Recommended low-fan-out architecture), §21 (Refactoring Checklist). Read end-to-end before planning.

### Wave 0 artifacts (plan 11-01 — already executed)
- `.planning/phases/11-splitter-v2-unified-5k-cap/11-01-SUMMARY.md` — Wave 0 results. D-02 spike PASSED; audit tools committed; dispatch staleness (Blocker B1) documented. Read for the deviation list (D1–D7) — D1 (verify/ OOM) directly motivates the D-05 revision.
- `.planning/phases/11-splitter-v2-unified-5k-cap/11-BASELINE.md` — pre-phase metrics snapshot (235 oversized files, max 16703, 27 numbered subcrates, 35 workspace members).
- `.planning/phases/11-splitter-v2-unified-5k-cap/11-DISPATCH-AUDIT.md` — Blocker B1: dispatch tree references stale batch IDs (`batch15..22` GGA, `batch17..35` MGGA). Phase 11 collapse must regenerate dispatch.
- `tools/audit_kernel_size.py`, `tools/audit_subcrate_collapse.sh`, `tools/audit_cube_launch.sh`, `tools/test_idempotency.sh`, `tools/audit_dispatch_tree.sh` — Wave 0 audit tooling. Note: `audit_subcrate_collapse.sh` must be updated for the per-functional-subcrate invariant (it currently checks for absence of numbered subcrates; the new invariant also requires absence of per-family crates).
- `crates/kernels/math/tests/spike_tuple_return_cube.rs` — the proven D-02 ABI spike. Reference shape for chunk signatures.
- `verify/tests/parity_phase11.rs` — parity harness scaffold (smoke + worst-case test sets).

### Current splitter implementation (the thing being re-engineered)
- `tools/translate_lda_v2.py` — current LDA translator. `SPLIT_THRESHOLD = 6000` (~line 362).
- `tools/translate_gga.py` — current GGA translator. `SPLIT_THRESHOLD = 6000` (~line 483).
- `tools/translate_mgga.py` — current MGGA translator. `SPLIT_THRESHOLD = 6000` (~line 553).
- `tools/maple_to_kernels.py` — unified driver. `DEFAULT_SPLIT_THRESHOLD = 100_000` / `DEFAULT_TARGET_MAX = 500_000` — defaults need re-tuning for the 5K hard cap.
- `tools/split_oversized_kernel.py`, `tools/split_oversized_mgga.py`, `tools/split_mgga_7_kcis.py`, `tools/rebatch_mgga.py` — post-split sub-crate helpers assuming the numbered-subcrate layout; now obsolete (D-10a).
- `tools/batch_translate_{lda,gga,mgga}.py` — batch drivers.
- `tools/split_lda_subcrates.py`, `tools/audit_deferred_gga.py`, `tools/demote_deferred_lda_fanout.py`, `tools/demote_unrouted_kernels.py` — supporting helpers.
- `tools/generate_gga_dispatch.py` (+ MGGA equivalent if present) — dispatch generators, updated per D-10b.
- `tools/translators/` directory — exists but appears empty; investigate during planning.

### Maple source
- `libxc-master/maple/` — 48 Maple input files (`gga_exc`, `gga_vxc`, top-level `.mpl` files). The splitter's input.

### Current kernel layout (the thing being collapsed)
- `crates/kernels/lda/`, `crates/kernels/gga/`, `crates/kernels/mgga/` — family-level façade crates. Under D-10a these lose their `Cargo.toml`/`lib.rs` and become plain directories holding per-functional subcrates.
- `crates/kernels/lda-{1,2}/`, `crates/kernels/gga-{1..8}/`, `crates/kernels/mgga-{1..14, 8a, 8b, 9a, 9b, 11a, 11b}/` — 27 numbered subcrates, deleted by D-10a.
- `crates/kernels/math/` — shared math primitives. **Stays a crate** — out of scope for the collapse; every per-functional subcrate depends on it.
- `crates/kernels/shared/` (under `src/kernel/shared/`) — shared kernel utilities.
- `Cargo.toml` root — `[dependencies]` and `[workspace] default-members` both rewritten by D-10a; `default-members` becomes the ~264 per-functional subcrate list.

### Recent quick tasks touching this area (post-CONTEXT, must read)
- `.planning/quick/260514-q01-split-mgga-2-large-kernels/` — q01 split mgga-2 and `mgga_c_ccalda` to ≤5K files (commit `0506d0e5`). Output is **discarded** by the D-10a clean-slate regen, but the SUMMARY documents which functionals were the hardest and `cargo check -p libxc-kernel-mgga-2` cost (22m 11s).
- `.planning/quick/260514-q02-evaluate-mgga2-memory-peaks/260514-q02-DESIGN-MEMO.md` — q02's analysis of the three memory-peak surfaces (build-time / runtime / test-run). Its Option A (feature-gate deferred kernels) was committed (`1eec03e2`) then reverted (`59b11dcd`) in favor of the `kxc_pol/`+`kxc_unpol/` nesting (`504d8560`). The nesting motivated D-04's revision; the deferred-kernel analysis motivated D-11. **The feature-gate approach is superseded by D-11.**

### Existing project policy that this phase touches
- `CLAUDE.md` — § "Constraints": "f64 only; no silent f32 fallback" AND "Maple2c formula translations must preserve floating-point operation order for bit-level equivalence". Phase 11 amends both (D-03, D-05). The amendment must land in this phase's executor commits.
- `.cargo/config.toml` — **load-bearing build environment**. `[build] jobs = 1`, `target-dir = .cache/cargo-target`, `[env] RUST_MIN_STACK = 67108864`. See D-07/D-08/D-09.

### Project memory references (must read before planning)
- `~/.claude/projects/-home-user-Documents-workspace-libxc-rs/memory/project_splitter_algorithm_floor.md` — "Splitter bottoms out at one output component; 8–15K-line single-output leaves unavoidable today." D-01 explicitly attacks this floor.
- `~/.claude/projects/-home-user-Documents-workspace-libxc-rs/memory/project_split_threshold_history.md` — "SPLIT_THRESHOLD history 5K→18K→50K→100K→6K; lda-2 OOM'd at 100K; don't go below 4500 without recalibrating." Phase 11 targets 5K — at the edge of the historical OOM zone.
- `~/.claude/projects/-home-user-Documents-workspace-libxc-rs/memory/feedback_ram_constraints.md` — "inline sequential, jobs≤2". D-07 tightens to jobs=1 (trust `.cargo/config.toml`).
- `~/.claude/projects/-home-user-Documents-workspace-libxc-rs/memory/feedback_splitting_terminology.md` — "decrease splitting criteria ⇒ FEWER files ⇒ RAISE threshold". Phase 11 inverts: the goal IS more files, smaller files.
- `~/.claude/projects/-home-user-Documents-workspace-libxc-rs/memory/feedback_kernel_build_failure.md` — "kernel build/test failure → refactor per cubecl_macro_fanout_manual." Directly applicable.
- `~/.claude/projects/-home-user-Documents-workspace-libxc-rs/memory/feedback_verify_crate_oom.md` — "verify/ test runs OOM via libxc-kernel-{lda,mgga} dev-deps." D-05's narrowed-deps revision is the structural fix.

### Superseded work
- `.planning/quick/260513-8nv-update-splitter-tool-enforce-3000-line-c/` — superseded by this phase (D-LOCK-C).

### Phase-adjacent ROADMAP context
- `.planning/ROADMAP.md` § Phase 11 — success criteria #1 and #4 need rewording for the per-functional-subcrate target and the D-12 build-gate reinterpretation (see re-plan note). § Phase 10 ("Workspace-Level Modular Split") — Phase 11 sequences BEFORE Phase 10 (D-06).

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `tools/maple_to_kernels.py` — unified driver pattern (commit 37820e2d). Provides translate+split orchestration with knob-style configuration. Splitter v2 can extend this.
- `tools/translate_lda_v2.py`, `tools/translate_gga.py`, `tools/translate_mgga.py` — the three translator families. All share the `SPLIT_THRESHOLD` constant pattern and accept it via argv override.
- Wave 0 audit tooling (`tools/audit_*.py|sh`, `tools/test_idempotency.sh`) — already committed (commit `c181b469`). Extend rather than rebuild; note `audit_subcrate_collapse.sh` needs the per-functional-subcrate invariant added.
- `crates/kernels/math/tests/spike_tuple_return_cube.rs` — proven D-02 chunk ABI; reference shape.
- The numbered subcrates' existing `Cargo.toml` files — templates for the ~264 per-functional subcrate `Cargo.toml` files (same `[dependencies]` shape: `libxc-kernel-math`, `cubecl`, `bytemuck`, `libm`).

### Established Patterns
- **Quick-task pipeline iteration:** Phase 9 quick tasks (q01–q08) and the 260514 q01/q02 tasks show the project's iteration style — adjust, regenerate, audit, repeat. Phase 11 follows this with the splitter implementation itself as the moving target.
- **OOM mitigation precedent:** Quick tasks 260510-q01, 260512-q02, 260514-q01/q02 show the project has repeatedly fought the OOM cliff. Per-functional subcrates (D-10) are the structural answer the prior point-fixes were approximating.
- **Routing-aware emission:** Quick task 260512-q01 added routing-aware translator emission (`#[cube]` vs `#[cube(launch)]`). Splitter v2 chunks (D-02) are `#[cube]`, never `#[cube(launch)]` (manual §4, §19).
- **Nested-by-output layout precedent:** q02's `mgga_c_b94/kxc_pol/` + `kxc_unpol/` refactor (commit `504d8560`) is the concrete pattern D-04 standardizes on.

### Integration Points
- `Cargo.toml` root `[dependencies]` + `[workspace] default-members` — both rewritten (D-10a). `default-members` becomes the ~264-entry per-functional subcrate list with deferred kernels (D-11) omitted.
- `src/kernel/{lda,gga,mgga}/` façade module tree + `src/eval/{gga,mgga}_dispatch/` — regenerated against per-functional subcrates (D-10b). Currently references stale `batchN::` paths (Blocker B1).
- `verify/` — dev-deps narrowed to per-functional subcrates (D-05); `verify/tests/parity_phase11.rs` is the harness.
- `xtask` codegen flow — confirm during planning whether xtask interacts with the splitter or the registry-vs-subcrate naming.

</code_context>

<specifics>
## Specific Ideas

- The user explicitly referenced `cubecl_macro_fanout_manual.md` in the originating task. When a planning question is between two readings of the manual, the manual wins.
- The 2026-05-14 restructure decision: subcrates named by **functional id** (`gga_c_acgga`, `gga_c_gapc`), not by family number. Crate structure is `kernels/ > {lda,gga,mgga}/ > <func>/` where the family level is a plain directory and `<func>/` is the actual crate. One subcrate per functional, all ~264.
- The 5K cap is still HARD under the new structure — the user confirmed it when asked whether per-functional subcrates relax it. Per-functional subcrates fix the OOM; the 5K cap fixes the per-file proc-macro fan-out. Both are required.
- The 8–15K examples to attack (concrete evidence captured during discuss): `mgga_c_b94/kxc_pol.rs` (16,703 lines), `mgga_c_kcisk/lxc_pol_part15.rs` (16,138), `mgga_c_ccalda/lxc_pol.rs` (15,378), `mgga_c_kcis/lxc_pol_part13.rs` (14,127), `mgga_c_kcisk/lxc_pol_part{14,16}.rs` (13,719–13,913), `mgga_c_rppscan/lxc_pol.rs` (13,238), `mgga_c_revtpss/lxc_pol_part20.rs` (12,648). Splitter v2 must hit ≤5K on every one.
- `cargo build --workspace` is expected to FAIL with OOM right now — the user explicitly said no build verification is required at this stage (D-12).

</specifics>

<deferred>
## Deferred Ideas

- **f32 oracle gate at relaxed tolerance.** Rejected for now (D-03); a natural future capability once the f64 path is stable.
- **CI gate enforcing the 5K cap + subcrate-count invariant.** `tools/audit_kernel_size.py` exists from Wave 0; wiring it (and `audit_subcrate_collapse.sh`) into CI is a natural follow-up but not strictly required for phase completion.
- **Workspace boundary refactor.** Phase 10 — sequenced AFTER Phase 11 (D-06).
- **Promoting `#[cube]` traits in kernel chunks.** Rejected at D-02 (manual §9). A future phase finding a true trait-shaped abstraction is its own decision.
- **Bessel I0/I1 implementation for `mgga_x_2d_prp10`** — pre-existing deferral (libxc id 211) from quick task 260510-q02. Phase 11 must not regress this deferral.
- **Option C from q02's DESIGN-MEMO (gating verify's kernel dev-deps generally)** — partially absorbed into D-05 (verify deps narrowed to per-functional subcrates). Any further verify/ restructuring beyond that is a follow-up.
- **Option E from q02's DESIGN-MEMO (`comptime!`-gated all-orders kernel collapsing exc/vxc/fxc/kxc/lxc)** — explicitly conflicts with D-02's chunking ABI; rejected for this phase, possible future order-of-magnitude play once the f64 path is stable.

</deferred>

---

## Re-plan Note (2026-05-14)

The 2026-05-14 revisions change the **unification target itself** (per-family crates → per-functional subcrates), which is the structural spine of plans 11-02..06. Those plans were written on 2026-05-13 against the old D-10. **Plans 11-02..06 are stale and must be regenerated** via `/gsd-plan-phase 11`. Plan 11-01 (Wave 0) stays — it is already executed and its deliverables (D-02 spike, audit tools, baseline, dispatch audit) survive the restructure, though `audit_subcrate_collapse.sh` will need an invariant update during replanning.

The planner must also flag, for ROADMAP correction:
- Success criterion #1 — "only family-level crates" → "only per-functional subcrates; family level is a plain directory; no numbered subcrates".
- Success criterion #4 — `cargo build --workspace` succeeds → reinterpreted per D-12 (per-subcrate incremental verification; whole-workspace build is not a per-iteration gate).
- Success criterion #7 — CubeCL macro fan-out audit still applies.

---

*Phase: 11-splitter-v2-unified-5k-cap*
*Context gathered: 2026-05-13 · Revised: 2026-05-14*
