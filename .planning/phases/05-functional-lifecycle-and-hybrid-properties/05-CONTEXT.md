# Phase 5: Functional Lifecycle and Hybrid Properties - Context

**Gathered:** 2026-04-24
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 5 delivers the **runtime Functional handle** and its hybrid-property queries. Concretely:

1. A `Functional` struct that owns spin + dimensions + thresholds + current ext_params + derived params + eager auxiliary Functionals, constructed via `Functional::new(id, spin) -> Result<Functional>`.
2. A `FunctionalParams` trait object (`Box<dyn FunctionalParams + Send + Sync>`) stored inside `Functional` that carries raw and derived per-functional parameters and is plumbed through `dispatch_{lda,gga,mgga}`.
3. Hybrid/NLC/aux queries (`hybrid_type`, `exx_coefficient`, `cam_coefficients`, `nlc_coefficients`, `auxiliary_functionals`) satisfying HYB-01..04.
4. Ergonomic evaluate methods (`Functional::evaluate_{lda,gga,mgga}`) that own mixed-evaluation routing and delegate to the existing free `dispatch_*` functions for the non-mixed kernel launch.
5. A populated static registry — the currently-empty `ext_params`, `hybrid_terms`, `nlc_params`, `auxiliaries`, `references`, `flags`, and `hybrid_type` fields in `FunctionalMeta` are filled for all 649 IDs by a new `cargo xtask generate-metadata` target that links libxc and snapshots `xc_func_type` at xtask-run time.
6. Full mixed-evaluation paths for **all three families** — `evaluate_mixed_gga` and `evaluate_mixed_mgga` materialize `GgaScratch`/`MggaScratch` (currently `PhantomData` placeholders) so B3LYP, wB97, and every other hybrid actually evaluates.

**In scope:**
- New xtask target that links libxc (via the factored-out `libxc-sys` workspace crate, extracted from `verify/build.rs`), iterates all 649 IDs, calls `xc_func_init`, and emits committed Rust tables into `src/meta/generated.rs` and a new `src/meta/generated_hybrid.rs` (or equivalent) — **full population including references/DOI/bibtex**.
- Port of `xc_hyb_type()` logic from `libxc-master/src/hybrids.c:82` into Rust, plus snapshotting the classification into `FunctionalMeta.hybrid_type` — and a verify/ test that confirms the Rust port and the snapshotted value agree.
- `FunctionalParams` trait (`Send + Sync`) with one concrete `impl` per compiled functional; per-functional ext_params wiring for **all 229 compiled functionals** (37 LDA + 106 GGA + 86 MGGA) — deferred LDA/MGGA IDs stay deferred.
- `Functional` struct and public methods (`set_density_threshold`, `set_ext_param`, `ext_params`, `hybrid_type`, `exx_coefficient`, `cam_coefficients`, `nlc_coefficients`, `auxiliary_functionals`, `evaluate_{lda,gga,mgga}`).
- Eager recursive aux construction with an xtask-generated propagation map (parent ext_param → aux ext_param); xtask validates aux depth ≤ 2 for all hybrids at snapshot time (no runtime cycle detection).
- New `LibxcRsError` variants for unknown ext_param name, ext_param index out of range, propagation conflict, and auxiliary-init failure.
- verify/ round-trip test: every `FunctionalMeta` field-compared against a fresh `xc_func_init` for every ID.

**Out of scope:**
- `FunctionalBuilder` and `BatchEvaluator` — those live in `src/api/{builder,batch}.rs`, are stubs today, and remain stubs until Phase 6.
- `extern "C"` compat-layer functions — Phase 6 owns `src/compat/*` which is currently placeholder.
- Enabling any deferred LDA (4) or MGGA (6) functional — still deferred from Phase 4.
- GPU backends beyond `cubecl/cpu` — Phase 7.
- `FunctionalBuilder` chainable config, ergonomic `evaluate()` auto-dispatch by family — Phase 6 (API-01, API-03).
- Performance benchmarks (PERF-01..05) — Phase 7.

