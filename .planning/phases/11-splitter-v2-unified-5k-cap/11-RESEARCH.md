# Phase 11: Splitter v2 — Unified Kernels with 5K Line Cap - Research

**Researched:** 2026-05-13
**Domain:** Python AST/code-generation tooling + CubeCL `#[cube]` proc-macro fan-out + Cargo workspace mechanics
**Confidence:** HIGH on the existing-system map, MEDIUM on D-02 tuple-return ABI (cubecl-macros 0.10 parses tuples but no oracle-validated kernel uses them today), HIGH on D-08/D-09 build env (verified verbatim)

## Summary

Phase 11 re-engineers a splitter pipeline whose major gap is **only LDA has a chunked-scratch fallback for single-output expressions** (lines 480–852 of `tools/translate_lda_v2.py`). GGA and MGGA splitters bottom out at the per-output-component level and produce 5K–17K-line single files. There are 237 files >5K across 22 numbered subcrates, with the worst case at `mgga_c_b94/kxc_pol.rs` (16,703 lines). The existing chunked-scratch path uses an `Array<f64>` scratch buffer with `s[idx] = expr` writes — NOT the tuple-return + `<F: Float>` generic ABI specified by D-02, and the existing chunks are emitted at hardcoded `f64`. Adopting D-02 means replacing the entire chunk-helper ABI (4 functions in `translate_lda_v2.py`: `chunk_single_output_split`, `_generate_chunk_helper`, `_generate_chunked_wrapper`, `_build_scratch_replacer`), porting it to `translate_gga.py` and `translate_mgga.py`, and verifying that cubecl-macros 0.10 round-trips tuple-returning `#[cube]` functions (parser supports tuples but no kernel in this codebase exercises them — empirical spike required).

The subcrate collapse is mechanically simple: dispatch and verify never reach into `libxc-kernel-{lda,gga,mgga}-N` directly — they use `libxc_kernel_lda::lda_x::*` (LDA façade re-exports per-functional symbol) or `crate::kernel::mgga::batch17::...` (where `batch17` is a `pub use libxc_kernel_mgga_17 as batch17` re-export from the MGGA façade). The collapse can preserve the `batchN` aliases as actual submodules or eliminate them entirely with a coordinated update of `src/eval/{gga,mgga}_dispatch/*.rs`.

**Primary recommendation:** Build the CSE-aware splitter as a new `tools/translate_v2/` Python package (sharing `kernel_routing.py` only), spike the tuple-return `<F: Float>` ABI on a single small functional first (recommend `lda_x_2d` exc_unpol — known small, well-understood) BEFORE touching r4scan/br89_explicit. Subcrate collapse goes second, after the splitter can hit ≤5K on every file.

## User Constraints (from CONTEXT.md)

### Locked Decisions

- **D-01:** CSE-aware subdivision. Splitter detects common subexpressions and multi-use temporaries in the Maple AST (or post-translation Rust AST) and lifts each into a free `#[cube]` helper. Aligns with `cubecl_macro_fanout_manual.md` §10. Per-statement banding and arbitrary AST-token chunking explicitly rejected.
- **D-02:** Free `#[cube]` functions with explicit value args and tuple returns, generic over `<F: Float>`. Signature shape: `#[cube] fn chunk_NN<F: Float>(args: f64s as F) -> (F, F, ...)`. Each chunk's dependencies visible in its parameter list. Helper structs with `#[cube] impl` blocks and bag-of-floats shared mutable state explicitly rejected (`cubecl_macro_fanout_manual.md` §9, §19, §4).
- **D-03:** Kernel chunks generic over `<F: Float>`. **f64 is the default and sole correctness target** — oracle verification gate runs at f64 only. f32 is launch-time opt-in for performance with no correctness guarantee.
- **D-03a:** `CLAUDE.md` must be updated as part of this phase ("f64 only" → "f64 by default and for oracle gating; f32 opt-in at launch with no correctness gate").
- **D-04 (file layout):** Continue existing `_partNN` suffix convention (`mgga_c_r4scan/lxc_pol_part01.rs` ... `_part04.rs`). Functional entry stub re-exports the assembled function and dispatches into the parts.
- **D-05 (verify gate):** 1e-12 relative error on energy AND all routed derivatives, at f64. `verify/` regression sweep on representative LDA/GGA/MGGA functionals runs after every translation iteration. Bit-exact f64 rejected (CSE introduces named temporaries that may legitimately reorder accumulation). Energy-only relaxed-derivative gate rejected.
- **D-06:** Phase 11 lands BEFORE Phase 10 (workspace modular split).
- **D-07 (RAM ceiling):** Inline executor (no `isolation="worktree"` for cargo-touching work). `cargo` `jobs = 1` enforced via `.cargo/config.toml`; Phase 11 MUST NOT relax via `CARGO_BUILD_JOBS`, `--jobs N`, or editing `.cargo/config.toml`. Read-only researcher / scout subagents permitted.
- **D-08 (RUST_MIN_STACK):** `RUST_MIN_STACK = "67108864"` (64 MB) is load-bearing for `libxc-kernel-math` (br89, mbrxc Brent-method root-finders). Default 8 MB SIGSEGVs deep `#[cube]` proc-macro expansion. CSE-aware subdivision will introduce MORE deeply nested `#[cube]` helpers, so the 64 MB stack is even more load-bearing after Phase 11. MUST NOT remove, reduce below 64 MB, or restore the buggy `2_000_000_000` (≈1.87 GiB) value. MAY raise further if a chunk-graph still SIGSEGVs after splitting.
- **D-09 (cargo config is source of truth):** Phase 11 reads build env from `.cargo/config.toml`, not from agent prompts or memory. `[build] jobs = 1`, `[build] target-dir = "/home/user/Documents/workspace/libxc_rs/.cache/cargo-target"`, `[env] RUST_MIN_STACK = "67108864"`. Iteration loops MUST NOT clean target dir; sccache must remain enabled; incremental compilation in Cargo.toml profiles MUST stay disabled (incompatible with sccache).
- **D-LOCK-A:** Unification scope = collapse per-family subcrates ONLY. Multiple files per functional are permitted; `_partNN` per D-04 is the convention.
- **D-LOCK-B:** 5,000-line cap is HARD. Splitter extended (D-01) until it can hit the cap on every functional including the 8–15K single-output leaves.
- **D-LOCK-C:** Supersedes `.planning/quick/260513-8nv-update-splitter-tool-enforce-3000-line-c` (3,000-line target abandoned; directory empty — no artifacts to discard).
- **D-LOCK-D:** Iteration is required. Pipeline must be re-run until both invariants hold AND `cargo build --workspace` passes AND D-05 oracle gate passes. Idempotency is a success criterion — running pipeline twice must produce no diff.

### Claude's Discretion

- Internal structure of the CSE pass (Maple AST walker vs post-translation Rust AST walker vs Python-side intermediate IR).
- Whether to extend existing `tools/translate_*.py` family in place or fork a `tools/translate_v2/` tree.
- Exact migration path for the existing 22 numbered subcrates (in-place rename + content merge vs new tree + cutover).
- Whether to add a `tools/audit_kernel_size.py` that fails CI when a kernel file exceeds 5K (recommended but not locked).
- Whether to retain existing `tools/split_oversized_{kernel,mgga}.py` / `tools/rebatch_mgga.py` / `tools/split_mgga_7_kcis.py` helpers as scaffolding or fold them into the unified pipeline.

### Deferred Ideas (OUT OF SCOPE)

- f32 oracle gate at relaxed tolerance.
- CI gate enforcing the 5K cap (`tools/audit_kernel_size.py`) — natural follow-up.
- Workspace boundary refactor (Phase 10).
- Promoting `#[cube]` traits in kernel chunks (manual §9 warns).
- Bessel I0/I1 implementation for `mgga_x_2d_prp10` (libxc id 211, pre-existing deferral from quick task 260510-q02).

## Project Constraints (from CLAUDE.md)

- **f64 only; no silent f32 fallback** — DEPRECATED as of D-03/D-03a; phase must update CLAUDE.md to reflect "f64 by default and for oracle gating; f32 opt-in at launch with no correctness gate." Until that update lands, this phase is the deliberate exception.
- **Maple2c formula translations must preserve floating-point operation order for bit-level equivalence** — RELAXED by D-05 (CSE-aware subdivision introduces named temporaries that may reorder accumulation; gate is now 1e-12 relative error, not bit-exact).
- **Energy relative error <= 10^-12 vs libxc oracle** — UNCHANGED. D-05 hardens this to also cover routed derivatives (vrho, fxc, kxc, lxc) at the same tolerance.
- **GSD workflow enforcement** — Phase 11 work runs through `/gsd-execute-phase`, no direct edits.
- **`ctx7` for library/framework documentation** — used during this research for CubeCL 0.10 doc lookups.

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Maple AST parsing | Python tooling (tools/) | — | Existing `extract_functions` + `parse_function_body` regex parsers; not a Rust concern |
| CSE detection | Python tooling | — | Operates on Maple-emitted compute lines (already stripped by `parse_function_body`) before translation |
| `#[cube]` Rust emission | Python tooling | — | Existing `generate_function`, `_generate_chunk_helper`, `_generate_chunked_wrapper` |
| `#[cube]` macro expansion | rustc proc-macro (cubecl-macros 0.10) | — | Phase 11 cannot change this — must work within its constraints |
| Subcrate Cargo wiring | Cargo workspace (`Cargo.toml`) | — | Mechanical edit + `pub use` re-export tweaks |
| Family-level façade re-exports | `crates/kernels/{lda,gga,mgga}/src/lib.rs` | — | Already exists per family; collapse means folding child content here |
| Dispatch-side import paths | `src/eval/{gga,mgga}_dispatch/*.rs` | — | References `crate::kernel::{family}::batchN::...`; collapse must either preserve `batchN` as alias modules or update these files |
| Oracle verification | `verify/` crate | — | Already imports via family façade only; survives collapse without changes |
| Translator routing decision (`#[cube]` vs `#[cube(launch_unchecked)]`) | `tools/kernel_routing.py` | — | Single source of truth shared by translators and `demote_unrouted_kernels.py` |

