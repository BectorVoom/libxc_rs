# Phase 11: Splitter v2 — Unified Kernels with 5K Line Cap - Context

**Gathered:** 2026-05-13
**Revised:** 2026-05-14 (unification target changed to per-functional subcrates; D-10 family, D-LOCK-A, D-04, D-05 revised; D-11/D-12 added)
**Revised:** 2026-05-15 (D-13 added — P11-INV-5 per-design budget under D-10b)
**Revised:** 2026-05-15 (second pause — D-02 spike-pending; D-14, D-15, D-16, D-17 added for the architectural blocker found at 11-04 Task 1A)
**Revised:** 2026-05-18 (D-02 locked to Option C via user decision, then reconsidered; D-02 re-locked to Option A via user decision — improve Python tooling to refactor helpers correctly; timeline open-ended; blocking anti-patterns codified; replan structure adjusted to 11-05..08)
**Revised:** 2026-05-18 (third session — stale artifacts deleted; D-18 added for Serena MCP refactoring tooling; 11-05 status clarified as COMPLETE; 11-06..08-PLAN-NEW files and .continue-here.md removed)
**Revised:** 2026-05-18 (fourth session, post-11-06 HALT — D-03 amended for f32 correctness gating; D-19..D-24 added: f32+f64 parametric test scope, A1 locked as the only path supporting helper-level dual-precision tests, cast_from script policy, 3-gate pre-bulk validation, surgical revert scope, AP-7 codified)
**Status:** Ready for planning (re-plan required — A1 LOCKED via D-20; 11-06..08 plans REGENERATE per D-24; f32+f64 dual-precision test surface mandated by D-19)

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
  - **LOCKED (2026-05-18 via user decision, Option A selected — reconsidered 2026-05-18):** Refactor all 38 helpers in `crates/kernels/math/src/` to be generic over `<F: Float>`. Chunks call the now-generic helpers with no call-site wrapping needed. Internal helpers use `F::new(literal)` and `F::sqrt(x)` for generic arithmetic. Named constants (`M_PI`, `M_CBRT3`, …) are wrapped at definition in the helper module. Sound per `cubecl_macro_fanout_manual.md` §6. **Approach:** The Phase 2 `_refactor_helper_*` scripts have systematic syntax errors in 11 files (incomplete regex patterns). Rather than work around these via translator-side boilerplate (Option C), the replan will improve the Python tooling to fix the helpers correctly. This is the architecturally cleaner solution. Timeline is open-ended — quality over speed.

### Precision policy (overrides existing CLAUDE.md "f64 only" rule for kernel chunks)
- **D-03:** Kernel chunks are generic over `<F: Float>`. **f64 is the default and the primary correctness target** — the oracle verification gate (D-05) runs at f64 with 1e-12 relative error.
  - **AMENDED 2026-05-18 (fourth session, by D-19):** f32 is now ALSO a correctness target at a relaxed tolerance (1e-6 relative error vs f64 oracle widened to f64). f32 is no longer "performance-only opt-in, no correctness gate" — it is a first-class secondary correctness target, env-gated at run time (`LIBXC_RS_F32=1`).
  - Typed-error-if-device-lacks-selected-precision rule still applies.
- **D-03a:** `CLAUDE.md` must be updated as part of this phase to reflect BOTH the original policy shift AND the D-19 amendment: "f64 is the primary correctness target at 1e-12; f32 is a secondary correctness target at 1e-6 relative, env-gated at test time via `LIBXC_RS_F32=1`."

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

### P11-INV-5 redefined — launch-surface invariant for the per-functional design (NEW 2026-05-15)
- **D-13:** P11-INV-5's original form (`#[cube(launch_unchecked)]` count ≤ 23 in `crates/kernels/`) is **unsatisfiable** against the D-10b dispatch design and is **revised**. Root cause: the dispatch macros (`ten_arm_dispatch_gga!`, `mgga_zero_scalar_unpol_dispatch!`), preserved verbatim per D-10b, call `.launch_unchecked()` per `(functional × output-module)` — so every routed entry kernel MUST be `#[cube(launch_unchecked)]` (~1677 total: ~168 routed functionals × ~10 output modules). The ≤22/23 baseline was measured on the pre-Phase-11 numbered-subcrate tree whose dispatch layer never compiled (Blocker B1) — it was never a working reference point. **Resolution (user decision 2026-05-15, replan of 11-03..06): keep the dispatch macros as-is (D-10b honored); redefine the invariant AND rewrite `tools/audit_cube_launch.sh` to a per-design budget:**
  - Every **routed** functional has exactly one `#[cube(launch_unchecked)]` entry kernel per emitted output module — no more, no fewer than the routing table (`tools/kernel_routing.py`) dictates.
  - No **unrouted** functional kernel is launchable — the `kernel_routing.py` "is routed?" gate still demotes unrouted entry kernels to plain `#[cube]` (the §4 anti-pattern fix from quick task 260512-q01 stands).
  - `crates/kernels/math/` `#[cube(launch_unchecked)]` count stays **≤ 22** (unchanged — Phase 11 MUST NOT add launchables to `math/`).
  - The rewritten `audit_cube_launch.sh` asserts these three conditions, NOT a flat `≤23` count.
  - **Why this is sound under D-10:** the `cubecl_macro_fanout_manual.md` §5/§19 fan-out warning is about launch surface *within one compilation unit*. After the D-10 per-functional-subcrate restructure, each subcrate holds only its own ~10 launch wrappers and compiles independently under `jobs = 1` — the ~1677 never expand in one rustc invocation. Per-functional subcrates are themselves the structural mitigation P11-INV-5 was guarding for.
  - **Rejected:** redesigning the dispatch into manual §5/§19 generic-launch kernels + `#[comptime]` functional/output selection (would satisfy `≤23` as-is, but requires revising D-10b's preserve-the-macros mandate, re-researching the generic-launch architecture, and reworking the translators' launch policy + `emit.py` + both dispatch generators — disproportionate when per-functional subcrates already neutralize the cost).

