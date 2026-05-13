# Roadmap: libxc_rs

## Overview

This roadmap delivers a pure-Rust reimplementation of libxc 7.0.0 covering all 649 exchange-correlation functionals with f64 precision, CubeCL-based GPU compute, and C API compatibility. The journey follows the natural dependency chain: domain types and registry (foundation) -> math core and CubeCL validation (risk gate) -> I/O bundles and evaluation framework (interface contract) -> bulk kernel translation (core value) -> functional lifecycle and public API (usability) -> C compatibility layer (interop) -> GPU backends and performance (differentiator). Each phase delivers a verifiable capability that unblocks the next.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [ ] **Phase 1: Foundation and Registry** - Domain types, error handling, static registry, dimension calculation, and oracle harness
- [ ] **Phase 2: Math Core and CubeCL Substrate** - All #[cube] math building blocks, CubeCL CPU backend, LDA_X canary kernel validation
- [ ] **Phase 3: Input/Output and Evaluation Framework** - I/O bundle types, output masks, dispatch routing, mixed functional accumulation
- [x] **Phase 4: Bulk Kernel Translation** - All 270 maple2c kernel files translated to #[cube] functions with oracle verification
- [ ] **Phase 5: Functional Lifecycle and Hybrid Properties** - Functional struct, parameter management, hybrid queries, evaluation orchestration
- [ ] **Phase 6: Public API and C Compatibility** - Builder pattern, BatchEvaluator, ergonomic API, all 85 extern "C" functions
- [ ] **Phase 7: GPU Backends and Performance** - ROCM/HIP/WGPU backends, GPU buffer management, benchmarks, performance targets
- [ ] **Phase 11: Splitter v2 — Unified Kernels with 5K Line Cap** *(INSERTED)* - Collapse per-family subcrates; extend splitter to subdivide single output expressions so every emitted kernel file is ≤5,000 lines

## Phase Details

### Phase 1: Foundation and Registry
**Goal**: All domain types, error handling, static registry with 649 functionals, dimension calculation, and oracle verification harness are in place -- the project compiles, tests pass, and any functional can be looked up by ID or name with correct metadata
**Depends on**: Nothing (first phase)
**Requirements**: DOM-01, DOM-02, DOM-03, DOM-04, DOM-05, REG-01, REG-02, REG-03, REG-04, REG-05, ERR-01, ERR-02, ERR-03, VERIFY-01, BUILD-01, BUILD-02, BUILD-03, BUILD-04, BUILD-05
**Success Criteria** (what must be TRUE):
  1. Any of the 649 functional IDs can be looked up by numeric ID in O(1) or by name in O(log n), returning complete metadata (family, kind, references, ext_params)
  2. All 52 removed functional IDs return a typed error containing the replacement ID and name
  3. Dimension calculation returns correct array sizes for all family/spin/order combinations (verified against libxc util.c, including the 477-component case)
  4. The verify/ crate links against system libxc 7.0.0 via bindgen and can call C libxc functions to obtain oracle values
  5. cargo build, cargo test, and cargo clippy all pass with zero warnings
**Plans:** 3 plans

Plans:
- [x] 01-01-PLAN.md -- Domain types, error enum, and dimension calculation
- [x] 01-02-PLAN.md -- Xtask code generator, static registry, and FunctionalId wiring
- [x] 01-03-PLAN.md -- Oracle verification harness and build quality validation

### Phase 2: Math Core and CubeCL Substrate
**Goal**: All mathematical building blocks are implemented as #[cube] functions, validated against known values and libm references, and the CubeCL CPU backend produces bit-accurate f64 results for the LDA_X canary kernel
**Depends on**: Phase 1
**Requirements**: MATH-01, MATH-02, MATH-03, MATH-04, MATH-05, MATH-06, MATH-07, MATH-08, MATH-09, MATH-10, KERN-01, KERN-02
**Success Criteria** (what must be TRUE):
  1. safe_cbrt(-8.0) returns -2.0 (not NaN) on both CPU and GPU backends
  2. erf and erfc approximations match libm values to within f64 precision across the full input domain
  3. LDA_X kernel (both unpolarized and polarized) produces energy with relative error <= 10^-12 vs libxc oracle
  4. All #[cube] math functions produce identical results on CubeCL CPU backend as on native Rust (cross-backend consistency)
  5. Kernel launch wrapper correctly handles buffer creation, CubeCount/CubeDim calculation, and backend selection