## Existing System Map

### Splitter Pipeline (current state)

```
libxc-master/maple/*.mpl          (Maple source, 48 files)
            │
            ▼
libxc-master/src/maple2c/         (C output of Maple, the actual translator input)
   {lda_exc,lda_vxc,gga_exc,gga_vxc,mgga_exc,mgga_vxc}/<func>.c
            │
            ▼
tools/regen_phase09.py            (per-functional discovery + atomic dir replace)
            │
            ▼
tools/translate_{lda_v2,gga,mgga}.py    (per-family Maple→Rust translator)
            │
            ├─► extract_functions / parse_function_body  (regex parser)
            ├─► scan_param_accesses                       (params->X discovery)
            ├─► detect_imports                            (math primitive imports)
            ├─► translate_expr (per-line)                 (C→Rust expression translation)
            ├─► generate_function                         (single-#[cube] emitter)
            └─► IF est > SPLIT_THRESHOLD (6000):
                    split_by_output_array                 (per-output transitive dep cut)
                    merge_small_splits (cap-suffix at 60ch)
                    IF still > SPLIT_THRESHOLD AND len(sub_outputs) > 1:
                        split per output component
                    IF STILL > SPLIT_THRESHOLD (single output):
                        chunk_single_output_split  ★LDA ONLY★  (Array<f64> scratch chunks)
            │
            ▼
crates/kernels/{lda-N,gga-N,mgga-N}/src/<func>/{level}_{spin}[_partNN].rs
            │
            ▼
tools/split_oversized_{kernel,mgga}.py    (post-emit per-file bin-pack into N letter-suffix subcrates)
tools/rebatch_mgga.py                      (functional-granularity bin-pack into N numbered subcrates)
            │
            ▼
crates/kernels/{lda,gga,mgga}/src/lib.rs   (family façade — re-exports child subcrates)
```

### Tools Inventory (line counts verified)

| File | Lines | Role | Status |
|------|------:|------|--------|
| `tools/translate_lda_v2.py` | 1,643 | LDA translator + chunked-scratch path | Has chunked path at 480-852; D-02 will replace |
| `tools/translate_gga.py` | 1,336 | GGA translator | NO chunked path — bottoms out at output-component split |
| `tools/translate_mgga.py` | 1,316 | MGGA translator | NO chunked path — bottoms out at output-component split |
| `tools/maple_to_kernels.py` | 246 | Thin orchestrator over `regen_phase09.py` + `split_*.py` | Driver-level CLI; defaults `DEFAULT_SPLIT_THRESHOLD = 100_000` and `DEFAULT_TARGET_MAX = 500_000` are stale (not honored — translator constants win) |
| `tools/split_oversized_kernel.py` | 302 | Per-file bin-pack within oversized functional → letter-suffix subcrates | Becomes irrelevant after D-LOCK-A subcrate collapse |
| `tools/split_oversized_mgga.py` | 235 | MGGA-specific oversized splitter (--target-max wired post-260510-q01) | Becomes irrelevant after collapse; latent multi-functional `rmtree` bug noted in 260510-q01 SUMMARY |
| `tools/split_mgga_7_kcis.py` | 218 | One-off splitter for mgga-7's kcis | Special-case; deletable post-collapse |
| `tools/rebatch_mgga.py` | 417 | First-fit-decreasing bin-pack for MGGA functionals → N numbered subcrates | Becomes irrelevant after collapse |
| `tools/shrink_part_fanout.py` | 101 | Demotes split-part `#[cube(launch_unchecked)]` → `#[cube]` | Already integrated into translators (per 260512-q01); deletable |
| `tools/kernel_routing.py` | ~220 | Single source of truth for "is this functional routed by `<X>Functional::from_id`?" | Translators import this; KEEP |
| `tools/translators/` | empty dir | Reserved namespace for future translator package | Available for D-discretion `tools/translate_v2/` |

### Current Splitter Decision Tree (LDA, the only one with chunked-scratch)

[VERIFIED: `tools/translate_lda_v2.py` lines 1116-1241]

```
estimate_function_lines(compute, outputs)
  ├─ ≤ 6000  → emit single `<level>_<spin>.rs`
  └─ > 6000  → split_by_output_array
                 ├─ each split ≤ 6000 → emit `<level>_<spin>_part{idx}_{field}.rs`
                 └─ split > 6000 with multi outputs → split per output component
                        ├─ ≤ 6000 → emit `<level>_<spin>_part{idx}_{field}_{component}.rs`
                        └─ > 6000 (single output) → chunk_single_output_split
                                ├─ wrapper file: `<level>_<spin>_part{idx}_<suffix>.rs`
                                └─ chunk files:  `<level>_<spin>_part{idx}_<suffix>_chunkN.rs`
                                                  (each chunk writes to s[idx] in shared
                                                  Array<f64> scratch; NOT generic <F: Float>;
                                                  NOT tuple-returning)
```

### Current chunked-scratch ABI (will be replaced by D-02)

Verified at `tools/translate_lda_v2.py` lines 571-702. Each chunk:

```rust
#[cube]
pub fn <func>_<level>_<spin>_part<N>_<suffix>_chunk<K>(
    rho: &Array<f64>,
    s: &mut Array<f64>,           // shared mutable scratch — VIOLATES D-02 "no bag-of-floats shared state"
    param_X: f64, param_Y: f64,   // hardcoded f64 — VIOLATES D-03 "<F: Float>"
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    let rho0 = rho[ip * 2]; let rho1 = rho[ip * 2 + 1];
    s[42usize] = expr_using_s_indices;
    s[43usize] = ...;
}
```

Wrapper allocates `let mut s = Array::<f64>::new(N_VARS);` and calls each chunk in sequence, then writes `out_field[ip * dim + comp] += s[idx_of_output_var];`. This is exactly what `cubecl_macro_fanout_manual.md` §19 rule 7 ("Derive macros only on true CubeCL boundary types") and §10 ("not every expression-level helper") tolerate AS LONG AS the chunks aren't trivially small — but D-02 explicitly rejects the shared-mutable-scratch shape in favor of free functions with explicit value args + tuple returns.

### Numbered Subcrate Inventory

[VERIFIED: `find crates/kernels -maxdepth 1 -type d`]

| Family | Numbered subcrates | Family façade (target) |
|--------|-------------------|------------------------|
| LDA | `lda-1`, `lda-2` | `crates/kernels/lda/` |
| GGA | `gga-1` ... `gga-8` | `crates/kernels/gga/` |
| MGGA | `mgga-1`, `mgga-2`, ..., `mgga-7`, `mgga-8a`, `mgga-8b`, `mgga-9a`, `mgga-9b`, `mgga-10`, `mgga-11a`, `mgga-11b`, `mgga-12`, `mgga-13`, `mgga-14` | `crates/kernels/mgga/` |
| **Total** | **22 numbered subcrates** | **3 façade crates** |

Plus `crates/kernels/math/` (out of scope — shared math primitives).

### Family Façade Re-export Patterns (DIVERGENT)

[VERIFIED: read `crates/kernels/{lda,gga,mgga}/src/lib.rs` end-to-end]

- **LDA** uses **per-functional** re-exports: `pub use libxc_kernel_lda_1::lda_x;` (54 lines, 41 functionals). Consumers see `libxc_kernel_lda::lda_x::*`.
- **GGA** uses **subcrate-as-batch** re-exports: `pub use libxc_kernel_gga_1 as batch1;` (19 lines, 8 batches). Consumers see `libxc_kernel_gga::batch1::<func>::*`.
- **MGGA** uses **subcrate-as-batch** re-exports: `pub use libxc_kernel_mgga_1 as batch1;` (30 lines, 17 batches with letter suffixes for the post-q01 split).

This divergence is the central wrinkle in D-LOCK-A collapse — see "Sub-crate Collapse Mechanics" below.

## CubeCL Macro Fan-out Manual — Operational Summary

The five sections explicitly cited in CONTEXT.md, translated into operational rules for the planner.

### §3 ("Keep the CubeCL expansion surface as small as possible") [CITED: docs/manual/Cubecl/cubecl_macro_fanout_manual.md §3]