### D-02 validation via canary spike (EXECUTED 2026-05-18 — plan 11-05 COMPLETE)
- **D-14:** **EXECUTED in plan 11-05.** All 38 helpers in 16 files were refactored to generic `<F: Float>` (commits `466e074d0`, `d8cc4da0c`, `7a65f3bc6`, `233a8890d`, `dcb7d517d`). The logical refactoring is 100% complete. Three-leg validation (compile + parity + idempotency) is **deferred to 11-06** — the automated refactoring script left syntax errors that block compilation. Syntax cleanup is the first task of 11-06 (D-18).
  - **Option A status:** All 16 helper files contain generic `<F: Float>` signatures. The `tools/refactor_helpers_generic.py` script was created and used for bulk transformation. Remaining blockers are syntactic (function signature malformations, malformed literals from the automated pass), not semantic.
  - **Known syntax errors from automated script (11-06's fix scope):**
    1. Function signature malformations — e.g., missing opening parenthesis, `param: f64` where should be `param: F`
    2. Numeric literal errors — incomplete exponents (`0.123e-` instead of `0.123e-4`), malformed loop constructs
    3. CubeCL 0.10 API drift in `#[cfg(test)]` blocks — `ArrayArg::from_raw_parts` signature changed; ~165 test-only errors across helper files
  - **Validation gate (carries into 11-06):** After syntax cleanup:
    1. `cargo build -p libxc-kernel-mgga_c_b94` GREEN under CubeCL 0.10 + `jobs = 1`
    2. Parity vs libxc oracle at **1e-12 relative error on energy AND all routed derivatives** (one-shot `is_deferred(id)` bypass for mgga_c_b94; D-11 preserved)
    3. Idempotency: re-run translator, no diff

### Compile-first entry gate (NEW 2026-05-15 — second pause)
- **D-15:** The 2026-05-15 replan establishes a single-canary compile-first entry gate that **MUST pass before the per-`-p` sweep starts**. This is the structural correction for Phase 11's repeated pattern of declaring structural completion without per-`-p` cargo gates (see anti-pattern table below).
  - **Canary:** Same as D-14's spike canary (`mgga_c_b94`). Spike outcome IS the gate's first deliverable.
  - **Gate scope (all three legs must be GREEN):**
    1. `cargo build -p libxc-kernel-mgga_c_b94` (the kernel subcrate compiles under the chosen ABI).
    2. `cargo build -p libxc_rs` (the dispatch tree at `src/kernel/mgga/` + `src/eval/mgga_dispatch/` imports + expands the dispatch macros around mgga_c_b94's deferred status, validating D-10b + D-13 alongside the chosen D-02 ABI).
    3. Ad-hoc parity vs libxc oracle at 1e-12 on energy + routed derivatives, with the **one-shot `is_deferred(id)` bypass** noted in D-14. NOT a permanent unfilter — D-11 stays.
  - **Failure recovery:** Gate failure halts the replan, writes `.continue-here.md` documenting the failure mode, triggers a **third `/gsd-discuss-phase 11` pass**. No in-plan retry-grinding — that was the failure mode of 11-04 pre-pause. Each retry of the same broken approach is itself an anti-pattern.

### Translator emit lives in cse.py AST pass (LOCKED 2026-05-18 — Option A selected)
- **D-16:** The D-02 Option A ABI's tooling-side emit code focuses on **f64 literal wrapping** in `tools/translate_v2/cse.py` as an **AST-level pass**, with no call-site wrapper logic needed (helpers are now generic).
  - **For Option A (helpers generic — LOCKED):** cse.py's AST visitor stays minimal — chunks compile cleanly against the now-generic helpers with no call-site wrapping needed. The bulk of Option A's work is in `crates/kernels/math/src/` (refactoring helpers to generic via improved Python tools in D-14), not in the translator emit logic.
  - **Family A literal wrapping stays as-is.** The q01 commit `5c379dc25` fixes (`F::new(...)` wraps for f64 literals, MAX_TUPLE_ARITY = 12, single-output scalar return on `-> F`) are preserved. These are orthogonal to the helper refactoring and remain in place.
  - **What stays in per_functional.py:** q01's single-output scalar-return decision (`-> F` vs `-> (F,)`, commit `5c379dc25`) stays in per_functional.py (no migration). The MAX_TUPLE_ARITY = 12 cap stays in cse.py.

### Replan structure: 5 plans 11-04..08 (REVISED 2026-05-18 — third session)
- **D-17:** Replan splits into five plan slots. Updated status:
  - **11-04 (COMPLETE — retroactive partial SUMMARY):** Commit `39eb75f93` (verify dev-dep narrowing per D-05) landed. Status: PARTIAL — replanned mid-Task-1A.
  - **11-05 — D-02 ABI spike (COMPLETE).** All 38 helpers in 16 files refactored to generic `<F: Float>`. Syntax errors remain from automated script; three-leg validation deferred to 11-06. See 11-05-SUMMARY.md.
  - **11-06 — Syntax cleanup (D-18: Serena MCP) + translator update (D-16) + from_raw_parts drift fix.** First task: fix syntax errors in `crates/kernels/math/src/` via Serena MCP server (Python tools as fallback). Second task: fix 165 `ArrayArg::from_raw_parts` API drift errors in `#[cfg(test)]` blocks. Third task: validate three-leg gate (compile + 1e-12 parity + idempotency on mgga_c_b94). cse.py AST visitor confirmation (Option A: minimal; chunks compile cleanly against now-generic helpers).
  - **11-07 — Regen 266 subcrates + compile-first entry gate (D-15) on mgga_c_b94.** Translator runs over full Maple input; mgga_c_b94 gate passes all three legs (kernel + dispatch + ad-hoc parity).
  - **11-08 — Per-`-p` sweep + audits + close.** Incremental per-subcrate `cargo build -p`. Audit tools rewritten per D-13. ROADMAP.md criteria updated. Phase close + final SUMMARY.

### Serena MCP refactoring tooling (NEW 2026-05-18 — third session; REAFFIRMED 2026-05-18 fourth session by D-21)
- **D-18:** For the 11-06 syntax cleanup task, use **Serena MCP server** (`serena start-mcp-server --context=claude-code --project-from-cwd`) as the primary refactoring tool. Serena is already configured in `~/.claude.json`. Python tools (e.g., `tools/refactor_helpers_generic.py`, ad-hoc sed scripts) are the fallback if Serena proves too difficult for specific patterns. The three error categories to fix (D-14 known syntax errors) are: (1) function signature malformations, (2) malformed numeric literals, (3) `ArrayArg::from_raw_parts` API drift in tests.

### Test scope expansion: f32 + f64 parametric testing (NEW 2026-05-18 — fourth session)
- **D-19:** **All tests** in the libxc_rs test surface are parameterized over BOTH `<F = f32>` and `<F = f64>` precisions and exercised under each. Scope:
  - Helper unit tests under `crates/kernels/math/tests/` (e.g., `spike_tuple_return_cube.rs`, `spike_cse_emit_q01.rs`, all per-helper unit tests)
  - Per-functional subcrate tests under `crates/kernels/{lda,gga,mgga}/<func>/tests/` (where present)
  - All spike harnesses, both existing and new
  - `verify/tests/parity_phase11.rs` (smoke + worst-case sets)
  - `verify/tests/oracle_*.rs` — full 649-functional oracle harness under both precisions
  - **Architectural consequence:** Helper unit tests parameterized over F require **helpers to be generic over `<F: Float>`** at the source level. This is what locks **A1** as the architectural path (D-20). Hybrid and C are ruled out: their concrete-f64 helpers cannot be tested parametrically over F at the helper unit-test level.
- **D-19a (amends D-03):** F32 is elevated from "performance-only opt-in, no correctness gate" to "**correctness-gated at relaxed tolerance**". Oracle parity at **1e-6 relative error** on energy + routed derivatives (vs f64 libxc oracle widened to f64). The 1e-12 gate is preserved for f64.
- **D-19b — F32 test execution mode is env-gated:** Tests run f64 by default. `LIBXC_RS_F32=1` env var enables f32 test execution (compile + parity comparison). CI runs both modes; local devs run f64 by default unless f32 coverage is needed. Avoids day-to-day test-time doubling while keeping f32 a first-class correctness target. Discovery: a single env-var check at test-suite startup gates whether f32 instantiations are executed; helpers and chunks always *compile* against both (compile gate is unconditional).
- **D-19c — F32 tolerance for ill-conditioned cases:** Brent root-finders (`br89.rs`, `mbrxc.rs`) and similar iterative algorithms may need per-test relaxation beyond 1e-6 due to f32 convergence behavior. Default gate is 1e-6; per-test overrides documented in a small tolerance table per (functional, derivative-order) pair. Implementation surface left to planner.

### Architectural path: A1 LOCKED for 4th iteration (NEW 2026-05-18 — fourth session)
- **D-20:** Resolves the `F::new(val: f32)` vs f64-named-constant blocker that HALTed plan 11-06 (commit `75c0f5112`, 515 errors dominated by 447 × E0308 `expected f32, found f64`). **Locked path: A1 — cast_from script + surgical manual fixes.** Required by D-19's helper-level f32+f64 test scope (only generic-helpers paths support parametric tests at the helper layer).
  - **A1 approach:**
    1. Extend `tools/refactor_helpers_generic.py` with cast_from policy: every `F::new(IDENT)` site is classified by symbol class and rewritten accordingly:
       - **f64 const** (SQRT_DBL_EPSILON, LOG_DBL_MAX, TWO_DBL_MIN, TWO_SQRT2_SQRT_DBL_EPSILON, RS_CONST, KF_CONST, ERX, PI_TWO_THIRDS, POW_32PI_TWO_THIRDS, …): `F::new(IDENT)` → `F::cast_from(IDENT)` (cubecl-core 0.10 `Cast` trait — blanket `impl<P: CubePrimitive> Cast for P`, defined at `cubecl-core-0.10.0/src/frontend/element/cast.rs:14-37`)
       - **f32 const** (none currently known but classifier must handle): keep `F::new(IDENT)` (Float::new accepts f32 per `cubecl-core-0.10.0/src/frontend/element/float.rs:75`)
       - **Doc-comment / string-literal context** (LDA, MGGA, ID, A, C, BR89, MBRXC, "17.5K", …): revert to bare `IDENT` / original string text
       - **Non-generic file** (`deferred.rs`): full revert of all auto-script changes — the file is not `<F: Float>`-parameterized
       - **Numeric literal with `_f64` suffix** (e.g., `3.0_f64` mis-wrapped as `F::new(3.)0_f64`): rewrite to `F::new(3.0)`
       - **Range operator `..`** (e.g., `0..500` mis-wrapped as `0.F::new(.500)`): revert to `0..500`
       - **Double-wrap pattern** (e.g., `f64::MAX` mis-wrapped as `F::F::new(MAX)`): restore original `f64::MAX` semantics via case-by-case manual fix
    2. Surgical manual fixes for known non-script regressions (D-23 enumerates):
       - `deferred.rs` (full revert) — file is not generic-over-F
       - `special.rs:224` — `F::F::new(MAX)` → restore `f64::MAX` semantics
       - `bessel.rs` and similar — `let mut <var>: f64 = F::new(0.0)` → `let mut <var>: F = F::new(0.0)` (6 known sites)
       - `mbrxc.rs:145` — `F::new(3.)0_f64` × 3 → `F::new(3.0)`; line 154 `for _ in 0.F::new(.500)` → `for _ in 0..500`
  - **Rejected:**
    - **A2 (f32 demote):** Violates 1e-12 oracle gate in `CLAUDE.md` core constraint and `REQUIREMENTS.md:4` core value. Non-starter.
    - **C (Option C revival):** Reverses the session-2 reconsideration (cast boilerplate at ~581K call sites). Also incompatible with D-19's helper-level dual-precision test scope (concrete-f64 helpers cannot be tested parametrically over F).
    - **Hybrid (Phase-1 generic + Phase-2 revert + translator casts):** Phase-2 concrete-f64 helpers cannot satisfy D-19's helper-level test parameterization. Ruled out.

### Primary refactoring tool: Serena MCP (NEW 2026-05-18 — fourth session; reaffirms D-18)
- **D-21:** Serena MCP is the **primary** tool for A1's source edits. The 11-06 HALT empirically confirmed that pure-regex Python is insufficient — distinguishing f64 const from f32 literal from doc-comment text from string-literal text from range-op `..` from `_f64` literal suffix requires semantic awareness. Serena MCP (LSP-backed, already configured in `~/.claude.json`) handles these classifications natively. **Fallback policy:** extended `tools/refactor_helpers_generic.py` is allowed for purely-syntactic bulk operations AFTER Serena MCP has identified the safe-to-bulk-transform call sites (e.g., a final f64-literal-wrap pass on already-classified locations). Pure regex MUST NOT be the primary classifier.

### Pre-bulk validation gate: 3-gate sequence (NEW 2026-05-18 — fourth session; structural mitigation for 11-06 failure mode)
- **D-22:** Before the cast_from-aware refactor script bulk-runs on the 11 problematic Phase-2 helper files, three gates must green **in strict sequence**. Skipping or reordering = AP-7 violation.
  - **Gate 1 — Synthetic-fixture coverage matrix.** Create `tools/refactor_test_fixtures/symbol_class_matrix.rs` (or planner-equivalent path) containing every known symbol class:
    - f64 const declaration + usage (in generic body)
    - f32 const declaration + usage (in generic body)
    - Doc-comment with constant-like text (`LDA`, `MGGA`, `ID`, `BR89`, …)
    - String literal with constant-like text (`"17.5K"`, `"BR89 model"`, …)
    - Range operator `..` (`for _ in 0..500`)
    - `_f64` literal suffix (`3.0_f64`)
    - Double-wrap pattern (`f64::MAX`)
    - Non-generic helper context (`pub fn is_deferred(id: u16) -> bool { … }`)
    - Mixed: f64 const used inside generic body with arithmetic against `F`
    - Run the new script on it; `cargo build` on the fixture MUST green; diff inspection: every change matches the per-symbol-class policy from D-20.
  - **Gate 2 — Canary file: bessel.rs.** Chosen as canary because it has the highest `F::new(` count (200) AND the most diverse symbol classes (f64 const usage, doc-comments, type annotations).
    - Revert all Phase-2 changes on `crates/kernels/math/src/bessel.rs` (from `7a65f3bc6`/`dcb7d517d`/`233a8890d`) — start fresh from pre-Phase-2 state
    - Run the new script on bessel.rs alone
    - `cargo build -p libxc-kernel-math` MUST green
    - Diff inspection: every change matches the policy
  - **Gate 3 — Spike harness: chunk → helper integration boundary (mgga_c_b94).** The missing 11-05 spike coverage. Tests the actual production integration boundary that the helper-level changes affect.
    - **Compile gate:** `cargo build -p libxc-kernel-mgga_c_b94` green (depends on bessel.rs via post-Gate-2 state + other helpers as needed)
    - **Parity gate at f64:** 1e-12 relative error on energy + routed derivatives of mgga_c_b94 (one-shot `is_deferred(id)` bypass per D-14/D-15)
    - **Parity gate at f32 (NEW per D-19):** 1e-6 relative error under `LIBXC_RS_F32=1`
    - **Idempotency:** re-run the script on bessel.rs, `git diff` must be empty
  - **Only after all three gates green in strict sequence** does the script bulk-run on the remaining 10 Phase-2 files (`expint_e1.rs`, `integrate.rs`, `br89.rs`, `mbrxc.rs`, `special.rs`, `erf.rs`, `dft_quantities.rs`, `bspline.rs`, and others minus those that have already been validated). After bulk run, `cargo build -p libxc-kernel-math` MUST green as the bulk-run exit gate.

### Surgical revert scope (NEW 2026-05-18 — fourth session; derived from D-20 + D-22)
- **D-23:** Revert/fix scope of commits `7a65f3bc6` (batch convert 10 helpers) + `dcb7d517d` (subset fixes) + `233a8890d` (partial syntax fixes):
  - **`deferred.rs` — FULL REVERT.** All 34 `F::new(…)` sites are wrong: the file is not generic-over-F (`pub fn is_deferred(id: u16) -> bool`). String literals corrupted (`"… F::new(17.)5K lines …"` was `"17.5K"`), doc comments corrupted (`F::new(LDA)`, `F::new(MGGA)`, `F::new(ID)` were bare identifiers in prose), real code corrupted (`F::new(DEFERRED_LDA_FUNCTIONALS).iter()` should be bare). Restore to pre-`7a65f3bc6` state.
  - **`special.rs:224` — Surgical fix.** `F::F::new(MAX);` (double-wrap of `f64::MAX`) → restore the original `f64::MAX` semantics. Specific transformation TBD by planner after re-reading the pre-commit state (likely `result = f64::MAX;` or `result = F::cast_from(f64::MAX);` depending on the surrounding context).
  - **`bessel.rs` and any similar files — Surgical fix.** All `let mut <var>: f64 = F::new(0.0)` patterns (6 known sites in bessel.rs per 11-06 HALT report) → `let mut <var>: F = F::new(0.0)`. Search-and-fix: `grep -nE 'let mut \w+: f64 = F::new'` enumerates the sites.
  - **`mbrxc.rs:145` — Surgical fix.** `F::new(3.)0_f64` × 3 sites → `F::new(3.0)`; line 154 `for _ in 0.F::new(.500)` → `for _ in 0..500`.
  - **All other Phase-2 changes — KEEP, then re-process** with the D-20 cast_from-aware script (Gate 2 canary first per D-22, then bulk on remaining 10 helpers).
  - **Phase-1 manually-refactored files — UNCHANGED.** `powers.rs` (3 F::new), `piecewise.rs` (1 F::new), `lambert_w.rs` (14), `polynomials.rs` (0), `spin.rs` (6) — these are proven clean from commits `466e074d0` + `d8cc4da0c` and stay as-is.

### Plan 11-06/07/08 regeneration scope (NEW 2026-05-18 — fourth session)
- **D-24:** All three forward plans regenerate per the locked decisions D-19..D-23:
  - **11-06** — **REPLACED scope** (not amendment). Tasks:
    1. Pre-flight: verify `.cargo/config.toml` invariants per AP-2 (jobs=1, RUST_MIN_STACK=67108864, target-dir).
    2. Surgical revert per D-23 (deferred.rs full revert; special.rs/bessel.rs/mbrxc.rs surgical fixes).
    3. Extend `tools/refactor_helpers_generic.py` with cast_from policy per D-20 (or replace it with Serena-MCP-driven equivalent per D-21).
    4. Gate 1 — synthetic fixture build per D-22.
    5. Gate 2 — bessel.rs canary per D-22 (revert bessel.rs Phase-2 changes first; run script; compile-gate `cargo build -p libxc-kernel-math`).
    6. Gate 3 — mgga_c_b94 chunk→helper spike at f64 1e-12 AND f32 1e-6 (`LIBXC_RS_F32=1`).
    7. Bulk-run script on remaining 10 helpers.
    8. Three-leg exit gate: `cargo build -p libxc-kernel-math` green, `cargo build -p libxc-kernel-mgga_c_b94` green, parity green at both precisions, idempotency green.
  - **11-07** — Regen 266 subcrates + D-15 entry gate (compile-first) **AT BOTH PRECISIONS for mgga_c_b94 canary**. The original 11-07 plan already includes D-15; the amendment is the f32 leg.
  - **11-08** — Per-`-p` sweep + audits + close, **with f32 test mode exercised under env-gate**. The per-`-p` sweep includes a `LIBXC_RS_F32=1` pass on the smoke parity set. The full 649-functional f32 oracle sweep is a phase-end deliverable, not a per-iteration gate (matches D-05's "full per-subcrate parity sweep runs at phase end" pattern, now extended to both precisions).

### Critical Anti-Patterns for Phase 11 Replan (NEW 2026-05-18 — documented in `.continue-here.md`)

The following patterns have been **empirically observed to break the replan** in prior iterations and must be actively prevented:

- **AP-1 (blocking): Re-executing without replanning**
  - **What it is:** Running `/gsd-execute-phase 11` against the existing 11-04..06 plan tree without a fresh replan. The plans are stale; each compile attempt loops on the architectural mismatch D-02 was supposed to resolve.
  - **How it manifested:** Plans 11-01..03 claimed structural completion without per-`-p` compile gates. When 11-04 Task 1A introduced the first per-`-p` gate, it surfaced the D-02 helper-layer incompatibility.
  - **Structural fix:** The regenerated 11-05..08 plans MUST have **entry-gate criteria** (per-`-p` compile check on a canary functional) BEFORE any structural work begins. This reverses the gate order: compile-first (entry), not compile-after (exit).

- **AP-2 (blocking): Modifying `.cargo/config.toml`**
  - **What it is:** Changing `[build] jobs` or `[env] RUST_MIN_STACK` in `.cargo/config.toml`. These are D-07/D-08/D-09 load-bearing constraints.
  - **How it manifested:** An uncapped `jobs` override (even temporary) causes OOM (exit 137) on the 30GB machine. The committed `jobs = 1` is the source of truth; user restores the cap by hand.
  - **Structural fix:** Phase 11 plans MUST include a **pre-flight check** task that verifies `.cargo/config.toml` has `jobs = 1` and `RUST_MIN_STACK = 67108864`. No plan task touches this file directly, and no task overrides these values via `CARGO_BUILD_JOBS` or env-var proxies.

- **AP-3 (blocking): Hand-editing generated kernel files**
  - **What it is:** Manually patching `crates/kernels/{lda,gga,mgga}/*.rs` files to fix compile errors instead of fixing the root cause in `tools/translate_v2/`.
  - **How it manifested:** When 11-04 encountered "Mul<F> for {float}" errors in generated chunks, the temptation was to hand-edit the generated code. Hand edits don't survive regen (D-LOCK-D idempotency requirement).
  - **Structural fix:** D-LOCK-D idempotency is NON-NEGOTIABLE. Every plan task addressing a kernel-tree compile error MUST follow: "identify root cause in `tools/translate_v2/<file>` → modify translator → regen from Maple → verify gate". Explicit ban in plan task descriptions: "no hand-edits of generated files".

- **AP-4 (warning):** Reverting commit `5c379dc25` (q01 emit fixes). Three CubeCL 0.10 emit fixes are independently correct and validated by `spike_cse_emit_q01.rs`. The replan **builds on them** — MAX_TUPLE_ARITY=12 stays, single-output scalar return stays. Only the regex `_wrap_f64_literals` stays (not retired, since Option C doesn't replace it with an AST pass).
- **AP-5 (warning):** Treating 11-01/11-02/11-03 SUMMARYs as needing redo. Their structural deliverables (D-02 isolated spike, audit tools, baseline, dispatch audit, clean-slate regen of 266 subcrates at `97d6347be`, D-13 dispatch verification) survive the second pause. The replan reframes WHAT 11-04..08 verify, not WHAT 11-01..03 produced.
- **AP-6 (blocking — reframed under AP-1):** Declaring structural completion without per-`-p` cargo gates. The defining failure mode of Phase 11 across 2026-05-13..05-15 was three structural-completion claims (wide-tuple chunk emission, literal-coercion, 1-tuple scalar return, helper-layer architecture) each without a per-`-p` compile gate. This is now the **entry-gate structural fix** for AP-1: compile-first, before structural claims.

- **AP-7 (blocking — NEW 2026-05-18, fourth session):** **Spike exercises unit boundary instead of integration boundary.**
  - **What it is:** The architectural validation spike tests one component in isolation (e.g., tuple-return, literal coercion) but never exercises the integration boundary the production code will cross (chunk → helper call, dispatch macro → launch_unchecked, etc.). The integration boundary is where API-contract mismatches surface; isolating the components hides them.
  - **How it manifested:** The 11-05 spike used `spike_cse_emit_q01.rs` and `spike_tuple_return_cube.rs` to validate Option A's tuple-return + literal-coercion in isolation against synthesized expressions. It never compiled a chunk that called a helper. The CubeCL `Float::new(val: f32)` constraint on the helper side vs the chunk's `<F: Float>` call site was untested. Plan 11-06's entry gate (`cargo build -p libxc-kernel-math`) was the first time chunk → helper integration was exercised — and it surfaced 447 × E0308 errors that an integration-boundary spike would have caught in 11-05.
  - **Structural fix:** Every architectural decision in Phase 11 (and beyond) MUST be validated by a spike that exercises the **same integration boundary** the production code will cross. For helper-layer changes: a real chunk → helper call must compile + parity-test. For dispatch layer: a real `from_id → ten_arm_dispatch → launch_unchecked` chain must compile + parity-test. Unit-level spikes (one component in isolation) are insufficient for architectural validation. **D-22's Gate 3 codifies this pattern for the 4th-iteration replan.**

### Claude's Discretion
- **Subcrate package naming.** Recommended: follow the existing `libxc-kernel-*` convention — package `libxc-kernel-<func>` (e.g. `libxc-kernel-gga_c_acgga`), lib name `libxc_kernel_<func>`. Planner confirms the exact spelling (hyphen vs underscore handling in the package name) after reading the current numbered-subcrate `Cargo.toml` naming.
- Internal structure of the CSE pass (Maple AST walker vs post-translation Rust AST walker vs Python-side intermediate IR). The decision is "CSE-aware" — implementation surface is left to the planner + phase researcher.
- Whether to extend the existing `tools/translate_*.py` family in place or fork a `tools/translate_v2/` tree. Planner's call after reading the current splitter implementation.
- How the splitter generates ~264 per-subcrate `Cargo.toml` files and rebuilds the root `[workspace] default-members` array (template vs programmatic). Planner's call.
- Whether to add a `tools/audit_kernel_size.py` CI gate — already partially built in Wave 0 (`tools/audit_kernel_size.py` exists per 11-01-SUMMARY). Planner decides whether to extend it for the subcrate-count invariant too.
- Whether to retain the existing `tools/split_oversized_{kernel,mgga}.py` / `tools/rebatch_mgga.py` / `tools/split_mgga_7_kcis.py` helpers as scaffolding or delete them (they assume the numbered-subcrate layout, now obsolete). Planner's call.
- **(NEW 2026-05-15)** Spike harness shape for D-14. Whether the A-vs-C race runs as a pair of git branches (`spike-d02-a` / `spike-d02-c`) compared via `git diff` + benchmark, or as two sequential commits on the same branch with `git stash` rollback, or as two pairs of files (`mgga_c_b94/{a,c}/`) under a spike subdirectory. Planner's call after reading 11-05's PLAN.md.
- **(NEW 2026-05-15)** Exact mechanism for the one-shot `is_deferred(id)` bypass in the D-15 gate's parity step. Options: a `#[cfg(feature = "phase11-spike")]` gate, a runtime env var, a separate gate-only test binary that constructs `MggaFunctional` bypassing the constructor's deferred check. Planner picks; whichever is most local and most easily reverted.
- **(NEW 2026-05-15)** Whether 11-08's `audit_cube_launch.sh` rewrite (D-13 per-design budget) gates the close, or whether it lands as a separate audit-suite update task that can defer to a follow-up if D-13's invariant proves contentious. Planner's call.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### CubeCL design constraints (load-bearing)
- `docs/manual/Cubecl/cubecl_macro_fanout_manual.md` — THE authoritative reference for how to subdivide kernels under CubeCL. Key sections: §3 ("Keep the CubeCL expansion surface as small as possible"), §6 (Prefer Generic Numeric Kernels — supports D-03 generic `<F: Float>`), §10 ("Break apart meaningful algorithmic stages, NOT every expression-level helper" — supports D-01 CSE-aware over per-statement), §13 (Reduce Element-Type Generic Explosion — caveats D-03), §19 (Recommended low-fan-out architecture), §21 (Refactoring Checklist). Read end-to-end before planning.

### CubeCL 0.10 API contracts (load-bearing for D-20 cast_from policy)
- `/home/user/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cubecl-core-0.10.0/src/frontend/element/float.rs:75` — `pub trait Float { fn new(val: f32) -> Self; ... }`. **The root cause of the 11-06 HALT.** `Float::new` accepts only `f32`; passing an `f64` const triggers E0308 in `<F: Float>` body. ~447 of the 515 errors in `cargo build -p libxc-kernel-math` post-11-05 derive from this.
- `/home/user/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cubecl-core-0.10.0/src/frontend/element/cast.rs:14-37` — `pub trait Cast: CubePrimitive { fn cast_from<From: CubePrimitive>(value: From) -> Self; ... }` with blanket `impl<P: CubePrimitive> Cast for P`. **The fix per D-20:** `F::cast_from(<f64 const>)` preserves f64 precision through generic helpers when F=f64, and emits a narrowing cast when F=f32 (precision-only — per D-03/D-19a, f32 is gated at 1e-6 relative).

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
- **`tools/translate_v2/cse.py` (NEW post-2026-05-15 — D-16 target)** — current home of MAX_TUPLE_ARITY=12 cap. D-16's AST visitor lands here.
- **`tools/translate_v2/per_functional.py` (NEW post-2026-05-15)** — current home of q01's `_wrap_f64_literals` (retiring per D-16) and the single-output scalar-return shape (staying or migrating per planner).
- **`tools/translate_v2/emit.py`** — body emit path; F-coercion marks (if D-16's AST visitor uses the hybrid mark-and-emit pattern) read here.

### Math helper layer (D-14 scope)
- `crates/kernels/math/src/piecewise.rs` (6 fns: `piecewise3`, `piecewise5`, `Heaviside`, …)
- `crates/kernels/math/src/powers.rs` (20 fns: `pow_1_3`, `pow_2_3`, `pow_4_3`, `pow_5_3`, `pow_3_2`, `pow_1_4`, `pow_7_3`, `pow_2`, `pow_3`, `safe_cbrt`, …)
- `crates/kernels/math/src/erf.rs` (6 fns)
- `crates/kernels/math/src/lambert_w.rs` (3 fns: `lambert_w`, …)
- `crates/kernels/math/src/bspline.rs` (7 fns: `case21_xbspline`, `case21_cbspline`, …)
- `crates/kernels/math/src/br89.rs` (3 fns — Brent root finder, D-08 stack-sensitive)
- `crates/kernels/math/src/bessel.rs` (14 fns: `xc_bessel_I0`, `xc_bessel_I0_scaled`, `xc_bessel_I1`, `xc_bessel_I1_scaled`, …)
- `crates/kernels/math/src/dft_quantities.rs` (8 fns: `wigner_seitz_rs`, `reduced_gradient_s`, `tf_kinetic`, `dimensionless_alpha`, …)
- `crates/kernels/math/src/spin.rs` (10 fns: `compute_total`, `compute_zeta`, `to_total_zeta_total`, `spin_scaling`, `clamp_zeta`, …)
- `crates/kernels/math/src/integrate.rs` (11 fns)
- `crates/kernels/math/src/polynomials.rs` (4 fns)
- `crates/kernels/math/src/mbrxc.rs` (3 fns)
- `crates/kernels/math/src/special.rs` (6 fns)
- `crates/kernels/math/src/expint_e1.rs` (8 fns: `xc_e1_scaled`, …)
- `crates/kernels/math/src/{constants,deferred,lib}.rs` — non-function modules.
- **Total: 38 helper functions, all concrete `f64`. Option A refactors all 38; Option C wraps call sites instead.**

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
- **`.planning/quick/260515-q01-cse-chunk-arity-cap-12/BRIEF.md` + `SPIKE-FINDINGS.md` (NEW 2026-05-15)** — empirically established the four-layer architectural blocker that triggered the second pause: (1) MAX_TUPLE_ARITY 16→12 cap, (2) CubeCL 0.10 literal-coercion E0277, (3) 1-tuple `let`-binding E0308 on `-> (F,)`, and **(4) the math/src/ helper-layer concreteness mismatch — 38 helpers f64, 0 generic, ~581,694 call sites in `crates/kernels/{lda,gga,mgga}/`**. q01 delivered fixes for layers 1–3 in commit `5c379dc25`; layer 4 is D-14's spike scope. Read SPIKE-FINDINGS.md "Pattern test matrix" before planning 11-05.
- **`crates/kernels/math/tests/spike_cse_emit_q01.rs` (NEW 2026-05-15)** — surviving Q4/Q5 positive-regression tests for the q01 emit idioms (`F::new(literal)` wraps, scalar return on 1-output chunks). Reference shape for the D-14 spike's compile gate.

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

<spec_to_criterion_map>
## SPEC-11-Rx → ROADMAP Phase 11 Success Criterion Map (NEW 2026-05-18 fix per checker BLOCKER 1)

Plan frontmatter (11-05/06/07/08) tags requirements using the local IDs `SPEC-11-R1..R8`. These IDs are NOT defined in REQUIREMENTS.md, ROADMAP.md, or VALIDATION.md — ROADMAP.md uses **Success Criteria #1..#8 + P11-INV-A1** (see ROADMAP.md lines 251–260). The table below provides the canonical mapping so traceability is unambiguous.

| Local ID | ROADMAP Success Criterion (source: .planning/ROADMAP.md lines 251–260) | Delivered by plan(s) |
|---|---|---|
| SPEC-11-R1 | #1 — `find crates/kernels -maxdepth 1 -type d` shows no `lda-N`/`gga-N`/`mgga-N` numbered children AND no per-family Cargo.toml (per D-10a; family dirs are plain directories) | 11-03 (clean-slate delete + 266-subcrate regen), re-verified by 11-07 (idempotent regen) |
| SPEC-11-R2 | #2 — Zero `.rs` files >5,000 lines (hard cap per D-LOCK-B) | 11-02 (CSE pass tooling), 11-03 (initial regen), 11-07 (full regen verifies post-Option-A) |
| SPEC-11-R3 | #3 — Splitter capable of subdividing single-output expressions; r4scan, br89_explicit, mgga-{8,9,11} all ≤5K | 11-02 (CSE pass), 11-03 (full-tree empirical verification), 11-07 (re-verified), 11-08 (final audit sweep) |
| SPEC-11-R4 | #4 — Per-`-p` cargo build across routed subcrates succeeds (D-12 reinterpretation: NOT `cargo build --workspace`) | 11-08 (the per-`-p` sweep IS this criterion's empirical gate) |
| SPEC-11-R5 | #5 — Oracle parity preserved at 1e-12 (per D-05; energy + routed derivatives at f64) | 11-05 (helper refactor preserves parity by design), 11-06 (three-leg gate on mgga_c_b94), 11-07 (D-15 entry gate + smoke), 11-08 (sweep verifies no parity regression) |
| SPEC-11-R6 | #6 — Pipeline idempotent (running twice produces no diff per D-LOCK-D / P11-INV-6) | 11-02 (deterministic emit), 11-03 (verified post-regen), 11-06 (canary idempotency), 11-07 (full-tree `test_idempotency.sh`), 11-08 (final 5-audit sweep) |
| SPEC-11-R7 | #7 — CubeCL macro fan-out audit clean per D-13 per-design budget (NOT the original ≤23 flat count — see D-13 rationale) | 11-03 (audit_cube_launch.sh D-13 rewrite committed at eea58fed7), 11-08 (post-regen re-confirmation + ROADMAP correction) |
| SPEC-11-R8 | #8 — Dispatch tree resolves cleanly post-collapse; Blocker B1 closed; zero `batchN` segments survive | 11-03 (dispatch regen + Blocker B1 closure), 11-07 (re-regen against full-tree Option-A subcrates), 11-08 (final audit_dispatch_tree.sh) |

**Note on the executed plans 11-01..04:** Their PLAN.md files do NOT use the SPEC-11-Rx scheme (this naming convention was introduced in 11-05 onward), but their *delivered work* covers many criteria. The "Delivered by plan(s)" column above documents which plan contributed evidence for each criterion regardless of whether the plan's frontmatter labeled it. Per ROADMAP.md the 5/8 executed plans (11-01..05) have already produced verifiable evidence for criteria #1 (11-03), #2 (11-03), #3 (11-02/03), #6 (11-02/03), #7 (11-03), #8 (11-03), and partial #5 (11-05 helper refactor — full validation in 11-06+).

**Coverage check across forward plans 11-06..08 frontmatter (post-fix):**
- 11-06 must list: SPEC-11-R5, SPEC-11-R6, SPEC-11-R7 (three-leg gate validates parity + idempotency on canary; preserves D-16 emit which protects R7 fan-out)
- 11-07 must list: SPEC-11-R1, SPEC-11-R2, SPEC-11-R3, SPEC-11-R5, SPEC-11-R6, SPEC-11-R8 (full regen empirically re-verifies the structural criteria + idempotency + dispatch + parity)
- 11-08 must list: SPEC-11-R3, SPEC-11-R4, SPEC-11-R6, SPEC-11-R7, SPEC-11-R8 (per-`-p` sweep is R4; final 5-audit sweep re-verifies R3/R6/R7/R8 at phase close)

After post-fix frontmatter updates: every SPEC-11-Rx appears in at least one plan's `requirements:` field across 11-05..08. 11-05's frontmatter (SPEC-11-R5, SPEC-11-R7) is preserved as-committed.

</spec_to_criterion_map>

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

- **f32 oracle gate at relaxed tolerance.** ~~Rejected for now (D-03)~~ **PROMOTED to scope (2026-05-18 fourth session) via D-19a — f32 is now correctness-gated at 1e-6 relative, env-gated at test time via `LIBXC_RS_F32=1`.**
- **Audit `error/` and `math/` module placement before workspace-modular-split phase.** Cross-referenced from todo backlog (`audit-error-math-placement.md`, score 0.6). Deferred to Phase 10 — the audit is workspace-split prep, not Phase 11 splitter/architecture work.
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

## Re-plan Note (2026-05-15)

Phase 11 paused mid-execution at plan **11-03 Task 2** (`.continue-here.md`): the
per-functional dispatch design (D-10b — dispatch macros preserved verbatim, calling
`.launch_unchecked()` per `(functional × output)`) is **mathematically incompatible**
with P11-INV-5's original `≤23` flat count — `audit_cube_launch.sh` reports 1677.
The user's resolution is **D-13** above: keep the macros, redefine the invariant +
rewrite the audit to a per-design budget. **Plans 11-03..06 must be regenerated** to
reflect D-13:
- **11-01, 11-02** stay — executed; their deliverables survive D-13 (11-02's `emit.py`
  routing-aware launch policy is correct as-is; only the *audit* and the *invariant
  definition* change).
- **11-03** — Task 1 (clean-slate delete + 266-subcrate regen + root manifest rewrite)
  is committed (`95727cb36`, `97d6347be`) and its acceptance criteria re-verified;
  the replan preserves it. Task 2 (dispatch regen) has sound WIP at `c3fba8089` the
  replan can build on. The replan must add: rewrite `tools/audit_cube_launch.sh` per
  D-13, and make the revised P11-INV-5 the gate (not the old `≤23`).
- **11-04, 11-05, 11-06** — regenerated; every `P11-INV-5` reference updated to the
  D-13 form; 11-06's close-out also corrects `11-BASELINE.md` and `11-VALIDATION.md`
  P11-INV-5 rows.

---

## Re-plan Note (2026-05-18 — second pause resolution, Option A locked)

Phase 11 paused a second time at **plan 11-04 Task 1A** (`.continue-here.md`). Task 1A's
verify dev-dep narrowing (commit `39eb75f93`, the D-05 OOM structural fix) landed cleanly
and is preserved. What failed was the assumption that D-02's `<F: Float>` chunk ABI
composes with `crates/kernels/math/src/`'s concrete-`f64` helper layer under CubeCL 0.10.
The 11-01 D-02 spike never exercised a helper call — it tested tuple-return in isolation
against synthesized expressions. The architectural mismatch only surfaced when 11-04's
per-functional `cargo build -p` first exercised real chunks + real helper calls together.

The quick task **`260515-q01-cse-chunk-arity-cap-12`** (commit `5c379dc25`) empirically
established the four-layer bug structure (MAX_TUPLE_ARITY cap, literal coercion, 1-tuple
`let`, helper-layer concreteness) via `crates/kernels/math/tests/spike_cse_emit_q01.rs`.
Layers 1–3 are fixed in `tools/translate_v2/`; layer 4 is **D-14**'s scope.

**Initial resolution (user decision 2026-05-18, first pass):** Abandon Option A; lock Option C (cast-at-call-site).
**Reconsidered resolution (user decision 2026-05-18, second pass):** **Lock Option A (generic helpers).** Rationale: Option A is the architecturally sound solution — making helpers properly generic is cleaner than accepting cast boilerplate throughout the generated tree. The Phase 2 `_refactor_helper_*` scripts have systematic syntax errors in 11 files, but these can be fixed via improved Python tooling. Timeline is open-ended — quality over speed. This is the right long-term abstraction.

The decisions in this final revision (D-02 locked to A, D-14 updated for Python-tooling approach, D-16 simplified for non-cast emit, AP-1..3 with structural mitigations) are designed to prevent recurrence: **D-02 Option A** targets the right abstraction via improved tooling, not workarounds; **AP-1/AP-2/AP-3** provide structural gates and checks to catch breakage patterns early (entry-gate compile, pre-flight `.cargo/config.toml` check, explicit ban on hand-editing); **AP-6 reframed as part of AP-1**: compile-first entry gate, not compile-after exit gate.

**Plans 11-04..06 are stale and must be regenerated per the Option A decision.** 11-01..03 SUMMARYs survive. The
retroactive 11-04 SUMMARY (D-17) documents the Task 1A landing and the pause. Forward
work starts at 11-05.

**Carry-forward summary:**
- 11-01 SUMMARY ✓ (Wave 0 deliverables — audit tools, baseline, dispatch audit, D-02 isolated spike)
- 11-02 SUMMARY ✓ (emit.py routing-aware launch policy, MAX_TUPLE_ARITY=12 work)
- 11-03 SUMMARY ✓ (D-13 audit + dispatch verification under per-functional subcrates)
- 11-04 partial commit `39eb75f93` ✓ (verify dev-dep narrowing — retroactive SUMMARY in this replan)
- `5c379dc25` (q01 three emit fixes in `tools/translate_v2/`) ✓ — building on, not redoing
- D-14 spike now validates **Option A** (via improved Python tooling; timeline unconstrained for quality)

---

---

## Re-plan Note (2026-05-18 — third session: cleanup + D-18)

Third discuss-phase session. Changes from prior state:

1. **Stale artifacts deleted:** `.continue-here.md` (said "Option C"), `11-06/07/08-PLAN-NEW.md` (Option C plan variants). These contradicted the Option A decision in CONTEXT.md and have been removed.
2. **11-05 status:** COMPLETE (all 38 helpers in 16 files refactored to `<F: Float>`). Syntax errors from automated script are 11-06's first task, not a blocker on 11-05 completion.
3. **D-18 added:** Serena MCP server as primary refactoring tool for 11-06 syntax cleanup. Already configured in `~/.claude.json`. Python tools as fallback.
4. **11-06..08 plans need regeneration** to reflect D-18 tooling and the updated 11-05 starting state. Existing `11-06/07/08-PLAN.md` files are stale (written before 11-05 executed).

**Carry-forward summary (current):**
- 11-01 SUMMARY ✓
- 11-02 SUMMARY ✓
- 11-03 SUMMARY ✓
- 11-04 SUMMARY (retroactive partial) ✓
- 11-05 SUMMARY ✓ — helpers logically complete, syntax errors deferred to 11-06
- 11-06..08: REGENERATE via `/gsd-plan-phase 11`

---

## Re-plan Note (2026-05-18 — fourth session: post-11-06 HALT, A1 locked, f32+f64 test scope)

Fourth discuss-phase session. Triggered by plan 11-06's HALT (commit `75c0f5112`, FAILED SUMMARY): the 11-05 Phase 2 auto-script (`tools/refactor_helpers_generic.py`) wrapped every f64 literal AND every named identifier in `F::new(...)`. CubeCL `Float::new(val: f32)` rejects f64 named constants — 447 × E0308 errors out of 515 total. The plan's "3 syntax categories" model accounted for 7 errors (1.4%); the remaining 508 are architectural.

### What this session changed

1. **New directive: f32 + f64 dual-precision test surface (D-19).** All tests in the project — helper unit tests, per-functional subcrate tests, spike harnesses, parity_phase11.rs, and the 649-functional oracle harness — are parameterized over both `<F = f32>` and `<F = f64>` and exercised under each. F32 elevated to a first-class correctness target at 1e-6 relative (vs f64 oracle widened). Env-gated via `LIBXC_RS_F32=1`. **This locks A1 as the only viable architectural path** — helpers must be generic to support parametric tests at the helper layer.

2. **D-03 amended (D-19a).** F32 is no longer "performance-only, no correctness gate". It is now correctness-gated at 1e-6 relative. The amended policy: f64 is the primary correctness target at 1e-12; f32 is a secondary correctness target at 1e-6 relative, env-gated at test time. `CLAUDE.md` must be updated as part of this phase to reflect both the original D-03 shift and the D-19a amendment.

3. **A1 locked (D-20).** Extends `tools/refactor_helpers_generic.py` (or Serena-MCP-driven equivalent) with cast_from policy: classify every `F::new(IDENT)` site by symbol class (f64 const → `F::cast_from`; f32 const → keep `F::new`; doc-comment / string-literal → revert; non-generic file → revert; `_f64` suffix bug → fix; range op bug → fix; double-wrap bug → fix). Surgical manual fixes per D-23 for known non-script regressions. A2 / C / Hybrid rejected with reasons captured.

4. **3-gate pre-bulk validation sequence (D-22).** Structural mitigation for AP-7 (newly codified). Before the script bulk-runs on the 11 problematic Phase-2 helper files: (1) synthetic-fixture coverage matrix with all known symbol classes, (2) bessel.rs canary (highest F::new count + most diverse symbol classes), (3) chunk → helper integration-boundary spike on mgga_c_b94 at both precisions (compile + parity at f64 1e-12 + parity at f32 1e-6 + idempotency). Only after all three green does bulk run proceed.

5. **AP-7 codified.** "Spike exercises unit boundary instead of integration boundary." The 11-05 spike validated tuple-return + literal-coercion in isolation but never compiled a chunk that called a helper. D-22's Gate 3 enforces integration-boundary spikes going forward.

6. **D-24 — plan regeneration scope.** 11-06 replaced (not amended): pre-flight + surgical revert + script extension + 3-gate sequence + bulk run + three-leg exit gate. 11-07 amended: D-15 entry gate now runs at both precisions. 11-08 amended: per-`-p` sweep includes f32 leg under env-gate; full 649-functional f32 oracle sweep is phase-end deliverable.

### Carry-forward summary (post-fourth-session)

- 11-01 SUMMARY ✓ — Wave 0 deliverables
- 11-02 SUMMARY ✓ — emit.py routing-aware launch policy, MAX_TUPLE_ARITY=12
- 11-03 SUMMARY ✓ — D-13 audit + dispatch verification + 266-subcrate regen
- 11-04 SUMMARY (retroactive partial) ✓ — D-05 verify dev-dep narrowing
- 11-05 SUMMARY ✓ — helpers logically refactored, syntax errors deferred (now superseded by D-22 sequence + D-23 surgical revert)
- 11-06 FAILED SUMMARY ✓ (commit `75c0f5112`) — HALT report; no source edits applied
- 11-06..08: REGENERATE via `/gsd-plan-phase 11` per D-24

### Phase 11 forward gating

Per D-22 + D-24, the next plan-phase + execute-phase cycle MUST honor:

| Phase point | Gate | At what precision |
|---|---|---|
| Pre-flight | `.cargo/config.toml` invariants (AP-2) | n/a |
| Surgical revert | `deferred.rs` reverted; `special.rs:224`, `bessel.rs` type-annotations, `mbrxc.rs:145` fixed | n/a |
| Gate 1 (D-22) | Synthetic fixture compiles after script run | n/a (compile-only) |
| Gate 2 (D-22) | `cargo build -p libxc-kernel-math` after script run on bessel.rs alone | n/a (compile-only) |
| Gate 3 (D-22) | mgga_c_b94 chunk→helper compile + parity + idempotency | **BOTH** f64 (1e-12) AND f32 (1e-6 under `LIBXC_RS_F32=1`) |
| Bulk run | Script transforms remaining 10 helpers | n/a |
| Exit gate (11-06) | `cargo build -p libxc-kernel-math` + spike re-test | BOTH precisions |
| Entry gate (11-07, D-15) | mgga_c_b94 canary post full-tree regen | BOTH precisions |
| Per-`-p` sweep (11-08) | Each routed subcrate `cargo build -p <crate>` | f64 only at sweep; f32 smoke only |
| Phase end | Full 649-functional oracle sweep | BOTH precisions (one-shot, phase-end deliverable) |

---

*Phase: 11-splitter-v2-unified-5k-cap*
*Context gathered: 2026-05-13 · Revised: 2026-05-14 · Re-planned: 2026-05-15 (D-13) · Re-planned: 2026-05-15 (D-14..D-17, second pause) · Re-planned: 2026-05-18 (Option A locked) · Re-planned: 2026-05-18 (D-18 Serena MCP, 11-05 COMPLETE, stale cleanup)*