</domain>

<decisions>
## Implementation Decisions

### Registry Metadata Population

- **D-01:** Metadata source = **xtask links libxc and snapshots `xc_func_type` at xtask-run time**. For each of 649 IDs the xtask calls `xc_func_init(&mut t, id, XC_UNPOLARIZED)`, reads `t.info->ext_params`, `t.hyb_number_terms`/`hyb_type`/`hyb_coeff`/`hyb_omega`, `t.func_aux[]` IDs, `t.nlc_b`/`nlc_C`, `t.info->flags`, `t.info->references[]`, and emits static Rust entries. The xtask process is the only thing that links libxc in the Phase 5 workflow — committed generated Rust output keeps `libxc_rs` itself FFI-free at runtime (preserves Phase 1 D-04's spirit).

- **D-02:** Snapshot scope = **ALL `FunctionalMeta` fields**, including `references` (citation / DOI / bibtex / key). This closes out Phase 1 D-05's "populate incrementally" carry-over in one pass. Smaller xtask pass is a false economy: the round-trip verify test (D-04) will tell us about any field we skip.

- **D-03:** xtask location = **new subcommand `cargo xtask generate-metadata`** inside the existing `xtask/` crate. A new workspace member `libxc-sys` is factored out of the current `verify/build.rs` so that **both** `verify/` (for oracle tests) and `xtask` (for metadata snapshotting) link the same libxc build — no duplicate cmake invocations, one source-of-truth bindgen. The existing `generate-registry` subcommand is preserved; it continues to parse `xc_funcs.h` for the ID/name/family/kind skeleton, and the new subcommand augments the same `generated.rs` file with the runtime-snapshotted fields.

- **D-04:** Validation gate = **verify/ round-trip test `verify/tests/metadata_oracle.rs`**. For every id in `lookup_all_ids()`, construct an `xc_func_type` via FFI, compare our static `FunctionalMeta` field-by-field. Fails loud on drift. Runs as part of `cargo test -p libxc_rs-verify`. No version-checksum mechanism — the round-trip test is the single source of truth.

- **D-05:** Regen policy = **manual**. Developer runs `cargo xtask generate-metadata` on libxc version bump; generated files committed. Matches Phase 1 D-04 (committed xtask output) and keeps `cargo build` of the main crate FFI-toolchain-free.

### Ext_params Storage and Plumbing

- **D-06:** Runtime ext_params storage on `Functional` = **`Option<Box<[f64]>>`**. `None` when `meta.ext_params.is_empty()` — which is the majority of functionals and preserves Phase 3 EVAL-04 (zero heap alloc in non-mixed hot path) for those. `Some(Box<[f64]>)` when the functional has ext_params; initialized from `meta.ext_params[i].default_value` at `Functional::new`.

- **D-07:** Dispatch signatures after Phase 5 = **`dispatch_{lda,gga,mgga}` take a `&dyn FunctionalParams` trait object** in addition to the existing `functional`/`input`/`order`/`output`/`thresholds` arguments. The trait object replaces the current typed `LdaFunctionalParams { alpha }` (LDA) and the call-site-literal scalar args (GGA/MGGA). The `Functional` struct's `Box<dyn FunctionalParams>` field is passed by deref.

- **D-08:** Derived-parameter computation = **`FunctionalParams` trait with one concrete `impl` per functional**. Trait shape (sketch):
  ```rust
  pub trait FunctionalParams: Send + Sync {
      fn raw_ext_params(&self) -> &[f64];           // echoes Functional's storage
      fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError>;
      fn as_any(&self) -> &dyn Any;                 // for dispatch downcast to concrete type
  }
  ```
  Each dispatch arm (37 LDA + 106 GGA + 86 MGGA) downcasts the trait object to its concrete params type via `as_any().downcast_ref::<LdaXParams>()` and extracts scalar args for the kernel launch. Functionals with zero ext_params get a blanket `NoParams: FunctionalParams` struct.

- **D-09:** Wiring rollout = **all 229 compiled functionals get real ext_params plumbing in Phase 5**. No "silent-default" trap — if the user calls `functional.set_ext_param("alpha", 0.7)` on any functional with a defined ext_param, it affects evaluation. Deferred functionals (4 LDA + 6 MGGA from Phase 4) continue to return their existing `UnsupportedFunctional` errors; they get `FunctionalParams` impls only to the extent needed to keep the trait cohesive.

### Functional Struct and Evaluation API

- **D-10:** Module location = **new `src/functional/` top-level module** (NOT `src/func/` — `functional` is more idiomatic and avoids the keyword-abbreviation reading). Internal layout mirrors design doc §9.11: `functional/mod.rs` (Functional struct + public re-exports), `lifecycle.rs` (`new`, `Drop`), `config.rs` (threshold + ext_param setters/getters), `params.rs` (`FunctionalParams` trait + per-functional `impl`s — possibly split per family into `params_lda.rs`, `params_gga.rs`, `params_mgga.rs` if the file becomes unwieldy), `hybrid.rs` (`hybrid_type`, `exx_coefficient`, `cam_coefficients`, `nlc_coefficients`, `auxiliary_functionals` — Rust port of `hybrids.c`).

- **D-11:** Evaluation API = **`Functional::evaluate_{lda,gga,mgga}` methods delegate to the existing free `dispatch_{lda,gga,mgga}` functions**. Functional handles mixed detection (non-empty auxiliaries → route to `evaluate_mixed_*`; empty → direct dispatch). Free `dispatch_*` functions **stay public** (either `pub` or `pub(crate)`, planner decides) — `verify/tests/{lda,gga,mgga}_oracle.rs` continue to call them directly, no test churn. This matches libxc's two-tier C API (low-level `xc_lda()` vs high-level `xc_func_init + evaluate`).

- **D-12:** Mixed GGA/MGGA paths = **Phase 5 fully materializes both**. `evaluate_mixed_gga(input, order, output, auxiliaries, workspace)` mirrors `evaluate_mixed_lda` with the GGA field set (zk, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2, …); `evaluate_mixed_mgga` covers the MGGA field set (adds vlapl, vtau, and all cross-derivatives up to 4th order). `GgaScratch` and `MggaScratch` replace their current `PhantomData` placeholders with real `split_at_mut`-carved slices over the workspace's single contiguous MGGA-superset buffer (matches Phase 3 D-12 — workspace sizing already MGGA-superset). This is load-bearing for HYB-01..04: B3LYP, CAM-B3LYP, wB97X, r2SCAN, all GGA/MGGA hybrids need it.

- **D-13:** Thread-safety = **`FunctionalParams: Send + Sync`** is a mandatory trait bound. `Functional` auto-derives `Send + Sync` from its fields (`&'static FunctionalMeta` is Sync, `Box<[f64]>` is Send+Sync, `Box<dyn FunctionalParams + Send + Sync>` is Send+Sync, `Vec<Functional>` is Send+Sync by recursion). `set_*` methods take `&mut self`, no interior mutability needed.

### Hybrid and Auxiliary Semantics

- **D-14:** HybridType strategy = **both snapshot + Rust port**. `FunctionalMeta.hybrid_type: HybridType` is populated by xtask calling `xc_hyb_type(p)` at snapshot time; a Rust port of `xc_hyb_type` (40-line match on `hybrid_terms.len()` and `hybrid_terms[0].kind`, `hybrid_terms[1].kind`) lives in `src/functional/hybrid.rs`. The verify/ round-trip test compares snapshotted value vs Rust-port value across all 649 IDs — defense in depth against both staleness and port bugs.

- **D-15:** Auxiliary construction = **eager, recursive, at `Functional::new` call-time**. `Functional.auxiliaries: Vec<Functional>`. Empty `Vec` for the ~500 non-hybrid functionals. `Vec<Functional>` of length 1-4 for hybrids (B3LYP = 3 aux, mgga_c_b94 = 2). Matches design doc §10.1 flow literally. Drop is a no-op beyond the automatic `Vec<Functional>` recursive drop.

- **D-16:** Auxiliary ext_params propagation = **xtask-generated static propagation map**, NOT per-functional Rust callback code. During snapshotting, xtask records for each hybrid functional the parent→aux ext_param flow (e.g. for `camy_b3lyp`: parent's `_omega` ext_param → `func_aux[1]`'s `_omega` ext_param). Committed as a static `&'static [(parent_id, parent_idx, aux_slot, aux_param_name)]` table (exact shape planner's discretion). `Functional::new` reads this map and copies values after constructing auxiliaries. This avoids 270 hand-written Rust init callbacks.

- **D-17:** Aux depth = **static bound of 2**. xtask walks the aux graph for all 649 IDs during snapshotting and asserts `max_aux_depth ≤ 2`. If a future libxc release exceeds this, xtask fails loud — forces a conscious decision rather than silent recursion. **No runtime cycle detection** — the aux graph is static-data-driven and already validated at xtask time.

### Claude's Discretion

- Plan decomposition across the 3 plans the roadmap allocates to Phase 5. Suggested split (planner may revise): **Plan 05-01** `libxc-sys` factoring + `xtask generate-metadata` + full `FunctionalMeta` population + verify/ round-trip test; **Plan 05-02** `FunctionalParams` trait + per-functional impls + dispatch signature migration to `&dyn FunctionalParams` + `Functional` struct + lifecycle + ext_params/threshold setters; **Plan 05-03** hybrid queries (HYB-01..04) + eager aux construction + `evaluate_mixed_{gga,mgga}` + `Functional::evaluate_{lda,gga,mgga}` methods.
- Exact trait shape of `FunctionalParams` (getters by name vs index, error variants, `set_ext_params` mutation semantics).
- Whether per-functional `FunctionalParams` impls are hand-written, macro-generated, or xtask-generated from the same metadata snapshot that populates `FunctionalMeta`.
- Internal file layout of `src/functional/` (single `params.rs` vs per-family split).
- Error variant names/messages for unknown ext_param name, index out of range, propagation conflict, auxiliary-init failure.
- Exact `GgaScratch`/`MggaScratch` field layout inside `EvaluationWorkspace`'s contiguous buffer (offsets calculation).
- Whether free `dispatch_*` functions stay `pub` or become `pub(crate)`.
- Whether to fold `xc_hyb_exx_coef` and `xc_hyb_cam_coef` directly into Functional methods or keep them as free functions in `functional::hybrid`.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Roadmap and Requirements

- `.planning/ROADMAP.md` §Phase 5 — goal, requirements FUNC-01..06 + HYB-01..04, success criteria 1-5, 3 plans allocated
- `.planning/REQUIREMENTS.md` — FUNC-01..06 (Functional Instance) and HYB-01..04 (Hybrid Properties) full requirement text
- `.planning/PROJECT.md` §Constraints — thiserror v2 at boundary, f64-only policy, no runtime FFI

### Design Document (primary spec)

- `docs/design/libxc_rs_detailed_design.md` §5.4 — Functional Lifecycle C-to-Rust API mapping
- `docs/design/libxc_rs_detailed_design.md` §5.5 — Threshold Configuration API mapping
- `docs/design/libxc_rs_detailed_design.md` §5.6 — External Parameters API mapping
- `docs/design/libxc_rs_detailed_design.md` §5.7 — Evaluation Functions (35 C → 3 Rust methods)
- `docs/design/libxc_rs_detailed_design.md` §5.8 — Hybrid and Auxiliary API mapping (xc_hyb_type, cam_coef, nlc_coef, aux_func_*)
- `docs/design/libxc_rs_detailed_design.md` §6.8 — Functional struct layout (runtime state)
- `docs/design/libxc_rs_detailed_design.md` §6.9 — EvaluationWorkspace reuse strategy
- `docs/design/libxc_rs_detailed_design.md` §9.11 — `func/` module responsibilities (adapt naming to `functional/`)
- `docs/design/libxc_rs_detailed_design.md` §9.12 — `hybrid/` module responsibilities
- `docs/design/libxc_rs_detailed_design.md` §10.1 — Functional Initialization Flow (step-by-step aux recursion)
- `docs/design/libxc_rs_detailed_design.md` §10.2 — LDA Evaluation Flow (non-mixed)
- `docs/design/libxc_rs_detailed_design.md` §10.3 — Mixed Functional Evaluation Flow
- `docs/design/libxc_rs_detailed_design.md` §15 — Error enum (extend with Phase 5 variants)
- `docs/design/libxc_rs_detailed_design.md` §17 — Oracle verification plan (for metadata round-trip test shape)

### libxc Reference Implementation

- `libxc-master/src/hybrids.c` — `xc_hyb_init`, `xc_hyb_init_{hybrid,sr,cam,camy,camg}`, `xc_hyb_type`, `xc_hyb_exx_coef`, `xc_hyb_cam_coef`. This is the authoritative reference for HYB-01..03 semantics (port the `xc_hyb_type` match into Rust; FFI-snapshot the coefficients).
- `libxc-master/src/hyb_gga_xc_camy_b3lyp.c` — Canonical example of parent→aux ext_param propagation (`_omega` push into aux[1]). Use shape in designing the xtask propagation-map extractor.
- `libxc-master/src/hyb_gga_xc_b2plyp.c` — `xc_hyb_init` with 2-3 terms; example of `XC_HYB_PT2` double-hybrid.
- `libxc-master/src/mgga_c_b94.c` — MGGA hybrid with 2 aux, `_at`/`_gamma`/`_css`/`_cab` propagation — exercise for the MGGA mixed path.
- `libxc-master/src/mix_func.c` — Mixed functional accumulation loop (`output += weight * scratch`); template for `evaluate_mixed_gga` and `evaluate_mixed_mgga`.
- `libxc-master/src/funcs_lda.c` / `funcs_gga.c` / `funcs_mgga.c` — Lists of all `xc_func_info_type` entries per family; xtask iterates these.
- `libxc-master/src/xc.h` — Public C API and struct definitions; authoritative for `xc_func_type` field names that xtask snapshots.
- `libxc-master/src/util.h` — `xc_func_info_type`, `xc_hyb_type` constants (`XC_HYB_NONE`, `XC_HYB_FOCK`, `XC_HYB_ERF_SR`, etc.).

### Current libxc_rs Code (what Phase 5 touches)

- `src/lib.rs` — Add `pub mod functional;`, re-export `Functional`. Currently re-exports `dispatch_lda, dispatch_gga, dispatch_mgga` at crate root — keep or drop per D-11 discretion.
- `src/meta/mod.rs` — `FunctionalMeta` struct definition; confirm all fields we snapshot (ext_params, hybrid_terms, nlc_params, auxiliaries, references, flags) have existing field slots. Add `hybrid_type: HybridType` field (new).
- `src/meta/generated.rs` — Currently 9741 lines of skeleton FunctionalMeta entries with empty arrays. xtask rewrites this with fully-populated entries. Generated file stays marked "DO NOT EDIT".
- `src/meta/{functional_meta,hybrid,auxiliary,nlc,ext_param,library,reference}.rs` — All currently 1-2 line placeholder stubs. Either populate or delete; planner's choice. At minimum, `hybrid.rs` gains the Rust port of `xc_hyb_type`.
- `src/model/mod.rs` — `HybridType`, `HybridTermKind` enums already defined (lines 50-80); confirm they cover the full classification from `xc_hyb_type`.
- `src/eval/dispatch.rs` — Currently `LdaFunctionalParams { alpha }`; signature migrates to `&dyn FunctionalParams`. Every one of the 37 LDA arms changes to downcast + extract. Keep the zero-then-accumulate contract and BUILD-04 no-raw-launch invariant.
- `src/eval/gga_dispatch/mod.rs` + `gga_dispatch/batch*.rs` — 106 GGA arms; same signature migration. Shared `GgaDispatchContext` struct may be extended with a `&dyn FunctionalParams` field.
- `src/eval/mgga_dispatch/mod.rs` + `mgga_dispatch/batch*.rs` — 86 MGGA arms; same migration.
- `src/eval/mix.rs` — `AuxiliaryConfig` struct + `evaluate_mixed_lda` + `add_to_mix`. Add `evaluate_mixed_gga` and `evaluate_mixed_mgga` mirroring the LDA shape.
- `src/eval/workspace.rs` — `LdaScratch` populated; `GgaScratch` and `MggaScratch` currently `PhantomData`. Materialize them with real slice fields.
- `src/registry/mod.rs` — Existing `lookup_by_id`, `lookup_by_name`, `functional_count`. If xtask needs a `lookup_all_ids()` iterator for the round-trip test, add it.
- `src/error/mod.rs` — Extend `LibxcRsError` with `UnknownExtParamName`, `ExtParamIndexOutOfRange`, `AuxiliaryInitFailed`, (possibly) `PropagationConflict`.
- `xtask/` — Existing `generate-registry` subcommand. Add `generate-metadata` subcommand that depends on the new `libxc-sys` workspace crate.
- `verify/build.rs` — Current cmake + bindgen. Factor the cmake-build + bindgen logic into a new `libxc-sys` workspace member; `verify/build.rs` becomes a thin re-export, `xtask/` depends on `libxc-sys` via path dependency.

### Prior Phase Context (decisions that carry forward)

- `.planning/phases/01-foundation-and-registry/01-CONTEXT.md` — Phase 1 D-04 (xtask-generated committed Rust), D-05 (metadata "populated incrementally"), D-06 (module decomposition), D-08 (full LibxcRsError).
- `.planning/phases/02-math-core-and-cubecl-substrate/02-CONTEXT.md` — Phase 2 D-01 (cpu feature only), D-12 (preserve maple2c variable naming — relevant for derived-params helpers).
- `.planning/phases/03-input-output-and-evaluation-framework/03-CONTEXT.md` — Phase 3 D-08 (match-based dispatch), D-10 (scaffold all families), D-11/D-12/D-13 (workspace sizing + non-mixed zero-alloc hot path).
- `.planning/phases/04-bulk-kernel-translation/04-CONTEXT.md` — Phase 4 D-04-R..D-08-R (dispatch shape), D-13-R (libxc defaults hardcoded at call site — this is what Phase 5 undoes), deferred lists (4 LDA + 6 MGGA — stay deferred).
- `.planning/phases/08-extract-kernel-lda-kernel-gga-and-kernel-mgga-into-independe/` — sub-crate layout; every `libxc_kernel_*` re-export path must continue to work after Phase 5 touches dispatch.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `src/eval/mix.rs::evaluate_mixed_lda` (already shipped) — template for the new `evaluate_mixed_gga` and `evaluate_mixed_mgga`. Same structure: zero output, loop over auxiliaries, zero scratch, dispatch aux into scratch, `add_to_mix(output, weight, scratch)`.
- `src/eval/mix.rs::add_to_mix` — generic `dst[i] += coeff * src[i]` loop; already auto-vectorizable. Reused verbatim by the two new evaluate_mixed_* functions.
- `src/eval/workspace.rs::EvaluationWorkspace` — single contiguous `Vec<f64>` sized for MGGA superset (Phase 3 D-12). GGA/MGGA scratch views just need `split_at_mut` carving; no allocation changes.
- `src/kernel/launch.rs` — `cpu_client()`, buffer helpers, `calculate_launch_config` — unchanged, keep using.
- `src/registry/mod.rs::lookup_by_id` / `lookup_by_name` — both work for any populated FunctionalMeta; no registry changes needed for the new fields, just populate.
- `src/meta/mod.rs::FunctionalMeta` — struct layout already includes `ext_params`, `hybrid_terms`, `nlc_params`, `auxiliaries`, `references` — all currently set to `&[]`/`None` in generated.rs. Populating them doesn't require struct changes beyond adding the new `hybrid_type` field.
- `src/model/mod.rs::HybridType` + `HybridTermKind` enums — already match the libxc hybrid classification taxonomy.
- `verify/` FFI infrastructure — libxc built, bindings generated, ready to be re-used by the new `libxc-sys` factored crate.
- `xtask/` — existing `generate-registry` subcommand; add a new sibling for metadata snapshotting.

### Established Patterns

- `#[cube(launch_unchecked)]` kernels with `ABSOLUTE_POS` bounds check — kernels unchanged by Phase 5.
- Zero-then-accumulate output contract (`dispatch_*` zeros caller buffers before launching; kernels write `+=`). Mixed paths double-enforce by zeroing scratch each aux.
- `#[deny(warnings)]` + edition 2024 — keep.
- thiserror v2 at library boundary — extend `LibxcRsError` with Phase 5 variants.
- Committed xtask-generated Rust output (Phase 1 D-04) — extend the same pattern to metadata snapshot.
- Per-family dispatch via typed enum (`LdaFunctional`, `GgaFunctional`, `MggaFunctional`) — unchanged. Functional stores the concrete family enum alongside the trait-object params.

### Integration Points

- `src/lib.rs` — one new `pub mod functional;` + re-exports of `Functional`, `FunctionalParams` trait (if public).
- `src/api/{builder,batch}.rs` — currently 2-line placeholders. They eventually wrap `Functional` in Phase 6; make sure Phase 5's `Functional` API shape is wrap-friendly (constructor returns `Result`, setters take `&mut self` and return `Result`).
- `src/compat/*` — all placeholders today. Phase 6 extern "C" layer wraps `Functional`; again, design with FFI wrapping in mind (no non-`Send` trait objects inside Functional — covered by D-13).
- `verify/tests/{lda,gga,mgga}_oracle.rs` — continue calling free `dispatch_*` for per-functional kernel tests. Add `verify/tests/metadata_oracle.rs` for D-04.
- Every kernel sub-crate facade (`libxc_kernel_lda`, `libxc_kernel_gga`, `libxc_kernel_mgga` + batches) — unchanged; dispatch layer is the only code Phase 5 touches in `src/eval/`.

### What Phase 5 Creates

- `src/functional/mod.rs` — Functional struct, public API
- `src/functional/lifecycle.rs` — `new`, `Drop` (if non-trivial)
- `src/functional/config.rs` — threshold setters, ext_param set/get by name and index
- `src/functional/params.rs` — `FunctionalParams` trait + blanket `NoParams` impl (may split per-family)
- `src/functional/hybrid.rs` — Rust port of `xc_hyb_type`, `xc_hyb_exx_coef`, `xc_hyb_cam_coef`, `xc_nlc_coef`; aux iteration helpers
- `libxc-sys/` (new workspace member) — cmake + bindgen of libxc-master
- `xtask/src/generate_metadata.rs` — new subcommand
- `verify/tests/metadata_oracle.rs` — round-trip test
- Extended `src/meta/generated.rs` (rewritten in place with fully-populated entries)
- `src/meta/generated_propagation.rs` (new) OR an extension of `generated.rs` — xtask-emitted static propagation map for aux ext_params
- Possibly `src/meta/generated_hybrid.rs` (new) — if hybrid-specific metadata wants its own file

</code_context>

<specifics>
## Specific Ideas

- When emitting the propagation map (D-16), include the parent ext_param's **name** in the record (not just index) so the xtask snapshot self-documents (e.g. a diff is readable). Indices are stable within a single libxc version but names are stable across versions — useful for drift diagnosis.
- The verify/ round-trip test (D-04) should compare **all** fields, including nested arrays (references array, ext_params array). Use a struct-level `PartialEq` on `FunctionalMeta` if it doesn't exist, or a custom field-by-field assertion helper.
- `FunctionalParams` trait's `as_any(&self) -> &dyn Any` method is what lets each dispatch arm downcast to the concrete type. This is a deliberate escape hatch — without `Any`, dispatch arms cannot access functional-specific derived fields.
- Design-doc §6.8 uses `Box<dyn FunctionalParams>` (not `Box<dyn FunctionalParams + Send + Sync>`) — our D-13 tightens this to require Send+Sync everywhere for uniform thread-safety guarantees.
- For functionals with zero ext_params, reuse a single static `NoParams` trait object — no per-instance allocation. Possible via `Box::leak(Box::new(NoParams))` once at startup, or a `&'static dyn FunctionalParams` variant if dispatch signature allows.
- The `xc_hyb_type` Rust port (D-14) is a direct translation of `libxc-master/src/hybrids.c:82-118` — 40-line match, no cleverness. The test is: `assert_eq!(rust_xc_hyb_type(&meta.hybrid_terms), meta.hybrid_type)` for all 649 IDs.
- For GgaScratch/MggaScratch materialization (D-12), use the same `split_at_mut` chain idiom as the existing `LdaScratch` in `workspace.rs` — offsets are already MGGA-superset so no re-sizing of the underlying buffer.
- Propagation map shape (suggested): `&'static [PropagationRule]` where `PropagationRule { parent_id: FunctionalId, parent_param_name: &'static str, aux_slot: u8, aux_param_name: &'static str, transform: PropagationTransform }`. `PropagationTransform::Copy` is the common case; xtask detects other transforms (e.g. multiplication) from the libxc _init source if any exist.
- Aux depth validator (D-17): a simple xtask pass that walks `meta.auxiliaries` starting from each hybrid's ID and asserts depth ≤ 2. Failure is loud panic at `cargo xtask generate-metadata` time, forcing human review.

</specifics>

<deferred>
## Deferred Ideas

- **Enabling the 4 LDA and 6 MGGA deferred functionals** — unchanged from Phase 4. Phase 5 treats them as permanently deferred from ext_params wiring too (their `FunctionalParams` impls may be bare-minimum or `NoParams`). Full enablement needs root-finders in `kernel-math` (MGGA) or kernel-splitting (LDA) — tracked, not scheduled.
- **`FunctionalBuilder` chainable API** — Phase 6 (API-01).
- **`BatchEvaluator` with reusable workspace** — Phase 6 (API-02).
- **Ergonomic `evaluate()` auto-dispatch by family** — Phase 6 (API-03).
- **extern "C" layer** — Phase 6 (COMPAT-01..03).
- **GPU backends + f64 capability check** — Phase 7 (GPU-01..07).
- **Performance benchmarks** — Phase 7 (PERF-01..05).
- **References/DOI surfacing at runtime** — population is in scope (D-02), but the user-facing API to query references (a `Functional::references() -> &[Reference]` getter) can be minimal in Phase 5 — polish can land in Phase 10.
- **Propagation transforms beyond Copy** — if xtask discovers that some `_init` functions apply transforms (scaling, subtraction) rather than straight copy during parent→aux ext_params flow, the propagation map shape supports a `PropagationTransform` enum. The planner may scope Phase 5 to Copy-only + an escape hatch error for non-Copy cases, deferring full transform support as follow-up if libxc has only a handful of exceptions.

### Un-discussed areas the planner owns

- Plan decomposition across 3 roadmap plans (suggested split in Claude's Discretion above).
- ext_params-by-name vs by-index API shape — both required per FUNC-02, but the trait method signatures are planner's call.
- New `LibxcRsError` variant names and messages.
- Whether `FunctionalParams` impls are hand-written, macro-generated, or xtask-emitted alongside the metadata snapshot.

</deferred>

---

*Phase: 05-functional-lifecycle-and-hybrid-properties*
*Context gathered: 2026-04-24*