**Plans**: 5 plans

Plans:
- [x] 02-01-PLAN.md — Math core: constants, powers, piecewise, polynomials, erf, spin, DFT quantities
- [x] 02-02-PLAN.md — Kernel launch infrastructure: backend selection, buffer management, CubeCount/CubeDim
- [x] 02-03-PLAN.md — LDA_X canary kernel translation and oracle verification
- [x] 02-04-PLAN.md — Math integration tests and workspace build quality gate
- [x] 02-05-PLAN.md — LDA_X edge-case and stress testing (thresholds, alpha, extreme spins)

### Phase 3: Input/Output and Evaluation Framework
**Goal**: Type-safe I/O bundles validate buffer sizes, output masks control which derivatives are computed, and the dispatch/accumulation framework correctly routes evaluation for single and mixed functionals
**Depends on**: Phase 2
**Requirements**: IO-01, IO-02, IO-03, IO-04, IO-05, EVAL-01, EVAL-02, EVAL-03, EVAL-04, EVAL-05
**Success Criteria** (what must be TRUE):
  1. LdaInput/GgaInput/MggaInput reject buffers whose length does not match the Dimensions for the given spin mode
  2. Output bundles support Option-based NULL-pointer semantics -- passing None for a derivative level skips its computation
  3. OutputMask bitflags correctly select which derivative levels (exc/vxc/fxc/kxc/lxc) to compute
  4. Mixed functional evaluation accumulates weighted results from auxiliary functionals, matching mix_func.c behavior
  5. Non-mixed evaluation hot path performs zero heap allocations
**Plans**: 3 plans

Plans:
- [x] 03-01-PLAN.md -- Input/output bundle types with validation and OutputMask bitflags
- [x] 03-02-PLAN.md -- LDA dispatch layer bridging bundles to CubeCL kernels
- [x] 03-03-PLAN.md -- Mixed functional workspace and accumulation framework

### Phase 4: Bulk Kernel Translation
**Goal**: All 270 maple2c kernel files are translated to Rust #[cube] functions preserving exact floating-point operation order, and every functional passes oracle verification through all applicable derivative orders and spin modes
**Depends on**: Phase 3
**Requirements**: KERN-03, KERN-04, KERN-05, KERN-06, KERN-07, KERN-08, KERN-09, VERIFY-02, VERIFY-03, VERIFY-04, VERIFY-05, VERIFY-06, VERIFY-07
**Success Criteria** (what must be TRUE):
  1. All ~43 LDA functional kernels pass oracle verification (energy relative error <= 10^-12)
  2. All ~130 GGA functional kernels pass oracle verification through applicable derivative orders
  3. All ~80 MGGA functional kernels pass oracle verification through applicable derivative orders
  4. Density thresholding correctly skips grid points below threshold and clamps spin densities
  5. Output accumulation uses += semantics to support mixed functional evaluation
**Plans:** 5 plans

Plans:
- [x] 04-01-PLAN.md — Infrastructure: missing power functions, oracle GGA/MGGA harness, module scaffolding
- [x] 04-02-PLAN.md — LDA dispatch: generalize dispatch_lda + deferred.rs + per-functional Rust-vs-C oracle comparison (37 compiled, 4 deferred)
- [x] 04-03-PLAN.md — GGA dispatch: dispatch_gga scaffolding + GgaFunctional enum + per-functional oracle comparison (105 routable; 42 zero-scalar dispatched + 63 UnsupportedFunctional pending Phase 4 follow-up)
- [x] 04-04-PLAN.md — MGGA dispatch: new dispatch_mgga + MggaFunctional enum + per-functional oracle comparison (86 compiled, 6 deferred)
- [x] 04-05-PLAN.md — Cross-family verification sweep (cargo xtask verify-phase-4) + Phase 4 COVERAGE sign-off

### Phase 5: Functional Lifecycle and Hybrid Properties
**Goal**: Users can construct a Functional instance by ID, configure external parameters and thresholds, query hybrid properties, and evaluate any of the 649 functionals through the Functional struct
**Depends on**: Phase 4
**Requirements**: FUNC-01, FUNC-02, FUNC-03, FUNC-04, FUNC-05, FUNC-06, HYB-01, HYB-02, HYB-03, HYB-04
**Success Criteria** (what must be TRUE):
  1. Functional::new(id, spin) returns a fully initialized instance with correct metadata, dimensions, thresholds, and default ext_params
  2. External parameters can be set/get by name or index, and modifying ext_params triggers recomputation of derived parameters
  3. Hybrid functionals correctly report their HybridType, CAM coefficients (omega, alpha, beta), and NLC coefficients (b, C)
  4. Auxiliary functionals for mixed/hybrid functionals are recursively constructed and iterable
  5. Drop implementation cleans up all resources without leaks