**Operational rules:**
- Total `#[cube(launch)]` / `#[cube(launch_unchecked)]` count is a fan-out budget — but **see D-13 (CONTEXT.md): P11-INV-5's original flat "≤22/23 count" form was unsatisfiable against the D-10b dispatch design and has been revised (2026-05-15).** The pre-Phase-11 "22, all in `crates/kernels/math/`" figure was measured on the numbered-subcrate tree whose dispatch layer never compiled (Blocker B1) — it was never a working reference. Under the D-10 per-functional-subcrate design, every **routed** functional has one `#[cube(launch_unchecked)]` entry kernel per output module — the dispatch macros `ten_arm_dispatch_gga!` / `mgga_zero_scalar_unpol_dispatch!`, preserved verbatim per D-10b, call `.launch_unchecked()` per `(functional × output)`, so ~168 routed × ~10 outputs ≈ 1677 launchables. Per D-13 this is **accepted**: the manual §5/§19 fan-out cost is per-compilation-unit, and per-functional subcrates isolate each subcrate's ~10 launch wrappers. The revised P11-INV-5 / `audit_cube_launch.sh` budget: one launch wrapper per routed `(functional, output)`, no **unrouted** kernel launchable, `crates/kernels/math/` ≤22 — NOT a flat count.
- Plain `#[cube]` count: **3,911 in `crates/kernels/`** (verified via `grep -h '^\s*#\[cube\]'`). Phase 11 will INCREASE this number — that's expected and permitted. The constraint is on `(launch)` only.
- Helper functions are unbounded — adding 5,000 new `#[cube]` chunk helpers does not violate §3 as long as none of them are `(launch)`.

### §4 (`#[cube]` vs `#[cube(launch)]` choice) [CITED: §4.3]

**Operational rules:**
- Split-helper chunks emitted by Phase 11 MUST be plain `#[cube]`. The translators already enforce this (`is_split_helper = fn_suffix.startswith('_part')` → `#[cube]`).
- Wrapper functions that re-assemble chunked output MUST also be plain `#[cube]` when the wrapper IS itself a split helper (`_partNN`); they are `#[cube(launch_unchecked)]` ONLY when they are the per-(level, spin) entry kernel for a routed functional.
- The `kernel_routing.py` "is routed?" decision is the second gate: unrouted functionals get `#[cube]` even at the entry-kernel level (per `260512-q01` change). This already lands the §4 anti-pattern fix.

### §9 (`#[cube]` traits and impl blocks) [CITED: §9]

**Operational rules:**
- D-02 explicitly rejects `#[cube] impl` blocks for chunk decomposition. Phase 11 chunks are FREE FUNCTIONS only.
- The existing chunked-scratch path (LDA only) does NOT use `#[cube] impl` — it uses free functions sharing a mutable `Array<f64>`. The change in D-02 is removing the shared mutable state, not adding impl blocks.

### §10 ("Avoid Too Many Tiny `#[cube]` Helpers" — break apart algorithmic stages, not every expression) [CITED: §10]

**Operational rules:**
- This is the heart of D-01. Two interpretations and the planner must pick one:
  1. **Coarse-grained CSE:** detect named temporaries used in ≥N (default: 5) downstream lines AND whose def-use-chain spans ≥M (default: 50) compute lines. Lift those to a helper. Skip every shorter-lived temporary.
  2. **Fine-grained CSE:** detect every multi-use temporary regardless of size. Lift all of them. Risk: produces hundreds of 5-line helpers, which §10 explicitly warns against.
- Recommendation: **start with coarse-grained**, with `min_uses=5` and `min_chain_length=50` as initial knobs. The 8–15K leaves are dominated by long deep dependency chains where many temporaries each accumulate 50+ uses; coarse-grained CSE should be sufficient.
- §10's positive example shows `load_and_preprocess` and `reduce_local` as helpers — **algorithmic stages, not arbitrary cuts**. Phase 11's CSE detector should aim for similar granularity.

### §19 (Recommended low-fan-out architecture) [CITED: §19]

**Operational rules:**
- Rule 1: "One public launchable kernel per real algorithm entry point" — current state already conforms (per-(level, spin) per functional; nothing else is launchable).
- Rule 2: "A small set of meaningful free `#[cube]` helper functions" — Phase 11 expands this set; it's already meaningful per functional.
- Rule 3: "Generic numeric abstraction using `Float`, `Int`, or `Numeric`" — this is D-03. **Currently NOT met anywhere in libxc_rs** (every existing `#[cube]` is hardcoded `f64`). Phase 11 introduces the first `<F: Float>` kernels in this codebase. Empirical spike strongly recommended before bulk rollout.
- Rule 4 (`#[define]` for launch-time type selection): NOT in scope for Phase 11 (deferred to f32-promotion future phase per D-03 deferred bucket).
- Rule 5 (`#[comptime]`): NOT used today and not introduced by Phase 11 (the chunked-scratch wrapper uses runtime `Array<f64>::new(scratch_size)` allocation; converting `scratch_size` to `#[comptime]` would be an optimization, not a correctness change).
- Rules 6–10: already met or out of scope.

## CSE Subdivision Strategy

Three concrete options for HOW to implement D-01. The planner must pick one.

### Option A: Maple AST walker (richest semantic info, hardest to implement)

