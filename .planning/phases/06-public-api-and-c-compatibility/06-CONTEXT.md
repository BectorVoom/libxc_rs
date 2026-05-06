# Phase 6: Public API and C Compatibility - Context

**Gathered:** 2026-05-06
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 6 delivers the **Layer-3 ergonomic Rust API** and the **Layer-1 C compatibility layer** — the two outer rings of the three-layer architecture, wrapping the `Functional` runtime handle that Phase 5 shipped.

Concretely:

1. **`api::batch::BatchEvaluator`** — workspace-only batch driver that wraps `EvaluationWorkspace` with a public name and an auto-dispatch entry point `BatchEvaluator::evaluate<I: EvaluateInput>(&Functional, &I, DerivativeOrder, &mut I::Output<'_>)`. Reusable across functionals on the same grid.
2. **`api::builder::FunctionalBuilder`** — chained-config sugar over `Functional::new(id, spin)` + Phase-5 setters. Final `.build() -> Result<Functional>` returns the same Functional handle that Phase 5 ships. (Exact chain shape is planner's discretion within the constraints below.)
3. **`api::evaluate::EvaluateInput` sealed dispatch trait** — three impls (LdaInput, GgaInput, MggaInput), each owning a `dispatch(&self, &Functional, order, &mut Self::Output, &mut Workspace) -> Result` that calls the family-specific Phase-5 method. BatchEvaluator's `evaluate` is one line that forwards to `input.dispatch(...)`.
4. **`compat::*` — all ~83 extern "C" functions in `libxc-master/src/xc.h`**. `xc_func_type*` is opaque; behind it sits a `Box<FunctionalSlot>` with two states (Empty / Initialized(Functional)). Lifecycle mirrors libxc's two-phase `alloc → init → end → free`. `xc_func_info_type*` is also opaque (points to `&'static FunctionalMeta`); all introspection goes through `xc_func_info_get_*` accessors.
5. **All extern "C" entry points return `int`** (0 ok, negative = typed `LibxcRsError` code). Panic-safe via `catch_unwind` at every boundary. NULL output pointers map to `Option::None` per libxc's "skip this derivative" idiom.
6. **Unsafe code budget**: confined to `compat/`, `kernel/launch.rs`, and GPU buffer management (per BUILD-04 + COMPAT-03). The Layer-3 `api/` module contains zero `unsafe`.

**In scope:**

- New module trees under `src/api/` (replacing today's 2-line placeholders): `batch.rs`, `builder.rs`, `evaluate.rs` (sealed trait + impls), `mod.rs` re-exports.
- New module trees under `src/compat/` (replacing today's 2-line placeholders): the existing files (`c_layout.rs`, `ids.rs`, `legacy_eval.rs`, `raw_handle.rs`, `removed.rs`) get populated; new files added as the planner sees fit. Final shape covers every function in `libxc-master/src/xc.h`.
- `FunctionalSlot` type (Empty / Initialized(Functional)) inside `compat/raw_handle.rs` (or equivalent), plus the alloc/init/end/free machinery.
- Thread-local errno mechanism (`xc_rs_last_error_code() -> i32`, `xc_rs_last_error_message() -> *const c_char`) — new accessors added to the C header. Existing libxc functions that originally returned `void` are changed to return `int`; this is the one signature-level departure from strict drop-in (see D-A4-1).
- C header generation (cbindgen or hand-written) covering every extern "C" symbol; committed to the repo.
- A handful of integration tests that exercise the FFI boundary from a C-or-Rust test harness (planner picks the cheapest credible mechanism).
- A `BatchOverflow` `LibxcRsError` variant for the fixed-`np_max` evaluator policy.

**Out of scope:**

- GPU backend selection / `LIBXC_RS_BACKEND` env var (Phase 7, GPU-07).
- Performance benchmarks (Phase 7, PERF-01..05).
- VV10 / 1D / 2D / OEP / LCA — already PROJECT.md "Out of Scope".
- Adding any new functional or kernel — Phase 6 wraps existing `Functional` capability, does not extend it.
- Changing Phase-5's `Functional::evaluate_{lda,gga,mgga}` shape. Those stay as the typed lower-level path; tests in `verify/tests/{lda,gga,mgga}_oracle.rs` continue to call them directly.
- Rustdoc polish + migration guide (DOC-01..03 are deferred to v2).

</domain>

<decisions>
## Implementation Decisions

### Area 1 — `xc_func_type` C struct & lifecycle (COMPAT-02)

- **D-A1-1 (struct exposure):** `xc_func_type` is **opaque** at the C boundary. The C header declares only `typedef struct xc_func_type xc_func_type;` (forward declaration). The pointer secretly references a `Box<FunctionalSlot>` allocated and managed by Rust. C/Fortran code that introspects fields directly (e.g. `p->cam_omega`, `p->info->kind`) will not compile against our header — those callers must migrate to `xc_hyb_cam_coef(p, ...)`, `xc_func_get_info(p)`, etc. accessor functions. This is the conscious trade: accessor-only C surface in exchange for zero ABI coupling and all `unsafe` confined to `compat/`.

- **D-A1-2 (lifecycle):** **Two-phase typed slot** mirroring libxc exactly.
  ```
  xc_func_alloc()      -> Box::into_raw(Box::new(FunctionalSlot::Empty))   as *mut xc_func_type
  xc_func_init(p,id,n) -> writes FunctionalSlot::Initialized(Functional::new(id,spin)?) into *p
  xc_func_end(p)       -> resets *p to FunctionalSlot::Empty (drops inner Functional)
  xc_func_free(p)      -> drops the Box (Box::from_raw(p as *mut FunctionalSlot))
  ```
  `xc_func_init` may be re-run after `xc_func_end` (matches libxc's documented contract). Calling `xc_func_init` on an already-initialized slot is **not** an error — it overwrites (matches libxc).

- **D-A1-3 (threading):** **Match libxc's de-facto contract — single-threaded per handle.** No interior mutability inside the slot. Setter functions `unsafe { (&mut *p).as_mut_initialized()?.set_*(...) }`. Caller is responsible for synchronization across threads. Rust's `Functional` is `Send + Sync` underneath (Phase 5 D-13), so users who *do* synchronize properly get sound concurrent reads, but the contract on the C side is "one thread per handle at a time."

- **D-A1-4 (xc_func_info_type):** Symmetric with D-A1-1 — `xc_func_info_type*` is **also opaque, accessor-only**. The pointer is `&'static FunctionalMeta as *const xc_func_info_type` — zero allocation, lifetime forever. All introspection goes through `xc_func_info_get_name(info)`, `xc_func_info_get_kind(info)`, `xc_func_info_get_n_ext_params(info)`, etc. C code that does `info->name` directly will not compile — same migration story as D-A1-1.

### Area 2 — BatchEvaluator scope & ownership (API-02)

- **D-A2-1 (ownership):** **Workspace only.** `BatchEvaluator` owns an `EvaluationWorkspace` plus the (family, spin, np_max) it was constructed with. It does **not** own a Functional. Caller passes `&Functional`, `&Input`, `&mut Output` on every `evaluate` call. Pro: the same evaluator drives any compatible (family/spin) Functional on the same grid — natural fit for SCF loops, oracle harnesses, and benchmarking sweeps that loop over many functionals.

- **D-A2-2 (resize policy):** **Fixed `np_max` at construction; `BatchOverflow` error on overflow.** No amortized-doubling growth. Preserves PERF-05's "zero heap allocation in non-mixed evaluation hot path" as a hard invariant. Callers who don't know peak np in advance must size for worst-case; this is the conscious trade for predictability. (If Phase 7 benchmarks reveal a real DX problem, an opt-in `Builder::allow_grow(true)` can be added without breaking the default contract.)

- **D-A2-3 (multi-functional):** **Workspace shared across functionals.** Because BatchEvaluator owns only the workspace (D-A2-1), a single instance can drive any functional that matches its (family, spin, np_max) — e.g. evaluate `gga_x_pbe` then `gga_c_pbe` on the same input grid using the same workspace. This is the explicit motivation for choosing the workspace-only ownership model; without it, oracle/benchmark harnesses pay N times the workspace cost.

- **D-A2-4 (API-03 location):** **API-03 (`evaluate()` auto-dispatch by family) lives on BatchEvaluator**, not on Functional. Surface: `be.evaluate(&functional, &input, order, &mut out)` where `input: &impl EvaluateInput`. Phase 5's `Functional::evaluate_{lda,gga,mgga}` stay public as the typed lower-level path used by `verify/tests/*` — no Phase-5-test churn.

### Area 3 — Ergonomic `evaluate()` auto-dispatch (API-03)

- **D-A3-1 (trait shape):** **Dispatch trait** — each `EvaluateInput` impl owns the family-specific dispatch call.
  ```rust
  pub trait EvaluateInput: sealed::Sealed {
      type Output<'a>;
      fn dispatch(
          &self,
          functional: &Functional,
          order: DerivativeOrder,
          output: &mut Self::Output<'_>,
          workspace: &mut EvaluationWorkspace,
      ) -> Result<(), LibxcRsError>;
  }
  // impl EvaluateInput for LdaInput { ... -> functional.evaluate_lda(...) }
  // impl EvaluateInput for GgaInput { ... -> functional.evaluate_gga(...) }
  // impl EvaluateInput for MggaInput { ... -> functional.evaluate_mgga(...) }
  ```
  BatchEvaluator's evaluate is one line: `input.dispatch(functional, order, output, &mut self.ws)`. **Zero `unsafe`** — type-correct by construction (rejected the alternative "vocabulary trait + match in evaluator" because it requires an unsafe transmute to satisfy the borrow checker). Family mismatch (`functional.meta().family != Family::Lda` for an LdaInput) is checked inside each impl and returned as `LibxcRsError::FamilyMismatch`.

- **D-A3-2 (borrow):** **`&Functional`** — evaluation is read-only. Allows `Arc<Functional>` shared across threads, lets multiple BatchEvaluators share one Functional, matches Phase 5's `&self` evaluate methods exactly. Callers who need to mutate Functional (set thresholds, ext_params) do so separately before calling `BatchEvaluator::evaluate`.

### Area 4 — Compat-layer error/panic boundary (COMPAT-01)

- **D-A4-1 (error convention):** **`int` return codes everywhere + thread-local errno** for typed error retrieval. Every extern "C" function returns `int` (0 = ok, negative = typed `LibxcRsError` discriminant). For libxc functions that originally returned `void` (`xc_lda_exc`, `xc_func_set_dens_threshold`, `xc_aux_func_ids`, ...), **we change the signature to `int`** — this is the **one place Phase 6 departs from strict drop-in**. Existing C call sites that don't capture the return value still compile and run (C lets you ignore int returns); Fortran call sites that bound the C function as `void` need a one-line subroutine-vs-function fix when picking up the new header. Trade accepted: type-checked error reporting beats silent failure.
  - Two new accessor functions added to the header: `xc_rs_last_error_code() -> i32` and `xc_rs_last_error_message() -> *const c_char` (pointing into a thread-local `CString`). The thread-local is set on every error path inside the catch_unwind boundary (D-A4-2).

- **D-A4-2 (panic boundary):** **`catch_unwind` at every extern "C" entry point.** Every extern "C" function wraps its body in `std::panic::catch_unwind`; on caught panic, the panic message is captured into the thread-local errno mechanism (D-A4-1) and the function returns a designated `LIBXC_RS_PANIC` error code. UB-free even if a kernel panics, an `assert!` in `src/` trips, or a Rust-side invariant breaks. Per-call overhead is one register save — negligible in the evaluation hot path. Rejected `extern "C-unwind"` because it couples our ABI to the caller's runtime.

- **D-A4-3 (NULL handling):** **NULL `*mut f64` → `Option::None`** at the FFI boundary. The compat layer maps each C buffer pointer to `Option<&mut [f64]>` for our typed `LdaOutput`/`GgaOutput`/`MggaOutput` constructors. Buffer length is computed from `np` + the per-family `Dimensions` lookup that Phase 1 already provides. Preserves Phase 3 D-05 (OutputMask semantics) and matches libxc's "pass NULL to skip this derivative" idiom exactly. No per-function NULL-ability table needed; the typed Output bundle handles it uniformly.

- **D-A4-4 (uninitialized handle):** **Return an error code (and set thread-local errno).** Any extern "C" function that operates on an `xc_func_type*` checks `FunctionalSlot::Initialized` at entry; on `FunctionalSlot::Empty` (alloc'd but never `xc_func_init`'d, or after `xc_func_end`), set errno = `LIBXC_RS_UNINITIALIZED_HANDLE` and return early. No abort, no UB — caller can recover. Aligns with D-A4-1's "everything returns int" convention.

### Claude's Discretion

These are intentionally not pinned — the planner and downstream agents choose:

- **Plan decomposition** across the 3 plans the roadmap allocates to Phase 6. Suggested split (planner may revise): **06-01** `api::evaluate::EvaluateInput` trait + 3 impls + `api::batch::BatchEvaluator` + `api::builder::FunctionalBuilder` + Layer-3 unit tests (API-01..03); **06-02** `compat::*` core lifecycle (`xc_func_alloc/init/end/free`), `FunctionalSlot`, threading machinery, error+panic boundary, threshold + ext_param setters, hybrid + aux accessors, info-struct accessors (~40 of the 83 functions); **06-03** the rest of `compat/*` (evaluation functions, library info, NULL-handling for `xc_lda*/xc_gga*/xc_mgga*` families, integration test from a C harness) — closes COMPAT-01.
- **Exact `FunctionalBuilder` chain shape** — owned `self`-by-move chain (`.spin(...).density_threshold(...).build()`) or `&mut self` chain (`b.spin(...); if cfg { b.density_threshold(...); }; b.build()`). Both are valid wrappers over Phase 5's setters. Planner picks based on what reads better against the existing `Functional::new + setters` shape.
- **Whether the 83 extern "C" functions are hand-written or partially codegen'd from `libxc-master/src/xc.h`** — both are acceptable. If hand-written, group by C-API category (lifecycle / thresholds / ext_params / evaluation × 3 families × 5 derivative orders / hybrid+aux / info / discovery / library). If codegen'd, an xtask reading the header is the natural pattern (matches Phase 1 D-04 and Phase 5 D-03).
- **C header generation strategy** — cbindgen, hand-written `.h`, or a hybrid where lifecycle/discovery is hand-written and the 33 evaluate functions are codegen'd. Whichever produces the lowest-churn header on libxc version bumps.
- **`LdaScratch`/`GgaScratch`/`MggaScratch` reuse inside BatchEvaluator** — already-materialized in Phase 5; planner just wires them through.
- **Exact `LibxcRsError` → `int` discriminant mapping** — keep close to `LibxcRsError` enum order; document the mapping in the C header.
- **Integration test mechanism** — a Cargo build script that compiles a 50-line C file and links it against the library, OR a Rust test that calls our extern "C" functions through their FFI signature. The latter is simpler (no toolchain coupling) but covers less ground (no header compile-test).
- **Exact filenames inside `compat/`** — the 5 existing placeholders (`c_layout`, `ids`, `legacy_eval`, `raw_handle`, `removed`) suggest the intended carve. Planner may rename, split, or merge as the implementation reveals natural boundaries.
- **Whether `removed.rs` returns errno = `LIBXC_RS_REMOVED` or maps to the existing typed error variant `RemovedFunctionalId` from Phase 1**. Both fit; pick the one that surfaces the replacement-id cleanly to the C caller.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Roadmap and Requirements

- `.planning/ROADMAP.md` §Phase 6 — goal, requirements API-01..03 + COMPAT-01..03, success criteria 1-5, 3 plans allocated
- `.planning/REQUIREMENTS.md` §High-Level API + §C Compatibility Layer — full requirement text
- `.planning/PROJECT.md` §Constraints + §Key Decisions — three-layer API, thiserror v2 at boundary, BUILD-04 unsafe budget, "drop-in replacement for libxc in C/Fortran DFT codes"

### Design Document (primary spec)

- `docs/design/libxc_rs_detailed_design.md` §4.1 — Three-layer API architecture diagram + rationale
- `docs/design/libxc_rs_detailed_design.md` §5.1–§5.9 — Full libxc C API → Rust mapping table (every public C function we must expose)
- `docs/design/libxc_rs_detailed_design.md` §6.8 — `Functional` struct (already shipped by Phase 5; Layer 3 wraps it)
- `docs/design/libxc_rs_detailed_design.md` §6.9 — `EvaluationWorkspace` reuse strategy (BatchEvaluator wraps this)
- `docs/design/libxc_rs_detailed_design.md` §15 — Error enum (extend with `BatchOverflow`, `FamilyMismatch`, plus the panic / uninitialized-handle codes)
- `docs/design/libxc_rs_detailed_design.md` §20 — Source tree (`api/`, `compat/` module layouts)
- `docs/design/libxc_rs_detailed_design.md` §22.3 — Alternatives considered: "Single Functional struct over Generic<F: Family>" (relevant for sealed trait shape)
- `docs/design/libxc_rs_detailed_design.md` Appendix A — Covered API scope (the authoritative list of public C functions in scope)

### libxc Reference Implementation

- `libxc-master/src/xc.h` — **Authoritative list** of every public C function we must implement (≈83 functions). Every extern "C" symbol in `compat/` must match a declaration here.
- `libxc-master/src/funcs_key.c` / `funcs_lda.c` / `funcs_gga.c` / `funcs_mgga.c` — function-name ↔ id tables; useful for `xc_functional_get_number`/`xc_functional_get_name` (already covered by Phase 1's static registry).
- `libxc-master/src/xc.c` — implementations of `xc_func_alloc`, `xc_func_init`, `xc_func_end`, `xc_func_free`, `xc_func_set_*`. Reference for the two-phase lifecycle semantics (D-A1-2).
- `libxc-master/src/work_lda.c`, `work_gga_new.c`, `work_mgga_new.c` — reference for how `xc_lda_exc`, `xc_gga_vxc_fxc`, etc. dispatch to the per-derivative-order kernel; informs how compat's evaluation functions choose the OutputMask before calling Phase-5 `Functional::evaluate_*`.
- `libxc-master/src/hybrids.c` — `xc_hyb_type`, `xc_hyb_exx_coef`, `xc_hyb_cam_coef`, `xc_nlc_coef`, `xc_num_aux_funcs`, `xc_aux_func_ids`, `xc_aux_func_weights`. Phase 5 already ports the math; Phase 6 wraps the Rust methods in extern "C".

### Current libxc_rs Code (what Phase 6 touches)

- `src/lib.rs` — Re-exports the public API surface. Phase 6 adds `pub use api::{BatchEvaluator, FunctionalBuilder, EvaluateInput};` and ensures `compat::*` is gated behind `pub mod compat;` (already declared).
- `src/api/mod.rs` — Currently `pub mod batch; pub mod builder;`. Phase 6 adds `pub mod evaluate;` for the `EvaluateInput` trait + impls.
- `src/api/batch.rs` — 2-line placeholder. Phase 6 implements `BatchEvaluator` per D-A2-1..4.
- `src/api/builder.rs` — 2-line placeholder. Phase 6 implements `FunctionalBuilder` (chain shape is planner's discretion).
- `src/compat/mod.rs` — Currently re-exports 5 placeholder modules. Phase 6 populates each.
- `src/compat/raw_handle.rs` — Natural home for `FunctionalSlot { Empty, Initialized(Functional) }` + `xc_func_alloc/init/end/free`.
- `src/compat/c_layout.rs` — Forward-declared opaque `xc_func_type`/`xc_func_info_type` types + repr/layout assertions.
- `src/compat/ids.rs` — Discovery functions (`xc_functional_get_number`, `xc_functional_get_name`, `xc_family_from_id`, `xc_number_of_functionals`, etc.) — wraps Phase 1's registry.
- `src/compat/legacy_eval.rs` — Natural home for the 33 evaluation functions (`xc_lda_exc`, `xc_lda_vxc`, `xc_lda_exc_vxc_fxc_kxc`, etc. across LDA/GGA/MGGA × 5 derivative orders × NULL-handling combinations). Each one builds a typed Input/Output bundle from the C buffers and forwards to `Functional::evaluate_{lda,gga,mgga}`.
- `src/compat/removed.rs` — Maps removed functional IDs to typed `RemovedFunctionalId` errors (Phase 1 D-08 already shipped this for the typed API; compat exposes via the int errno).
- `src/error/mod.rs` — Extend `LibxcRsError` with new variants: `BatchOverflow { requested, capacity }`, `FamilyMismatch { expected, actual }`, `UninitializedHandle`, `Panicked { message }`. Add a `discriminant() -> i32` method (or const lookup table) for the C errno mapping.
- `src/eval/workspace.rs` — `EvaluationWorkspace` is already public-by-virtue-of-being-used-by-tests; `BatchEvaluator` wraps it. No changes needed.
- `src/functional/{mod,lifecycle,config,evaluate,hybrid}.rs` — All Phase-5 surface stays exactly as-is; Layer 3 and compat both wrap it without modification.
- `src/input/*` and `src/output/*` — `LdaInput/GgaInput/MggaInput` and `LdaOutput/GgaOutput/MggaOutput` are the constructor targets for `compat::legacy_eval`. They already validate buffer sizes (Phase 3 D-02).
- `src/dims/mod.rs` — `Dimensions::lda/gga/mgga(spin)` is the source of truth for buffer lengths in compat's NULL-handling logic.

### Prior Phase Context (decisions that carry forward)

- `.planning/phases/01-foundation-and-registry/01-CONTEXT.md` — Phase 1 D-04 (xtask-generated committed Rust output — pattern available for compat header gen if planner picks codegen), D-08 (typed `LibxcRsError` with `RemovedFunctionalId`), Static registry shape (compat::ids wraps it).
- `.planning/phases/03-input-output-and-evaluation-framework/03-CONTEXT.md` — Phase 3 D-05 (OutputMask + Option<&mut> semantics — exactly what NULL→None maps to in D-A4-3), D-06 (caller-provided buffers, zero allocation), D-12/D-13 (workspace sizing — what BatchEvaluator wraps).
- `.planning/phases/05-functional-lifecycle-and-hybrid-properties/05-CONTEXT.md` — Phase 5 D-06 (ext_params storage), D-11 (free `dispatch_*` functions stay public — compat may use either Functional methods or direct dispatch), D-13 (Functional is `Send + Sync` — compat can rely on this), D-15..D-17 (aux + hybrid semantics — compat's hybrid accessors wrap these).
- `.planning/phases/04-bulk-kernel-translation/04-CONTEXT.md` — Phase 4 deferred functionals (4 LDA + 6 MGGA). compat must surface the deferred-functional error to the C caller via the int errno mechanism.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `Functional` struct (Phase 5, `src/functional/`) — Phase 6's central wrapping target. Public surface: `new(id, spin)`, `evaluate_{lda,gga,mgga}`, `set_density_threshold`, `set_zeta_threshold`, `set_sigma_threshold`, `set_tau_threshold`, `set_ext_params`, `set_ext_param`, `set_ext_param_by_index`, `ext_params`, `ext_param`, `ext_param_by_index`, `hybrid_type`, `exx_coefficient`, `cam_coefficients`, `nlc_coefficients`, `auxiliary_functionals`, `mix_coefficients`, `meta`, `spin`, `dims`, `thresholds`, `params`. **Both Layer 3 and compat wrap this — no Functional changes needed in Phase 6.**
- `EvaluationWorkspace` (`src/eval/workspace.rs`, Phase 3 + 5) — single contiguous `Vec<f64>` sized for MGGA-superset, `LdaScratch`/`GgaScratch`/`MggaScratch` views already materialized. BatchEvaluator literally owns one of these. The `EvaluationWorkspace::new(np, spin)` constructor is the model for `BatchEvaluator::new`.
- `LdaInput`/`GgaInput`/`MggaInput` constructors (`src/input/*`) — validate buffer sizes against `Dimensions`. compat's `xc_lda*/xc_gga*/xc_mgga*` build these from raw pointers.
- `LdaOutput`/`GgaOutput`/`MggaOutput` constructors with `Option<&mut [f64]>` per derivative (`src/output/*`) — direct target for D-A4-3's NULL→None mapping.
- `Dimensions::lda/gga/mgga(spin)` (`src/dims/mod.rs`) — per-family per-spin buffer-length lookup. compat uses this to compute slice lengths from raw pointers + np.
- `LibxcRsError` (`src/error/mod.rs`) — already covers most error cases; Phase 6 adds `BatchOverflow`, `FamilyMismatch`, `UninitializedHandle`, `Panicked` and a discriminant-to-int mapping.
- Static registry (`src/registry/mod.rs`, Phase 1) — `lookup_by_id`, `lookup_by_name`, `functional_count`. compat::ids wraps each of these one-to-one with a `xc_*` extern "C" function.
- `FunctionalMeta` (`src/meta/mod.rs`, populated by Phase 5 D-01..D-05) — `&'static FunctionalMeta` is exactly the pointee for `xc_func_info_type*` per D-A1-4. Zero allocation.

### Established Patterns to Continue

- **thiserror v2 at the library boundary** — `LibxcRsError` extensions use the same `#[error("...")]` shape as Phase 5.
- **`Send + Sync` everywhere** (Phase 5 D-13) — `BatchEvaluator` and the FFI types must compile-test as `Send + Sync` if they hold any references; `FunctionalSlot` should be too (it owns a `Functional` which is `Send + Sync`).
- **No `#[deny(warnings)]` regressions** — every new module compiles clean.
- **Edition 2024, MSRV 1.85+** — keep.
- **Committed xtask-generated output** (Phase 1 D-04) — if Phase 6 codegens the C header or any extern "C" function bodies, output is committed (no build.rs runtime gen).

### Integration Points

- `Cargo.toml` — Phase 6 adds NO new production dependencies. The compat layer uses only `std::ffi::{c_char, c_int, CString}` and `std::panic::catch_unwind` — both in core std. Optional dev-dep: `cc` 1.x for the C-harness integration test (planner discretion).
- `verify/` — Phase 6's compat layer should not break any existing oracle test. The natural Phase-6 verify addition is a tiny C/Rust integration test that exercises the FFI surface (planner picks the cheapest credible mechanism).
- `xtask/` — If header generation or extern "C" body generation goes through xtask, add a new subcommand alongside Phase 5's `generate-metadata`.

### What Phase 6 Creates

- `src/api/evaluate.rs` (new) — `EvaluateInput` sealed trait + 3 impls
- `src/api/batch.rs` (rewrite from placeholder) — `BatchEvaluator`
- `src/api/builder.rs` (rewrite from placeholder) — `FunctionalBuilder`
- `src/api/mod.rs` (update) — add `pub mod evaluate;` + re-exports
- `src/compat/raw_handle.rs` (rewrite from placeholder) — `FunctionalSlot`, opaque `xc_func_type` typedef, alloc/init/end/free
- `src/compat/c_layout.rs` (rewrite from placeholder) — opaque type forward decls, layout/size assertions, `repr(C)` invariants
- `src/compat/legacy_eval.rs` (rewrite from placeholder) — the 33 evaluate functions + threshold/ext_param setters
- `src/compat/ids.rs` (rewrite from placeholder) — discovery functions
- `src/compat/removed.rs` (rewrite from placeholder) — removed-id error mapping
- `src/compat/info.rs` or similar (likely new) — `xc_func_info_get_*` accessors
- `src/compat/hybrid.rs` or similar (likely new) — `xc_hyb_type`, `xc_hyb_exx_coef`, `xc_hyb_cam_coef`, `xc_nlc_coef`, `xc_num_aux_funcs`, `xc_aux_func_ids`, `xc_aux_func_weights`
- `src/compat/library.rs` or similar (likely new) — `xc_version`, `xc_version_string`, `xc_reference*`
- `src/compat/errno.rs` or similar (likely new) — thread-local errno + `xc_rs_last_error_code/message` accessors
- `src/error/mod.rs` (extend) — new variants + discriminant mapping
- C header file (likely `compat/include/xc.h` or `target/include/xc.h`, planner picks) — generated or hand-written; committed
- `verify/tests/compat_smoke.rs` or similar (new) — minimum-viable integration test exercising FFI

</code_context>

<specifics>
## Specific Ideas

- **`FunctionalSlot` shape** — likely the simplest possible:
  ```rust
  #[repr(C)]
  pub struct xc_func_type { _opaque: [u8; 0] }   // forward decl in C, zero-size in Rust

  #[repr(C)]
  enum FunctionalSlot {
      Empty,
      Initialized(Functional),
  }
  // alloc -> Box::into_raw(Box::new(FunctionalSlot::Empty)) as *mut xc_func_type
  // init  -> (*p as *mut FunctionalSlot).write(FunctionalSlot::Initialized(Functional::new(...)?))
  // end   -> (*p as *mut FunctionalSlot).write(FunctionalSlot::Empty)
  // free  -> drop(Box::from_raw(p as *mut FunctionalSlot))
  ```
  Pointer cast `*mut xc_func_type ↔ *mut FunctionalSlot` is layout-safe because the C type is opaque (zero-size forward decl).

- **Errno discriminant mapping** — keep close to declaration order in `LibxcRsError`. Suggested negative-int contract: `LIBXC_RS_OK = 0`, `LIBXC_RS_PANIC = -1`, `LIBXC_RS_UNINITIALIZED_HANDLE = -2`, `LIBXC_RS_UNKNOWN_FUNCTIONAL_ID = -3`, ... Document in the C header next to each function declaration so callers know which codes a given function can return.

- **`catch_unwind` macro** — write a single `extern_c_wrapper!` macro inside `compat/` that wraps any `Fn() -> Result<i32, LibxcRsError>` body in `catch_unwind`, sets thread-local errno on Err or panic, and returns the int code. Every extern "C" function uses it; ensures uniformity and one place to audit panic handling.

- **NULL→None for Output** — every `xc_lda_*`/`xc_gga_*`/`xc_mgga_*` evaluate function takes a fixed set of `*mut f64` args. The compat function:
  1. Reads `np` from the call.
  2. Looks up `dims = Dimensions::<family>(spin)`.
  3. For each `*mut f64` arg: `if ptr.is_null() { None } else { Some(unsafe { std::slice::from_raw_parts_mut(ptr, np * dims.<field>) }) }`.
  4. Constructs the typed Output bundle.
  5. Forwards to `Functional::evaluate_*` (or, if the planner prefers and it's cheaper, the free `dispatch_*` directly — Phase 5 D-11 keeps that public).
  6. Returns 0 on Ok, errno-int on Err.

- **`BatchEvaluator::new` signature** — bake (family, spin, np_max) at construction so workspace sizing is one-shot:
  ```rust
  impl BatchEvaluator {
      pub fn new(family: Family, spin: Spin, np_max: usize) -> Self {
          Self { ws: EvaluationWorkspace::new(np_max, spin /* MGGA-superset internally */), family, spin, np_max }
      }
  }
  ```
  Alternative: skip the family arg since EvaluationWorkspace already sizes for MGGA-superset (Phase 3 D-12). Planner picks; both are correct.

- **`FunctionalBuilder` chain** — recommended owned-self chain since it reads cleanest in DFT integration code:
  ```rust
  let f = FunctionalBuilder::new(FunctionalId::from_name("gga_x_pbe").unwrap())
      .spin(Spin::Polarized)
      .density_threshold(1e-12)
      .ext_param("alpha", 0.7)?
      .build()?;
  ```
  Trade for losing conditional chaining (`if cfg { b.density_threshold(t); }`); planner can override if integration patterns suggest `&mut self`.

- **Thread-local errno storage** — `std::cell::RefCell<Option<CString>>` inside `thread_local!`. The pointer returned by `xc_rs_last_error_message()` is valid until the next error call on the same thread; document this clearly. Returning a static "no error" CString when the cell is empty avoids null-pointer hazards.

- **Family mismatch error** — when a caller passes `LdaInput` to a Functional whose `meta().family == Family::Gga`, the impl's `dispatch` returns `LibxcRsError::FamilyMismatch { expected: Family::Gga, actual: Family::Lda }`. Same code path for the C side via the int errno.

- **Integration test minimum** — even a single Rust test that does `unsafe { xc_func_alloc() → xc_func_init(p, 1, 1) → xc_lda_exc(p, 4, rho.as_ptr(), zk.as_mut_ptr()) → xc_func_end(p) → xc_func_free(p) }` and compares zk against the typed-API result is enough to prove the FFI surface holds together. Add a C harness later if Phase 7 or Phase 10 polish wants header-compile testing.

- **Header generation policy** — strong recommendation: hand-write the C header (it's small, ~100 declarations, mirroring `libxc-master/src/xc.h` 1:1 minus the changed return types). cbindgen produces lots of noise for a stable, small surface. Commit the header at `include/xc_rs.h`; users include it instead of (or alongside) `xc.h`.

</specifics>

<deferred>
## Deferred Ideas

- **`extern "C-unwind"` migration** — if a future user explicitly needs panic-as-foreign-exception interop with C++, revisit D-A4-2.
- **`Builder::allow_grow(true)` for resizable BatchEvaluator workspaces** — only add if Phase 7 benchmarks reveal a real DX problem with the fixed `np_max` policy.
- **Multi-functional batch with summed output** (rejected scope creep) — `BatchEvaluator` summing `Vec<(Functional, weight)>` into a single output. Useful for some DFT codes but not in API-02. Track for v2.
- **GPU-resident BatchEvaluator** — Phase 7 (GPU-05) owns this. Phase 6's BatchEvaluator is CPU-only via `cpu_client()`; Phase 7 may add a backend-selection arg to `BatchEvaluator::new` (would be a non-breaking addition).
- **C++ `std::span`-style buffer types in the C header** — out of scope; raw pointers + length is the C contract.
- **Drop-in libxc binary compatibility** (literally substituting `libxc.so` at runtime) — explicitly **not** pursued. Drop-in here means source-level: recompile against our header, get our implementation. The signature-changing `void → int` decision (D-A4-1) makes binary compat impossible by design.
- **Migration guide / rustdoc polish** — DOC-01..03 are PROJECT.md v2 items; deferred to a documentation phase.

### Un-discussed areas the planner owns

- Plan decomposition across the 3 plans the roadmap allocates to Phase 6 (suggested split in Claude's Discretion).
- Exact `FunctionalBuilder` chain ergonomics (owned `self` vs `&mut self`).
- Whether the 83 extern "C" function bodies are hand-written or codegen'd from an xtask.
- C header generation tool / hand-written choice and committed location.
- Filenames inside `src/compat/` — the existing 5-file placeholder skeleton may or may not survive contact with the implementation.
- Integration test mechanism (Rust-only FFI exercise vs cc-built C harness vs both).
- Exact `LibxcRsError` discriminant → C errno integer mapping table.
- Whether `removed.rs` returns errno = `LIBXC_RS_REMOVED` or maps `RemovedFunctionalId`'s replacement-id payload through the thread-local message.

</deferred>

---

*Phase: 06-public-api-and-c-compatibility*
*Context gathered: 2026-05-06*