**Plans**: 7 plans (3 original + 4 gap-closure)

Plans:
- [x] 05-01-PLAN.md — libxc-sys factoring + cargo xtask generate-metadata + full FunctionalMeta population (references/ext_params/hybrid_terms/auxiliaries/nlc_params/hybrid_type for all 649 IDs) + verify/tests/metadata_oracle.rs round-trip test
- [x] 05-02-PLAN.md — FunctionalParams trait + 229 per-functional impls + dispatch signature migration to &dyn FunctionalParams + Functional struct (lifecycle + config: ext_param get/set by name/index/bulk, 4 threshold setters) + GgaScratch/MggaScratch materialization + 4 new LibxcRsError variants
- [x] 05-03-PLAN.md — Rust port of xc_hyb_type + CAM/NLC/aux queries (HYB-01..04) + eager recursive aux construction with propagation map + evaluate_mixed_gga/evaluate_mixed_mgga + Functional::evaluate_{lda,gga,mgga} + verify/tests/{hybrid_type_oracle,hybrid_oracle,mixed_oracle}.rs
- [x] 05-04-PLAN.md (gap-closure) — xtask generate-metadata real implementation: populate generated.rs/generated_hybrid.rs/generated_propagation.rs for all 649; unignore 6 oracle tests; CR-05 FFI rc-check
- [x] 05-05-PLAN.md (gap-closure) — set_ext_param_by_index defensive fallback (CR-04) + replace .expect() in ten_arm_dispatch_gga! and mgga_zero_scalar_unpol_dispatch! macros with typed LibxcRsError (CR-07)
- [x] 05-06-PLAN.md (gap-closure) — replace add_opt with length-checked add_opt_n (CR-02); add parent NEEDS_LAPLACIAN/NEEDS_TAU gates to evaluate_mixed_mgga (CR-03); harden add_to_mix assert (WR-11); remove dead let-discard (WR-10)
- [x] 05-07-PLAN.md (gap-closure) — remove 108 redundant libxc-kernel-mgga-* entries from root Cargo.toml [dev-dependencies] (CR-06 cleanup)

### Phase 6: Public API and C Compatibility
**Goal**: The library provides an ergonomic Rust API with builder pattern and batch evaluation, plus a complete C compatibility layer that enables drop-in replacement for libxc in C/Fortran DFT codes
**Depends on**: Phase 5
**Requirements**: API-01, API-02, API-03, COMPAT-01, COMPAT-02, COMPAT-03
**Success Criteria** (what must be TRUE):
  1. FunctionalBuilder supports chained configuration of spin, thresholds, and ext_params with validation at build time
  2. BatchEvaluator reuses workspace across repeated evaluations without per-call allocation
  3. All 85 public C API functions are implemented as extern "C" with correct signatures matching libxc headers
  4. C-compatible struct layouts pass size/alignment assertions matching libxc's xc_func_type
  5. All unsafe code is confined to compat/, kernel/launch.rs, and GPU buffer management modules
**Plans**: 4 plans (06-02 split during revision into 06-02a infra+lifecycle and 06-02b accessors)

Plans:
- [x] 06-01-PLAN.md — Layer-3 Rust API: FunctionalBuilder, BatchEvaluator, sealed EvaluateInput trait, +InvalidSpin variant (API-01..03; Wave 1)
- [ ] 06-02a-PLAN.md — Compat infrastructure + lifecycle: c_layout/errno/macros/raw_handle, 4 threshold setters, 5 ext_params setters/getters, 2 errno accessors, Pitfall 4 fix, Pitfall 10 substitution, 25-variant LibxcRsError::discriminant table (16 extern C; COMPAT-02/03; Wave 2 *(blocked on 06-01)*)
- [ ] 06-02b-PLAN.md — Compat accessors + AK13: 8 discovery, 14 info/reference, 5 library, 9 hybrid+aux+NLC+AK13 (formula inlined verbatim from gga_x_ak13.c with oracle pairs), removed helper (35 extern C; COMPAT-01 partial; Wave 3 *(blocked on 06-02a)*)
- [ ] 06-03-PLAN.md — 35 evaluate functions (12 LDA + 12 GGA + 11 MGGA, with NULL-skip + order-inference), hand-written include/xc.h with 25 LIBXC_RS_* codes, verify/tests/compat_smoke.rs (closes COMPAT-01; total ≥85 symbols; Wave 4 *(blocked on 06-02b)*)