**What:** Parse the Maple `.mpl` source files in `libxc-master/maple/` directly. Extract the symbolic expression tree (Maple's native sequence-of-assignments form). Apply CSE detection at the Maple expression level. Emit the chunked Rust output in one pass.

**Pros:**
- Maple already names temporaries idiomatically (`t1`, `t2`, ...) at meaningful breakpoints.
- Multi-use temporaries are visible at the Maple level via `=` re-binding analysis.
- Could reuse Maple's own simplification policy.

**Cons:**
- No off-the-shelf Python parser for Maple — would have to write one. Maple syntax has corners (procedure calls, packages, `if-then-elif-fi`, `seq`, `sum`).
- Skips libxc's own maple2c step, which means losing all of libxc's already-applied codegen tweaks (sign normalization, common-factor extraction).
- Diverges from upstream libxc's pipeline; any future libxc Maple-side change would require re-tooling our parser.
- Verdict: **REJECT** unless a future phase commits to a full Maple-frontend rewrite.

### Option B: Post-translation Rust AST walker via `syn` (cleanest, but requires Rust roundtrip)

**What:** Run the existing `translate_*.py` pipelines unchanged to produce 8–15K-line `#[cube]` files. Parse those files with the Rust `syn` crate (in a small helper Rust binary or via `tree-sitter-rust` in Python). Walk the AST, identify common subexpressions across `let tN = ...;` bindings via syntactic equality + def-use chain analysis. Rewrite into chunked form.

**Pros:**
- Operates on real, verified Rust code. CSE detection sees exactly what cubecl-macros sees.
- `syn` is mature; def-use analysis is straightforward via visitor pattern.
- Decoupled from any Maple/maple2c changes upstream.

**Cons:**
- Requires a new Rust binary (or JS-via-tree-sitter dependency in Python) — adds tooling surface.
- Two-pass pipeline: translator emits oversized file, then Rust pass rewrites it. Idempotency contract gets murkier (D-LOCK-D).
- Per-file rewrite must preserve operation order EXACTLY as the original — `syn` reads the AST faithfully but care needed when emitting.
- Verdict: **STRONG OPTION** if the team is comfortable adding a small Rust tool to `tools/`.

### Option C: Operate on the C compute_lines list before Rust emission (RECOMMENDED)

**What:** The current translators already extract `compute_lines: List[str]` from the C source via `parse_function_body`. This list is `["t1 = 0.5 * rho[0];", "t2 = pow(t1, 1.0/3.0);", ...]` — sequence-of-assignments at the C level, with all temporaries already named by Maple. Run CSE detection on this list (Python regex + dict-based def-use analysis is sufficient — see existing `build_dependency_graph` at `translate_lda_v2.py` line 370). Group def-use chains into chunks based on shared upstream temporaries. Emit Rust with each chunk as a free `#[cube]` function consuming N args (the upstream temporaries it depends on) and returning M values (the downstream temporaries other chunks need).

**Pros:**
- Stays inside the existing Python pipeline. No new language/dependency.
- The current `build_dependency_graph` and `transitive_deps` functions already do 60% of this work; extending them to compute "min-cut points in the dep DAG" is incremental.
- One-pass: translator emits chunked output directly. Idempotent by construction (deterministic Python on deterministic input).
- Each chunk's args/returns are visible in Python before Rust emission — easy to enforce arity caps and verify per-chunk size estimates against the 5K cap.
- Operation-order preservation is trivial: Python iterates `compute_lines` in order, just as it does today.

**Cons:**
- C-level analysis can't see post-translate_expr identities (e.g., two C lines that look different but produce identical Rust). Mitigation: dedup pass on translated Rust strings within each chunk.
- Must handle bool intermediates specially (`t45 = t44 <= zeta_threshold`) — they can't pass through `f64` tuple returns. This is already understood in the LDA chunked-scratch path (lines 532-548).

**Recommendation: Option C.** Rationale: it inherits the existing pipeline's verified parser, fits the project's Python tooling convention, and the dep-graph foundation is already written. Estimated implementation surface: ~600 lines of new Python in a `tools/translate_v2/cse.py` module (compute the def-use DAG, find min-cut points, partition into chunks, plan tuple signatures). Plus ~200 lines per family translator to wire CSE chunking into `translate_one_function` after `split_by_output_array` + `merge_small_splits` and BEFORE the per-output-component fallback.

### CSE Detection Heuristic (concrete proposal)

Given `compute_lines` list and a max chunk size `chunk_max_lines` (~4500 to leave room for boilerplate vs the 5K hard cap):

1. Build def-use DAG: `var → set of vars referenced` (already done by `build_dependency_graph`).
2. Compute reverse-dep counts: how many later lines reference each `var`.
3. Walk lines in order; accumulate into the current chunk until either:
   - chunk size hits `chunk_max_lines`, OR
   - a "natural breakpoint" is reached: a `var` with reverse-dep-count ≥ N (default 5) whose def-use chain extends past the chunk boundary.
4. At the breakpoint, close the chunk. Compute its INPUTS (vars referenced inside but defined outside, in chunk order — these become tuple args) and OUTPUTS (vars defined inside that have ≥1 use after the chunk closes — these become tuple returns).
5. Emit the chunk as `#[cube] fn chunk_<level>_<spin>_<idx><F: Float>(args: F, ...) -> (F, F, ...)`.
6. The wrapper destructures the returned tuple into named bindings: `let (t89, t142, t201) = chunk_3::<F>(t12, t34, t78);` — these named bindings then feed the next chunk.

Tuple arity will need a cap (recommend ≤16 inputs, ≤16 outputs). When exceeded, force a chunk split EARLIER. CONFIDENCE: MEDIUM — the cap value is tunable and may need to drop to 8 if cubecl-macros has trouble with deeply-arity tuples.

## Sub-crate Collapse Mechanics

### What references the numbered subcrates today?

Verified via `grep -hr "libxc_kernel_(gga|mgga|lda)_[0-9]" src/ verify/` (excluding worktree caches):

| Site | Today | After collapse |
|------|-------|----------------|
| `Cargo.toml` (workspace `default-members`) | Lists all 22 numbered + 3 façades + `math` | Lists 3 façades + `math` |
| `Cargo.toml` (root deps) | `libxc-kernel-lda-1`, `libxc-kernel-lda-2`, `libxc-kernel-mgga-1` ... `libxc-kernel-mgga-14` | Drop all `_-N` deps |
| `crates/kernels/{family}/Cargo.toml` | Numbered subcrate has `cubecl + libxc-kernel-math`; family façade `Cargo.toml` re-exports them | Family façade owns `cubecl + libxc-kernel-math` directly |
| `crates/kernels/{family}/src/lib.rs` | LDA: `pub use libxc_kernel_lda_1::lda_x;` <br> GGA: `pub use libxc_kernel_gga_1 as batch1;` <br> MGGA: `pub use libxc_kernel_mgga_1 as batch1;` | LDA: `pub mod lda_x;` (real submodule) <br> GGA/MGGA: keep `pub mod batchN;` AS REAL submodule names OR refactor dispatch (see below) |
| `src/eval/gga_dispatch/*.rs` | `crate::kernel::gga::batchN::<func>::...` | Either preserve `batchN` as alias submodules OR rewrite to direct functional path |
| `src/eval/mgga_dispatch/*.rs` | `crate::kernel::mgga::batch17::<func>::...` | Same options |
| `verify/tests/*.rs` | `libxc_kernel_lda::*`, `libxc_kernel_mgga::*` (family façade only) | UNCHANGED — already insulated |
| `src/lib.rs` | `pub use libxc_kernel_lda as lda;` etc | UNCHANGED |
| `src/eval/*` (non-dispatch) | One mention: `libxc_kernel_mgga::deferred::is_deferred` | UNCHANGED (lives in family façade) |

### Two collapse strategies (planner decides)

**Strategy 1: Identity submodules ("`batch1` becomes a real `mod batch1;`")**

- LDA: trivial — already uses per-functional re-exports; just expand each `pub use libxc_kernel_lda_1::<func>;` into `pub mod <func>;` and physically `mv crates/kernels/lda-1/src/<func> crates/kernels/lda/src/<func>`.
- GGA/MGGA: keep `batch1`, `batch2`, ... as real submodules holding the functionals. So the new layout is `crates/kernels/gga/src/batch1/<func>/...`. Dispatch's `crate::kernel::gga::batchN::<func>::...` paths CONTINUE TO WORK without edit.
- Pros: smallest-touch change to dispatch, idempotent commit.
- Cons: keeps a vestigial "batch" naming convention that has no real meaning post-collapse.

**Strategy 2: Flat layout, retarget dispatch**

- All families: `crates/kernels/{family}/src/<func>/...` (per-functional submodules at top level).
- Update every `src/eval/{gga,mgga}_dispatch/batch*.rs` to drop the `::batchN::` segment from import paths.
- Pros: clean post-collapse layout matching LDA's.
- Cons: touches `src/eval/{gga,mgga}_dispatch/*.rs` (auto-generated by `tools/generate_gga_dispatch.py` per the file header — must regen, not hand-edit).

**Recommendation:** Strategy 1 first (achieves D-LOCK-A invariant with minimal blast radius). Strategy 2 is a follow-up cleanup that fits naturally in Phase 10's workspace refactor.

### Tooling needed

- `tools/collapse_subcrates.py` — new helper that:
  1. For each family, walks `crates/kernels/<family>-N/src/<funcdir-or-batchdir>/`, moves directories into `crates/kernels/<family>/src/`, deduplicates name collisions (none expected per inventory), updates `crates/kernels/<family>/src/lib.rs`.
  2. For each family, deletes `crates/kernels/<family>-N/` directories.
  3. Updates root `Cargo.toml`: remove all `libxc-kernel-{family}-N = { path = ... }` deps and corresponding `default-members` entries; ensure family façade still has correct `cubecl + libxc-kernel-math` deps (move from one of the children).
- The collapse must be ATOMIC per family — partial collapses leave the workspace unbuildable. Recommend doing one family at a time as separate commits, with `cargo build --workspace` (under D-07/D-08/D-09 envelope) verifying after each family.

## Pathological Functional Inventory

Verified: `find crates/kernels -name '*.rs' -exec wc -l {} + | awk '$1 > 5000'` → **237 files** total.

### Distribution by file size

| Bucket | Count |
|--------|------:|
| 5K – 8K lines | 182 |
| 8K – 10K lines | 37 |
| 10K – 12K lines | 7 |
| > 12K lines | 11 |

### Top 15 worst offenders (all >9K)

| Lines | Path |
|-----:|------|
| 16,703 | `mgga-2/src/mgga_c_b94/kxc_pol.rs` |
| 16,138 | `mgga-4/src/mgga_c_kcisk/lxc_pol_part15.rs` |
| 15,378 | `mgga-6/src/mgga_c_ccalda/lxc_pol.rs` |
| 14,127 | `mgga-3/src/mgga_c_kcis/lxc_pol_part13.rs` |
| 13,913 | `mgga-4/src/mgga_c_kcisk/lxc_pol_part16.rs` |
| 13,719 | `mgga-4/src/mgga_c_kcisk/lxc_pol_part14.rs` |
| 13,238 | `mgga-2/src/mgga_c_rppscan/lxc_pol.rs` |
| 12,648 | `mgga-8b/src/mgga_c_revtpss/lxc_pol_part20.rs` |
| 12,598 | `mgga-2/src/mgga_c_scan/lxc_pol.rs` |
| 12,596 | `mgga-2/src/mgga_c_rregtm/lxc_pol.rs` |
| 12,164 | `mgga-3/src/mgga_c_kcis/lxc_pol_part14.rs` |
| 11,988 | `mgga-3/src/mgga_c_kcis/lxc_pol_part12.rs` |
| 11,177 | `gga-1/src/gga_c_acgga/lxc_pol.rs` |
| 10,955 | `mgga-8b/src/mgga_c_revtpss/lxc_pol_part21.rs` |
| 10,906 | `gga-3/src/gga_c_ft97/lxc_pol_part12_v4rho4_2.rs` |

### Per-functional (worst aggregate)

| Functional | Files >5K |
|------------|----------:|
| `gga_c_gapc` | 31 |
| `mgga_c_revtpss` | 30 |
| `mgga_c_tpssloc` | 25 |
| `mgga_c_kcisk` | 13 |
| `mgga_c_kcis` | 13 |
| `gga_c_pbe_erf_gws` | 10 |
| `gga_c_gaploc` | 10 |
| `mgga_c_tpss` | 9 |
| `mgga_c_r2scan` | 9 |
| `mgga_c_rmggac` | 8 |

**Observation:** All worst offenders are `lxc_pol` (4th-derivative polarized). They are the densest part of the codebase. Phase 11's chunked-CSE pass MUST work on these or D-LOCK-B fails.

**Cross-check:** The two functionals where the existing chunked-scratch path (LDA only) already runs successfully:
- `lda_xc_ksdt/lxc_pol_part5_v4rho4_1` — wrapper at 65 lines, chunk0 5,995 lines, chunk1 2,281 lines. **chunk0 is just under 6K (current threshold) but EXCEEDS 5K (Phase 11 target).** The chunk size budget computation at `translate_lda_v2.py:780-793` uses `chunk_max_lines = SPLIT_THRESHOLD - chunk_boilerplate`; lowering SPLIT_THRESHOLD to 5000 should produce chunks ≤5K with the same algorithm, but the chunk0 in this example is one line past the natural breakpoint and would need re-chunking. Empirical confirmation needed during Phase 11 plan execution.

### What makes them big

Reading r4scan's `lxc_pol_part5` and br89_explicit's `lxc_pol_part5` (sampled) — the body shape is hundreds of `let tNNN = ...;` bindings, where tNNN may reference 3-8 prior temporaries. Maple has done aggressive CSE already at the C level; Phase 11's CSE detector must operate on what Maple emitted, not "find more CSE." The job is partitioning the existing dep DAG into ~5K-line chunks while preserving exact eval order.

## Build & Verification Environment

### `.cargo/config.toml` (verified verbatim, NOT to be relaxed per D-08/D-09)

```toml
# Build configuration for CubeCL kernel crates.
[build]
jobs = 1
target-dir = "/home/user/Documents/workspace/libxc_rs/.cache/cargo-target"

[env]
# Inlined Brent-method root-finders in libxc-kernel-math (br89, mbrxc) recurse
# deep through the CubeCL `#[cube]` proc-macro expansion. The default 8MB
# rustc stack overflows (SIGSEGV); 64MB clears it with margin.
RUST_MIN_STACK = "67108864"
```

### Cargo.toml profile settings (verified, NOT to be relaxed)

```toml
[profile.dev]
debug = 0
codegen-units = 2
incremental = false  # MUST stay false — incompatible with sccache per .cargo/config.toml header

[profile.dev.build-override]
opt-level = 3        # Optimize proc-macros (cubecl-macros, syn, quote) — repaid many times
codegen-units = 2
debug = false

[profile.release]
debug = 0
incremental = false
codegen-units = 2
```

### Current target dir state (verified)

`/home/user/Documents/workspace/libxc_rs/.cache/cargo-target` is **15 GB**. Iteration loops MUST NOT clean this directory (per D-09); incremental rebuilds against this cache are the design.

### Pre-phase baseline metrics (verified)

| Metric | Value | Source of truth |
|--------|-------|-----------------|
| `#[cube(launch_unchecked)]` count in `crates/kernels/` (pre-Phase-11) | **22** — **NOT a Phase-11 target; see D-13** | `find crates/kernels -name '*.rs' \| xargs grep -h '#\[cube(launch'` |
| All 22 are in | `crates/kernels/math/` | `dft_quantities.rs:4`, `erf.rs:2`, `piecewise.rs:2`, `polynomials.rs:2`, `powers.rs:9`, `spin.rs:3`. **NOTE (2026-05-15):** measured on the pre-collapse tree whose dispatch never compiled. The Phase-11 P11-INV-5 target is the **D-13 per-design budget** (one launch wrapper per routed `(functional, output)`; no unrouted kernel launchable; `math/` ≤22), NOT a flat count. |
| Plain `#[cube]` count in `crates/kernels/` | **3,911** | `find crates/kernels -name '*.rs' \| xargs grep -h '^\s*#\[cube\]'` |
| Files >5K lines (Phase 11 target = 0) | **237** | `find crates/kernels -name '*.rs' -exec wc -l {} + \| awk '$1 > 5000' \| wc -l` |
| Largest file (Phase 11 target = ≤5000) | **16,703 lines** | `mgga-2/src/mgga_c_b94/kxc_pol.rs` |
| Numbered subcrates (Phase 11 target = 0) | **22** | `lda-{1,2}`, `gga-{1..8}`, `mgga-{1..7,8a,8b,9a,9b,10,11a,11b,12,13,14}` |
| Family façade crates (Phase 11 target = 3, unchanged) | **3** | `crates/kernels/{lda,gga,mgga}/` |

### Verification gate (D-05)

The relevant verify harness file is `verify/tests/parity_phase09.rs` (already written for Phase 9). It enforces strict 1e-12 relative error across all derivative orders for 25 deferred GGA functionals + MGGA non-regression spot-check. The pattern to extend for Phase 11:

```rust
const STRICT_TOL: f64 = 1e-12;
const REL_FLOOR: f64 = 1e-30;
```

The harness uses `dispatch_gga`/`dispatch_mgga` and compares against `oracle_gga_all`/`oracle_mgga_all` (libxc 7.0.0 via `libxc-sys`). Skip semantics: only `UnsupportedFunctional` or `UnsupportedDerivativeOrder` errors permitted; every skipped tuple printed for audit. Phase 11's gate would extend this list to cover every functional touched by re-translation.

**Verify command** (D-09 compliant):
```bash
cargo test -p libxc_rs-verify --test parity_phase09 -- --test-threads=1 --nocapture
```

`--test-threads=1` is mandatory under the project's RAM constraint — running tests in parallel would multiply compute-client mutex contention and inflate RSS.

**Estimated wall time:** Per the `260510-q01` measurement (mgga-1 alone: 23.5 min, 22.7 GB peak), a full workspace re-verify is estimated 2–4 hours wall-clock on the constrained machine. Iteration economics dictate that Phase 11 plans either (a) limit verify scope to a representative subset (e.g., 5–10 functionals covering each family + each derivative order + each spin), or (b) batch translation iterations and verify only at planned checkpoints.

### CubeCL macro fan-out audit script (recommended)

```bash
# tools/audit_cube_launch.sh (proposed, ~10 lines)
find crates/kernels -name '*.rs' -print0 | \
    xargs -0 grep -h '#\[cube(launch' | \
    sort | uniq -c
# Expected baseline: 22 launch_unchecked
# Phase 11 invariant: count must NOT increase
```

## Idempotency Contract

**Success criterion 6: "Pipeline is idempotent: running it twice produces no diff."**

### What is non-deterministic in the current pipeline?

Based on review of `translate_lda_v2.py`, `translate_gga.py`, `translate_mgga.py`, `split_oversized_kernel.py`:

1. **Set iteration order** — the param-access scanner uses `set` and sorts (`sorted(accesses, key=lambda a: (a.field, a.indices))`). Sorted output → deterministic.
2. **Dict iteration order** — Python 3.7+ guarantees insertion order. Translators rely on this. Deterministic.
3. **Filename collision suffixes** — `_capped_merge_suffix` deterministically truncates with `_etc`. Deterministic.
4. **`os.listdir` ordering** — `split_oversized_kernel.py:39` uses `sorted(os.listdir(...))`. Deterministic.
5. **`bin_pack` (in `split_oversized_kernel.py:89`)** — sorts files by `-x[1]` (reverse line count), then bin-sorts by `sorted(x[0] for x in b)[0]`. Deterministic.
6. **CSE chunk-id assignment (Phase 11 NEW)** — must use sequence index in compute_lines order, not hash-based or random.

### What might change between runs of the proposed Phase 11 splitter?

- **Tuple member naming** — if generated chunk-helper tuple-return names are derived from temporary IDs (e.g., `(t89, t142, t201)`), the order in which they appear in the tuple must be deterministic. Use sorted-by-temp-name OR sorted-by-first-use-line.
- **Helper file basenames** — the existing convention is `<level>_<spin>_part<N>_<suffix>_chunk<K>.rs`. Phase 11 must extend this pattern carefully — `_chunk<K>` indices must reset per `_part<N>`, not run globally.
- **Imports** — `detect_imports` already produces a deterministic sort. Phase 11's chunk helpers should also produce sorted imports.

### Idempotency test

Phase 11 plans should include a wave-final task:
```bash
git status --porcelain | grep '^.M' && echo "FAIL: re-run produced diff" && exit 1
# Run the splitter once with a clean checkout
python3 tools/maple_to_kernels.py all --family all
git add -A && git status --porcelain > /tmp/run1.txt
# Run again
python3 tools/maple_to_kernels.py all --family all
git add -A && git status --porcelain > /tmp/run2.txt
diff /tmp/run1.txt /tmp/run2.txt || (echo "FAIL idempotency"; exit 1)
```

## Failure Modes & Risks

### What broke at the abandoned 3000-line attempt (`260513-8nv-update-splitter-tool-enforce-3000-line-c`)

[VERIFIED: directory is empty (`ls -la` returns `total 8` — only `.` and `..`). No artifacts, no commits referencing the slug.]

The task was promoted to Phase 11 BEFORE any work was committed. Per CONTEXT.md D-LOCK-C, this is "If it produced uncommitted artifacts, they are discarded; if it created commits, they are reviewed during planning and either kept or reverted." Since there are no commits and no uncommitted artifacts, **nothing to discard**. The reason 3000 was abandoned in favor of 5000 was discussed during /gsd-discuss-phase but is NOT recorded in any artifact this researcher could find — likely it was a discussion-time decision that 3K was too aggressive against the existing 4,229-line max body in lda-1.

### Memory ceiling lessons from past iterations (verified evidence)

[VERIFIED: `260510-q01-SUMMARY.md` measured perforance]

- mgga-1 (299K LOC, 101 files): **22.7 GB peak RSS, 23m 30s wall**, jobs=1, 30 GB physical + 8 GB swap.
- Pre-fix linear extrapolation: ~76 KB peak RSS / source LOC.
- Post-fix prediction (reverting RUST_MIN_STACK from 1.87 GiB to 64 MB): "1–3 GB reduction."
- **Implication for Phase 11:** Lowering per-file size from ~10K to ~5K does NOT obviously reduce per-crate compile peak RSS, because rustc compiles per CRATE, not per file. The relevant metric is **per-(crate, file) HIR memory**, which scales with `#[cube]` body size — and the per-file cap IS what controls that. So Phase 11's 5K target should LOWER per-crate peak RSS proportionally if all things are equal.
- **BUT:** subcrate collapse (D-LOCK-A) means each family becomes ONE crate, and the WHOLE family's HIR sits in memory at once. Pre-collapse: mgga split across 17 subcrates means rustc sees ~50K-300K LOC per invocation. Post-collapse: mgga is ONE crate — could be ~6 MLOC of generated Rust. **THIS IS THE LARGEST RAM RISK.** Mitigation: the 5K-cap reduction in body size MUST land first; sucrate collapse second; verify per-family peak RSS at each step.

### Risks per success criterion

| Risk | Severity | Mitigation |
|------|---------|------------|
| Subcrate collapse exceeds RAM ceiling on a unified family crate | **HIGH** | Order of operations: 5K cap FIRST, collapse SECOND. Spike per-family RAM measurement (one family at a time) before committing to all three. Acceptance test: `cargo build -p libxc-kernel-mgga` peak RSS ≤ 30 GB. |
| Tuple-returning `<F: Float>` `#[cube]` round-trip fails through cubecl-macros 0.10 | **MEDIUM** | EMPIRICAL SPIKE REQUIRED before bulk rollout. Plan a dedicated Wave 0 task: minimal repro `#[cube] fn t<F: Float>(x: F, y: F) -> (F, F) { (x + y, x - y) }` + a launch test. Confirms parser, IR, codegen all round-trip cleanly. |
| CSE chunk arity exceeds cubecl-macros tuple parser limits | **MEDIUM** | Cap input arity at 16 and output arity at 16; force chunk split when exceeded. Tunable down to 8 if first spike fails at 16. |
| Naming collisions when collapsing subcrates (e.g., LDA and MGGA both have `lda_x` — but families are separate so no collision; intra-family collisions: NONE per file inventory) | **LOW** | `tools/collapse_subcrates.py` performs collision detection before move; aborts with diagnostic on conflict. |
| Idempotency violation (re-run produces diff) | **MEDIUM** | Implement post-run idempotency test (see "Idempotency Contract" above). Fix any non-deterministic ordering before declaring phase complete. |
| Oracle parity drift on chunked kernels (CSE temporaries change accumulation order beyond 1e-12 envelope) | **MEDIUM** | D-05 explicitly accepts 1e-12 relative error (NOT bit-exact). If specific functionals violate even 1e-12, fall back to a finer chunk granularity (preserve more named temporaries inline). |
| Inability to regen on `lxc_pol_part_*` of revtpss/tpssloc/kcisk (the 4th-derivative polarized monsters) | **MEDIUM** | These are ALREADY split into 25-30 `_partNN` files; the issue is each PART is still 5K-12K. CSE chunking inside each part is the answer. If even chunked, a part exceeds 5K, Phase 11 may need to deepen the part-split granularity (smaller per-output-component cuts). |
| Long verify wall-time blocks iteration | **MEDIUM** | Use representative subset for inner iteration loop; full sweep only at phase gate. Phase 11 plans should specify which functionals are in the per-iteration "smoke set" (recommend: lda_x, gga_x_pbe, mgga_x_scan, mgga_c_revtpss). |
| `tools/split_oversized_mgga.py` latent multi-functional `rmtree` bug (per 260510-q01 SUMMARY) | **LOW** | Deletable post-collapse; do not invoke during Phase 11 if subcrate collapse lands first. |
| `cargo` jobs/env override slipping in via subagent prompt | **MEDIUM** | D-09 mandates citing `.cargo/config.toml` as authoritative. Plans must NOT inline `RUST_MIN_STACK` or `CARGO_BUILD_JOBS` values; reference the config file. |

### What CANNOT be reused from the existing chunked-scratch path

The existing `chunk_single_output_split` in `translate_lda_v2.py` is the closest existing tooling to what D-02 wants — but it's structurally different in three important ways:
1. Uses shared `Array<f64>` mutable scratch — D-02 forbids this.
2. Hardcoded `f64` everywhere — D-03 wants `<F: Float>`.
3. LDA-only — must be ported to GGA and MGGA.

So Phase 11 builds the new ABI from scratch (CSE-aware tuple-returning `<F: Float>` chunks), then DELETES the existing chunked-scratch path once the new path covers the same cases. The 4 functions to delete (or rewrite): `chunk_single_output_split`, `_generate_chunk_helper`, `_generate_chunked_wrapper`, `_build_scratch_replacer` (LDA only).

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust `cargo test` + `approx` 0.5.1 (`relative_eq!` for 1e-12 comparisons) + `libxc-sys` (FFI to libxc 7.0.0 oracle) |
| Config file | `verify/Cargo.toml` (separate test crate); also `Cargo.toml` workspace `[profile]` settings |
| Quick run command | `cargo test -p libxc_rs-verify --test parity_phase09 -- --test-threads=1 --nocapture phase11_smoke` (proposed phase11_smoke filter) |
| Full suite command | `cargo test -p libxc_rs-verify -- --test-threads=1 --nocapture` (multi-hour, save for phase gate) |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| P11-INV-1 | No `crates/kernels/{family}-N` directories exist | smoke | `! ls crates/kernels/ \| grep -E '^(lda\|gga\|mgga)-[0-9]'` | ❌ Wave 0 needs `tools/audit_subcrate_collapse.sh` |
| P11-INV-2 | All `crates/kernels/**/*.rs` are ≤5000 lines | smoke | `find crates/kernels -name '*.rs' -exec wc -l {} + \| awk '$1 > 5000 && $2 != "total"' \| wc -l` (must equal 0) | ❌ Wave 0 needs `tools/audit_kernel_size.py` |
| P11-INV-3 | `cargo build --workspace` succeeds | smoke | `cargo build --workspace` (under D-08/D-09 envelope) | ✅ via cargo |
| P11-INV-4 | Oracle parity 1e-12 on energy + derivatives | unit + integration | `cargo test -p libxc_rs-verify -- --test-threads=1` | ✅ existing tests; add Phase 11 sweep file |
| P11-INV-5 | **(REVISED — D-13)** Launch surface matches the per-functional design: exactly one `#[cube(launch_unchecked)]` entry per routed `(functional, output)`; no **unrouted** kernel launchable; `crates/kernels/math/` ≤22. NOT a flat count. | smoke | `bash tools/audit_cube_launch.sh` (rewritten per D-13 in the replanned 11-03) | ✅ `tools/audit_cube_launch.sh` exists (Wave 0); **rewritten** in replanned 11-03 |
| P11-INV-6 | Pipeline idempotent (re-run produces no diff) | integration | `tools/test_idempotency.sh` (proposed) | ❌ Wave 0 needs `tools/test_idempotency.sh` |

### Sampling Rate

- **Per task commit:** P11-INV-1, P11-INV-2, P11-INV-5 (smoke audits — fast, <5s each)
- **Per wave merge:** P11-INV-3 + P11-INV-4 on a smoke set (~10 functionals representative of LDA, GGA, MGGA across orders/spins)
- **Phase gate:** Full P11-INV-4 oracle sweep + P11-INV-6 idempotency test

### Wave 0 Gaps

- [ ] `tools/audit_kernel_size.py` — fails non-zero if any `crates/kernels/**/*.rs` > 5000 lines (covers P11-INV-2)
- [ ] `tools/audit_subcrate_collapse.sh` — fails non-zero if any `crates/kernels/{family}-N` dirs exist (covers P11-INV-1)
- [ ] `tools/audit_cube_launch.sh` — Wave 0 built it as a flat `≤23` count check; **per D-13 (2026-05-15) it is rewritten in the replanned 11-03** to assert the per-design budget (one launch wrapper per routed `(functional, output)`; no unrouted kernel launchable; `math/` ≤22) (covers revised P11-INV-5)
- [ ] `tools/test_idempotency.sh` — runs the splitter twice, diffs the tree, fails on diff (covers P11-INV-6)
- [ ] `verify/tests/parity_phase11.rs` — modeled on `parity_phase09.rs`; covers a curated smoke set + the worst-case functionals (mgga_c_revtpss, mgga_c_kcisk, mgga_c_b94, mgga_x_r4scan)
- [ ] Spike test: `verify/tests/spike_tuple_return_cube.rs` — smallest possible `#[cube] fn f<F: Float>(x: F, y: F) -> (F, F) { (x + y, x - y) }` + launch + assert. RUNS FIRST IN PHASE 11.

## Code Examples

### Existing chunked-scratch wrapper (current LDA pattern, to be replaced by D-02)

[VERIFIED: `crates/kernels/lda-2/src/lda_xc_ksdt/lxc_pol_part5_v4rho4_1.rs` lines 1-25]

```rust
//! LDA_XC_KSDT chunked-scratch entry — wraps 2 `_chunkN` helpers via a shared `Array<f64>` slot file.

#![allow(unused_imports, unused_variables, non_snake_case, ...)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};
use super::lxc_pol_part5_v4rho4_1_chunk0::lda_xc_ksdt_lxc_pol_part5_v4rho4_1_chunk0;
use super::lxc_pol_part5_v4rho4_1_chunk1::lda_xc_ksdt_lxc_pol_part5_v4rho4_1_chunk1;

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_xc_ksdt_lxc_pol_part5_v4rho4_1(
    rho: &Array<f64>,
    v4rho4: &mut Array<f64>,
    param_T: f64, /* ... 40+ scalar params ... */
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < v4rho4.len() {
        let mut s = Array::<f64>::new(<scratch_size>usize);
        lda_xc_ksdt_lxc_pol_part5_v4rho4_1_chunk0(rho, &mut s, param_T, /* ... */);
        lda_xc_ksdt_lxc_pol_part5_v4rho4_1_chunk1(rho, &mut s, param_T, /* ... */);
        v4rho4[ip * 5 + 1] += s[<output_idx>usize];
    }
}
```

### Proposed D-02 chunked tuple-return wrapper (NEW — must be spiked)

```rust
// Source: D-02 design — verify with cubecl-macros 0.10 spike before rolling out
//! LDA_XC_KSDT D-02 chunked entry — wraps `_chunkN` helpers via tuple returns.

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::pow_1_3;
use libxc_kernel_math::piecewise::piecewise3;

#[cube]
pub fn lda_xc_ksdt_lxc_pol_part5_v4rho4_1<F: Float>(
    rho: &Array<F>,
    v4rho4: &mut Array<F>,
    // Per-functional scalars: f64 today; future-direction wraps as F if cubecl
    // supports value coercion. Phase 11 leaves them as f64 (wrapped via F::new
    // at use sites in chunks).
    param_T: f64, /* ... */
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < v4rho4.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        // Each chunk takes its inputs explicitly and returns its outputs as a tuple.
        let (t89, t142) = chunk_0::<F>(rho0, rho1, F::new(param_T) /* ... */);
        let (t201, t275) = chunk_1::<F>(t89, t142, rho0, F::new(dens_threshold));
        v4rho4[ip * 5 + 1] += t275;
    }
}

#[cube]
fn chunk_0<F: Float>(rho0: F, rho1: F, param_T: F) -> (F, F) {
    // ... compute lines, no shared mutable scratch ...
    (t89_value, t142_value)
}

#[cube]
fn chunk_1<F: Float>(t89: F, t142: F, rho0: F, dens_threshold: F) -> (F, F) {
    // ... compute lines ...
    (t201_value, t275_value)
}
```

[ASSUMED] cubecl-macros 0.10 round-trips this pattern correctly. **Empirical confirmation required during Wave 0 spike.**

### Existing dispatch site (will continue to work after Strategy-1 collapse)

[VERIFIED: `src/eval/mgga_dispatch/batch17.rs` lines 17-30]

```rust
mgga_zero_scalar_unpol_dispatch!(
    ctx, order, spin,
    [crate::kernel::mgga::batch17::mgga_k_gea2::exc_unpol::mgga_k_gea2_exc_unpol],
    [crate::kernel::mgga::batch17::mgga_k_gea2::vxc_unpol::mgga_k_gea2_vxc_unpol],
    "mgga_k_gea2"
);
```

After Strategy-1 collapse: `crate::kernel::mgga::batch17::...` resolves because `crates/kernels/mgga/src/lib.rs` continues to re-export `pub mod batch17;` (instead of `pub use libxc_kernel_mgga_17 as batch17;`).

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | cubecl-macros 0.10 cleanly round-trips `#[cube] fn f<F: Float>(...) -> (F, F)` tuple returns | D-02 ABI; Code Examples | HIGH — entire Phase 11 chunk ABI depends on this. Mitigation: Wave 0 empirical spike. Parser supports tuples (verified in `expression.rs:450` and `parse/expression.rs:232`); IR/codegen path is the unknown. |
| A2 | CSE chunk inputs/outputs ≤16 each is a safe arity cap | CSE Subdivision Strategy | MEDIUM — if cubecl-macros chokes at 16-tuple, lower the cap (will produce more, smaller chunks). |
| A3 | Lowering SPLIT_THRESHOLD from 6000 to ~5000 (with ~500 line boilerplate budget) is sufficient for the existing chunked-scratch path to produce ≤5K files for the LDA cases | Pathological Inventory | LOW — verifiable by quick LDA regen test before phase 11 implementation. Existing chunk0 at 5,995 is at the edge; lowering threshold should produce 2 chunks where there was 1. |
| A4 | Subcrate collapse RAM peak (~6 MLOC family crate compile) is survivable on the 30 GB + 8 GB swap machine | Failure Modes | HIGH — could OOM. Mitigation: 5K cap MUST land first; spike per-family RAM measurement before committing all three families. |
| A5 | The `_partNN` suffix convention is the right granularity for D-04 (already in use, well-grep-friendly) | D-04 (locked) | LOW — locked decision. |
| A6 | Strategy-1 (preserve `batchN` as identity submodules) is the cheapest collapse | Sub-crate Collapse Mechanics | LOW — verified by reading dispatch import paths; no functional code references the underlying numbered crate name. |
| A7 | The Python pipeline can detect CSE breakpoints from `compute_lines: List[str]` using existing `build_dependency_graph` infrastructure | CSE Strategy Option C | MEDIUM — heuristic tuning may need iteration; tunables are min_uses (default 5) and min_chain_length (default 50). |
| A8 | Verify wall-time on Phase 11's representative subset (~10 functionals across families/orders/spins) is under 30 minutes per iteration | Validation Architecture | MEDIUM — extrapolated from `260510-q01` mgga-1 measurement; specific subset wall-time will be measured at first iteration. |

**If user disagrees with A1/A2:** plan a Wave 0 spike that proves or disproves the tuple-return ABI BEFORE writing the CSE chunker.
**If user disagrees with A4:** plan an interim state where 5K cap holds without subcrate collapse, then evaluate.

## Open Questions (RESOLVED)

> **D11 closure (planner-checker dimension 11, Research Resolution):** All five questions below are operationalized in the Phase 11 plans. The recommendations have been adopted; resolutions are recorded inline below each question.

1. **Should chunk helpers themselves be allowed to call other chunk helpers (nesting)?**
   - What we know: cubecl-macros 0.10 expands `#[cube]` calls inline at the IR level; nesting is supported syntactically.
   - What's unclear: whether deeply-nested chunk-of-chunk-of-chunk graphs explode the proc-macro stack the same way deep `#[cube]` bodies do (per D-08 SIGSEGV history).
   - Recommendation: keep chunk graphs FLAT in v1 — wrapper calls chunk_0, chunk_1, ..., chunk_N in sequence, no chunk calls another chunk. If a chunk's deps argue for nesting, lift the shared dep INTO the wrapper rather than nesting. Revisit after the bulk regen succeeds.
   - **RESOLVED:** Adopted. Operationalized in plan 11-02 task 1 (`tools/translate_v2/cse.py` flat-graph contract: "chunks never reference symbols defined in another chunk except via `inputs`").

2. **What about the 25 `_partNN` files of `mgga_c_revtpss/lxc_pol_*` — should they ALL be re-chunked, or just the ones >5K?**
   - What we know: 30 of 35 revtpss files are >5K. The other 5 are between 1K-5K (already-merged small splits).
   - What's unclear: whether re-chunking the 5K-and-under files would IMPROVE or DEGRADE compile time.
   - Recommendation: re-chunk only files >5K. Leave files ≤5K untouched (idempotency-friendly).
   - **RESOLVED:** Adopted. Operationalized in plan 11-04 must_have ("only files >5K are re-chunked; the 5 already-merged small revtpss splits (1K-5K range) are left untouched").

3. **Are there any functionals where CSE-aware chunking can't help because the deepest dependency chain is itself >5K lines?**
   - What we know: the LDA case `lda_xc_ksdt/lxc_pol_part5_v4rho4_1` produces a chunk0 of 5,995 lines (existing chunked-scratch). At Phase 11 threshold of 5K, this is one chunk too few.
   - What's unclear: whether ANY single chunk's MINIMUM dep chain (root → leaf) exceeds 5K lines. If yes, mid-chain CSE breakpoints (forced even when not natural) become necessary.
   - Recommendation: planner adds an explicit "if no natural breakpoint within 4500 lines, force a breakpoint at the lowest-arity cut point in the next 500 lines" rule.
   - **RESOLVED:** Adopted. Operationalized in plan 11-02 task 1 (`tools/translate_v2/cse.py` forced-breakpoint contract: "when chunk size reaches `chunk_max_lines - force_headroom` without a natural breakpoint, scan the next `force_headroom` lines for the cut point that minimizes `len(inputs) + len(outputs)` and force a break there"; `DEFAULT_FORCE_HEADROOM = 500`).

4. **What's the right chunk-naming convention?**
   - Current LDA: `<func>_<level>_<spin>_part<N>_<suffix>_chunk<K>` — already 80+ chars on lxc-level functions. The 60-char merge-suffix cap (`_SUFFIX_MAX_CHARS = 60`) was added in 260512-q02 to avoid the Linux 255-byte filename limit. Phase 11 must continue to respect this; chunk indices append AFTER the suffix cap, so chunk indices `_chunk0` through `_chunkN` add ≤8 chars per chunk and stay safe.
   - Recommendation: keep the existing naming; verify max basename ≤200 chars (`83 chars` measured by 260512-q02).
   - **RESOLVED:** Adopted. Documented in PATTERNS.md ("Python translator deterministic-emit conventions" section). The `_SUFFIX_MAX_CHARS = 60` cap is preserved; chunk indices appended after the suffix cap stay within the 200-char basename budget.

5. **Should `tools/maple_to_kernels.py`'s stale `DEFAULT_SPLIT_THRESHOLD = 100_000` and `DEFAULT_TARGET_MAX = 500_000` be aligned to the new 5K cap?**
   - What we know: these defaults are NOT honored — the underlying translators have hardcoded `SPLIT_THRESHOLD = 6000` and would need to be modified to 5000. The driver just warns when its CLI values diverge from the translator constants.
   - What's unclear: whether the driver should be modified to MUTATE the translator constants on each invocation, or keep the warning + manual sync.
   - Recommendation: as part of Phase 11, refactor the driver to either (a) accept `--split-threshold` and override the translator constant via env var, or (b) remove the unused CLI knobs and document that translators own the constant. Decision is Claude's discretion.
   - **RESOLVED:** Option (b) adopted. Operationalized in plan 11-06 task 1 ("tools/maple_to_kernels.py stale defaults DEFAULT_SPLIT_THRESHOLD=100_000 and DEFAULT_TARGET_MAX=500_000 are removed per RESEARCH.md Q5 — translators now own the constant (5000); driver no longer pretends to").

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Python 3 | All `tools/translate_*.py` and `tools/split_*.py` | ✓ | system python3 | — |
| Rust toolchain | `cargo build --workspace`, `cargo test` | ✓ | edition 2024, MSRV 1.85+ | — |
| `cubecl` 0.10.0 | All kernel crates | ✓ | 0.10.0 (per Cargo.toml) | — |
| `libxc-sys` (libxc 7.0.0 FFI) | `verify/` oracle harness | ✓ | bundled, vendored at `libxc-sys/` and `libxc-master/` | — |
| `bindgen`, `cmake` | `verify/` build (oracle compile) | ✓ | 0.72.1, 0.1.58 | — |
| sccache | Per `.cargo/config.toml` (referenced) | Assumed ✓ | system sccache | If missing, builds slow but functional. Phase 11 MUST NOT disable sccache. |
| `syn` (Rust crate) | NOT needed if CSE Strategy C chosen (recommended); only needed if Strategy B chosen | n/a | n/a | — |
| `git` | Idempotency test, commit phase artifacts | ✓ | system git | — |

**Missing dependencies with no fallback:** None.
**Missing dependencies with fallback:** None.

## Standard Stack

### Core (already in use; no new dependencies for Phase 11)

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| cubecl | 0.10.0 | `#[cube]` proc-macro, IR, runtime client | The substrate; cannot change |
| cubecl-cpu | 0.10.0 (via `features = ["cpu"]`) | CPU backend for verify (no GPU dep) | Always-available test target |
| bytemuck | 1.25.0 | Safe slice→bytes for `client.create_from_slice` | Required by cubecl client API |
| thiserror | 2.0.18 | `LibxcRsError` enum | Library-boundary errors |
| Python 3 stdlib (`re`, `os`, `dataclasses`, `argparse`) | 3.x | Splitter tooling | Zero deps; matches existing `tools/translate_*.py` style |

### Verification (already in use)

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| libxc-sys (bindgen-generated) | 7.0.0 | Oracle source-of-truth | Every D-05 verification |
| approx | 0.5.1 | `relative_eq!` for 1e-12 comparisons | Every oracle comparison |
| anyhow | 1.0.100 | verify crate error handling | Test harness internals |
| rayon | 1.11.0 | Parallel test execution | NOT for Phase 11 — D-07 mandates serial |

### NOT recommended for Phase 11

| Tool | Why Not | What Instead |
|------|---------|-------------|
| `tree-sitter-rust` (Python) | Needs C dependency, install pain on RAM-constrained machine | CSE Strategy C (Python-side analysis on `compute_lines`) |
| `syn` (Rust crate) — for CSE Strategy B | Adds new Rust binary to `tools/`, two-pass complication | CSE Strategy C |
| Maple parser (Python) — for CSE Strategy A | Massive surface, no off-the-shelf option | CSE Strategy C |
| ndarray / nalgebra | Per existing CLAUDE.md guidance | Already excluded |
| `proptest` for Phase 11 | Phase 11 is correctness-by-oracle, not property-based | Existing oracle harness |

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| 5K SPLIT_THRESHOLD | 18K | Phase 9 plan 09-04 (raise) | Per-functional file count down ~3.6× |
| 18K SPLIT_THRESHOLD | 50K | Quick task 260509-q02 | Fewer files, larger per-file |
| 50K SPLIT_THRESHOLD | 100K | Quick task 260509-q07 (post-path-move) | Even larger files |
| 100K SPLIT_THRESHOLD | 6K | Quick task 260512 (lda-2 OOM unblock) | Reverted to small-files regime |
| `RUST_MIN_STACK = 2010886400` (1.87 GiB) | 67108864 (64 MiB) | Quick task 260510-q01 (typo fix) | OOM cliff fixed |
| Pre-routing-aware `#[cube(launch_unchecked)]` for split parts | Plain `#[cube]` for split parts via `is_split_helper` check | Quick task 260512-q01 | Closes regen-reintroduces-launch-wrappers loop |
| 22 numbered subcrates | 3 family façades + 22 numbered children | Quick tasks 260509-q05/q06/q08 + 260510-q01 | Workspace member count grew, RAM risk distributed |
| `f64`-only kernels | `f64` + `<F: Float>` generic chunks (D-03) | Phase 11 (PROPOSED) | First `<F: Float>` kernels in libxc_rs |

**Deprecated/outdated:**
- `tools/split_oversized_kernel.py` — becomes irrelevant after subcrate collapse (D-LOCK-A).
- `tools/split_oversized_mgga.py` — same; latent multi-functional `rmtree` bug noted in 260510-q01 SUMMARY.
- `tools/split_mgga_7_kcis.py` — one-off, deletable post-collapse.
- `tools/rebatch_mgga.py` — bin-packing into numbered subcrates, irrelevant post-collapse.
- `tools/shrink_part_fanout.py` — already integrated into translators per 260512-q01; deletable.
- `CLAUDE.md` "f64 only" constraint — D-03/D-03a explicitly relaxes it.

## Sources

### Primary (HIGH confidence)

- `docs/manual/Cubecl/cubecl_macro_fanout_manual.md` — re-read end-to-end §1-23 (1188 lines)
- `tools/translate_lda_v2.py` — read end-to-end (1643 lines)
- `tools/translate_gga.py` — read SPLIT_THRESHOLD region + chunked-path absence verification
- `tools/translate_mgga.py` — read SPLIT_THRESHOLD region + chunked-path absence verification
- `tools/maple_to_kernels.py` — read end-to-end (246 lines)
- `tools/split_oversized_kernel.py` — read first 100 lines (TARGET_MAX semantics)
- `.cargo/config.toml` — read end-to-end (12 lines)
- `Cargo.toml` (root) — workspace members + deps verified
- `crates/kernels/{lda,gga,mgga}/src/lib.rs` — façade re-export patterns verified
- `src/eval/mgga_dispatch/batch17.rs` — sample dispatch import path verified
- `verify/tests/parity_phase09.rs` — verify harness pattern verified
- `.planning/quick/260510-q01-investigate-kernel-oom/260510-q01-SUMMARY.md` — empirical RAM measurements
- `.planning/quick/260512-q02-fix-merge-filename-overflow/260512-q02-SUMMARY.md` — recent regen sample
- `.planning/phases/11-splitter-v2-unified-5k-cap/11-CONTEXT.md` — locked decisions
- `.planning/REQUIREMENTS.md` — confirmed no Phase 11 REQ-IDs
- `.planning/ROADMAP.md` Phase 11 entry — success criteria

### Secondary (MEDIUM confidence)

- Context7 lookup `/tracel-ai/cubecl` for `<F: Float>` generic kernel patterns — confirmed `cube` macro syntax with generic Float parameter; tuple returns NOT in any official example BUT macro source confirms `Expression::Tuple` is parsed and emitted as plain Rust tuple syntax
- `~/.cargo/registry/src/index.crates.io-*/cubecl-macros-0.10.0/src/{parse,generate}/expression.rs` — direct read of cubecl-macros source confirming tuple parse + emit support
- File-size measurements: `find crates/kernels -name '*.rs' -exec wc -l {} +` (verified)

### Tertiary (LOW confidence — flagged for validation)

- A1 (tuple-return cubecl round-trip) — parser confirmed; IR/codegen path UNVERIFIED. Wave 0 spike required.
- A4 (post-collapse family-crate RAM ceiling) — extrapolation only; verify per-family build at first iteration.

## Metadata

**Confidence breakdown:**
- Existing system map: HIGH — every file read end-to-end or thoroughly inspected
- D-01 CSE strategy: MEDIUM — three options offered with clear recommendation; planner picks
- D-02 tuple ABI: MEDIUM — parser confirmed, but no oracle-validated tuple kernel exists in libxc_rs today
- D-03 `<F: Float>` generics: MEDIUM — pattern documented in cubecl docs but not exercised in this codebase
- Subcrate collapse: HIGH — dispatch import paths verified, no hidden direct references to numbered crates
- Pathological inventory: HIGH — counted directly via `wc -l`
- Build env: HIGH — `.cargo/config.toml` and `Cargo.toml` read verbatim
- Pitfalls: HIGH — grounded in actual quick-task SUMMARY measurements (260510-q01, 260512-q02)

**Research date:** 2026-05-13
**Valid until:** 2026-06-13 (30 days for stable splitter pipeline; should be re-checked if cubecl 0.10 → 0.11 upgrade lands or if a new oversized functional appears in `libxc-master/maple/`)

## RESEARCH COMPLETE