### Phase 7: GPU Backends and Performance
**Goal**: GPU evaluation delivers >5x CPU throughput on ROCM, all performance targets are met, and the benchmark suite provides regression detection
**Depends on**: Phase 6
**Requirements**: GPU-01, GPU-02, GPU-03, GPU-04, GPU-05, GPU-06, GPU-07, VERIFY-08, PERF-01, PERF-02, PERF-03, PERF-04, PERF-05
**Success Criteria** (what must be TRUE):
  1. ROCM backend produces results matching CPU to within 10^-14 relative error for all tested functionals
  2. GPU batch evaluation (100k points) achieves >5x CPU batch throughput on ROCM
  3. WGPU backend returns a typed error at runtime if the device lacks f64 support (no silent f32 fallback)
  4. CPU batch evaluation (1000 points) is within 1.5x of libxc C performance
  5. Benchmark suite with criterion detects performance regressions across key functionals
**Plans**: 3 plans

Plans:
- [ ] 07-01: TBD
- [ ] 07-02: TBD
- [ ] 07-03: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Foundation and Registry | 0/3 | Not started | - |
| 2. Math Core and CubeCL Substrate | 0/5 | Not started | - |
| 3. Input/Output and Evaluation Framework | 0/3 | Not started | - |
| 4. Bulk Kernel Translation | 5/5 | Complete | 2026-04-24 |
| 5. Functional Lifecycle and Hybrid Properties | 0/3 | Not started | - |
| 6. Public API and C Compatibility | 1/4 | In progress | - |
| 7. GPU Backends and Performance | 0/3 | Not started | - |

### Phase 8: Rebuild MGGA kernel conversion tool from scratch with iterative pattern verification

**Goal:** Build translate_mgga.py from scratch, iteratively verify it against representative MGGA functionals, then batch-translate all 92 MGGA functionals into compiled sub-crates with oracle-verified numerical correctness
**Requirements**: KERN-05, KERN-06, VERIFY-03, VERIFY-04
**Depends on:** Phase 4 (kernel infrastructure), Phase 8-extract (sub-crate pattern)
**Success Criteria** (what must be TRUE):
  1. translate_mgga.py translates any MGGA maple2c C file into compilable Rust #[cube] kernels with lapl and tau arrays
  2. All 92 MGGA functionals are translated and organized into sub-crates that compile
  3. Representative MGGA functionals pass oracle comparison with relative error <= 1e-12 for exc
  4. kernel-mgga facade re-exports all sub-crates

Plans:
- [x] 08-01-PLAN.md -- Build translate_mgga.py and compile first functional (mgga_xc_lp90)
- [x] 08-02-PLAN.md -- Translate representative functionals and oracle verification
- [x] 08-03-PLAN.md -- Batch translate all 92 MGGA functionals into sub-crates

### Phase 9: Reduce Kernel Build Time

**Goal:** Unblock all 25 previously-deferred GGA functionals at full derivative-order coverage (exc/vxc/fxc/kxc/lxc, polarized + unpolarized) under the default `cargo build`, while preserving 1e-12 oracle parity vs libxc 7.0.0; maintain forward-guard caps (≤20,000 lines per generated `.rs` file) and profile single-source-of-truth. Per Round-4 SPEC revision: family feature gates and the default-build wall-clock target (≤900s as of 2026-04-29; originally ≤180s, relaxed by user directive) are deferred to a future phase.
**Requirements**: SPEC-09-R1, SPEC-09-R2, SPEC-09-R3
**Requirements note:** The three IDs above map to the locked requirements in `09-SPEC.md`. Legacy roadmap IDs `BUILD-OPT-01`/`02`/`03` (originally listed for this phase) status: `BUILD-OPT-01` was satisfied before Phase 9 started (sccache + `incremental = false` already in `.cargo/config.toml`); `BUILD-OPT-02` (default-build family scope, ≤900s wall-clock — relaxed from the original ≤180s on 2026-04-29 per user directive) and `BUILD-OPT-03` (family feature gates, cfg-gated re-exports) are deferred out of Phase 9 per SPEC Round 4 / CONTEXT D-05 — re-introduce in a future phase if/when desired.
**Depends on:** Phase 8 (kernel crates must exist)
**Success Criteria** (what must be TRUE — Round 4):
  1. All 25 previously-deferred GGA functionals compile at full derivative-order coverage under default `cargo build` with no `#[cfg(feature = "order-*")]` attributes
  2. No `.rs` file in crates/kernel-{lda,gga,mgga}*/src/ exceeds 20,000 lines
  3. No sub-crate Cargo.toml has a [profile.*] section (single-source-of-truth preserved)
  4. Every newly-unblocked (functional × derivative-order × spin) tuple passes oracle parity at strict 1e-12 vs libxc 7.0.0
**Plans:** 7 plans (3 already complete + 4 post-Round-4 — old 09-04 obsolete and archived)

Plans:
- [x] 09-01-PLAN.md — Translator shared-preamble + incremental-delta annotations (translate_lda_v2.py / translate_gga.py / translate_mgga.py) [archive-pre-round4/]
- [x] 09-02-PLAN.md — Re-translate all 239 functionals + LDA per-(level, spin) split [archive-pre-round4/]
- [x] 09-03-PLAN.md — GGA bin-pack into 59 letter-suffix sub-crates + MGGA into ~80 sub-crates (commit b5d4c742, no SUMMARY) [archive-pre-round4/]
- [obsolete] archived 09-04-PLAN.md — feature-gating + ≤180s target (obsoleted by Round-4 SPEC) [archive-pre-round4/]
- [x] 09-04-PLAN.md — Raise translator SPLIT_THRESHOLD 5K → 18K (D-06) + regenerate LDA/GGA/MGGA (D-07) + ≤20K cap forward-guard (Wave 1, autonomous)
- [x] 09-05-PLAN.md — Generate tools/audit_deferred_gga.py (D-12) + post-09-04 coverage audit + close any mod.rs/lib.rs gaps for the 25 deferred GGAs (Wave 2, autonomous)
- [ ] 09-06-PLAN.md — End-to-end `cargo check --workspace` (D-13) + re-run all SPEC §Acceptance Criteria commands → log/09-06-spec-acceptance.log (Wave 3, autonomous)
- [ ] 09-07-PLAN.md — Oracle parity full sweep (D-14): 25 deferred GGAs at strict 1e-12 + MGGA non-regression spot-check via verify/tests/parity_phase09.rs (Wave 4, autonomous)

### Phase 10: Workspace-Level Modular Split

**Goal:** Refactor the root `libxc_rs` crate into a layered Cargo workspace where types and metadata are separated from compute orchestration by compiler-enforced crate boundaries. After this phase, `libxc-core` (model, meta, registry, input, output, layout, dims) contains zero compute logic and zero CubeCL imports; `libxc-eval` (eval, functional, kernel glue, workspace) depends one-way on `libxc-core`; `libxc-compat` (extern "C" shim) depends on both but is depended on by neither; the root `libxc_rs` crate becomes a thin facade over `api/` re-exporting a curated public surface.
**Driver:** Coupling / unclear boundaries — eval orchestration currently reaches into model and registry types with no compiler wall. Captured via /gsd-explore on 2026-05-07. See `.planning/notes/workspace-modular-architecture.md`.
**Depends on:** Phase 6 (Public API and C Compatibility) being at a stable seam — best executed after 06-02a settles to avoid colliding with the in-flight extern "C" wrapper work.
**Pre-planning blockers:**
  1. Resolve `src/error/` and `src/math/` placement (todo: `audit-error-math-placement`)
  2. Answer generated-files research question (`.planning/research/questions.md`) — affects xtask codegen flow
**Success Criteria** (what must be TRUE):
  1. Four target crates exist: `crates/libxc-core`, `crates/libxc-eval`, `crates/libxc-compat`, plus the root `libxc_rs` facade
  2. `cargo tree -p libxc-core` shows zero CubeCL or kernel-* dependencies (pure data layer)
  3. `cargo tree -p libxc-eval` shows `libxc-core` as a dependency but `libxc-compat` is not in the dependency closure
  4. `cargo tree -p libxc-compat` shows `libxc-eval` and `libxc-core` as dependencies; nothing depends on `libxc-compat` except the cdylib output
  5. The root `libxc_rs` crate's public surface is unchanged from a downstream consumer's perspective (existing `use libxc_rs::...` paths still resolve via re-export)
  6. All existing tests pass: `cargo test --workspace` matches pre-refactor pass/fail set
  7. Oracle parity preserved: `verify/` regression sweep at 1e-12 strict relative error on representative LDA/GGA/MGGA functionals
  8. `cargo build --workspace` succeeds with zero new warnings
**Plans:** TBD (will be decomposed during /gsd-plan-phase)

Plans:
**Wave 2**
- [ ] 10-01-PLAN.md — TBD: extract libxc-core (pure data layer)

**Wave 3** *(blocked on Wave 2 completion)*
- [ ] 10-02-PLAN.md — TBD: extract libxc-eval (orchestration layer)

**Wave 4** *(blocked on Wave 3 completion)*
- [ ] 10-03-PLAN.md — TBD: extract libxc-compat (FFI shim) and reduce root to facade

### Phase 11: Splitter v2 — Unified Kernels with 5K Line Cap

**Goal:** Re-engineer the Maple → CubeCL conversion pipeline (`tools/translate_{lda_v2,gga,mgga}.py` and helpers) so that two invariants hold simultaneously: (a) per-family sub-crates are collapsed — no more `crates/kernels/{lda,gga,mgga}-N/` numbered parents; one kernel-family crate per family. (b) Every emitted Rust file under the kernel crates is ≤5,000 lines, with the splitter extended to subdivide single output expressions (the current 8–15K floor for r4scan, br89_explicit, etc.) into `#[cube]` helper functions following the CubeCL macro fan-out manual. The phase is done when both invariants hold AND `cargo build --workspace` succeeds AND oracle parity is preserved.

**Driver:** Reduce CubeCL macro fan-out and per-crate compile/RAM cost; consolidate accumulated splitter complexity. Locked decisions captured in 11-CONTEXT.md from the discuss-phase that produced this entry. Supersedes the in-progress quick task `.planning/quick/260513-8nv-update-splitter-tool-enforce-3000-line-c` (which targeted a 3000-line cap and is now abandoned).

**Depends on:** Open — relationship to Phase 10 (workspace modular split) is one of the gray areas being discussed.

**Pre-planning blockers:**
  1. Decide expression-subdivision strategy (CSE-aware? AST-level? per-statement?) — research-grade
  2. Define cross-file ABI for subdivided expressions under CubeCL macro fan-out manual constraints
  3. Decide naming/dispatch for functionals that span N files (numbered suffix vs single file with internal modules)
  4. Define verification gate (bit-exact, 1e-12, energy-only) post-re-translation

**Success Criteria** (what must be TRUE):
  1. `find crates/kernels -maxdepth 1 -type d` shows **no** `lda-N`, `gga-N`, `mgga-N` numbered children — only family-level crates
  2. `find crates/kernels -name '*.rs' -exec wc -l {} +` shows zero files >5,000 lines
  3. The splitter (`tools/translate_*.py`) is capable of subdividing a single output expression into multiple `#[cube]` helper functions; r4scan, br89_explicit, mgga-{8,9,11} sized cases all fit under 5K
  4. `cargo build --workspace` succeeds on the user's RAM-constrained machine (inline sequential, `cargo` jobs ≤ 2)
  5. Oracle parity preserved at the gate level chosen during discussion
  6. Pipeline is idempotent: running it twice produces no diff
  7. CubeCL macro fan-out audit clean: `#[cube(launch)]` count does not increase from pre-phase baseline (per cubecl_macro_fanout_manual.md §3, §19)

**Plans:** TBD (will be decomposed during /gsd-plan-phase after research)

**Canonical refs:**
  - docs/manual/Cubecl/cubecl_macro_fanout_manual.md
  - libxc-master/maple/ (Maple source)
  - tools/translate_lda_v2.py, tools/translate_gga.py, tools/translate_mgga.py (current splitter, SPLIT_THRESHOLD=6000)
  - tools/split_oversized_{kernel,mgga}.py, tools/split_mgga_7_kcis.py, tools/rebatch_mgga.py (post-split helpers)
  - .planning/phases/11-splitter-v2-unified-5k-cap/11-CONTEXT.md (this phase's locked decisions)
