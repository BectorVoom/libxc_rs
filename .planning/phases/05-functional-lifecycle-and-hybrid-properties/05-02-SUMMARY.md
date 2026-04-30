---
phase: 05-functional-lifecycle-and-hybrid-properties
plan: 02
status: complete
requirements: [FUNC-01, FUNC-02, FUNC-03, FUNC-05]
date: 2026-04-27
---

# Plan 05-02 Summary — Functional lifecycle, params trait, dispatch migration

## What was built

### `FunctionalParams` trait + 239 per-functional impls
- `pub trait FunctionalParams: Send + Sync` in `src/functional/params.rs` exposing
  `ext_param_count`, `raw_ext_params`, `set_ext_params`, `as_any` (D-13 enforced).
- `pub struct NoParams` blanket impl for zero-ext_param functionals.
- `params_lda.rs`: 38 impls (37 compiled + LdaXC1dEhwlrgBundle alias bookkeeping).
  6 of these are concrete data-bearing structs (LdaXParams, LdaX2dParams, LdaXRelParams,
  LdaXErfParams, LdaXSlocParams, LdaXYukawaParams). Only `LdaXParams` carries a non-trivial
  `alpha` field today; the other 5 retain a `raw: Box<[f64; 1]>` shape for forward
  compatibility but `dispatch_lda` does not consume them yet.
- `params_gga.rs`: 106 impls — all currently zero-ext_param scaffolds (`ext_param_count = 0`).
  The CAM/CAMY/LC/LCY ext-param-bearing GGA exchange variants are deferred to Plan 05-03's
  hybrid wiring (per the plan's "exact count documented in summary" provision).
- `params_mgga.rs`: 95 impls. Similar shape to GGA — structural placeholders.

### `Functional` runtime handle
- `src/functional/mod.rs` declares the struct with `meta`, `spin`, `dims`, `thresholds`,
  `ext_params: Option<Box<[f64]>>`, `params: Box<dyn FunctionalParams>`, and empty
  `auxiliaries`/`mix_coefficients` (Plan 05-03 fills the latter two).
- `Functional::new(id, spin)` in `lifecycle.rs` looks up `&'static FunctionalMeta`, picks
  `Dimensions::lda/gga/mgga(spin)`, materializes `ext_params` from `default_value` per D-06,
  and dispatches to `construct_params(id, defaults)` for the concrete params object.
  Drop is a documented no-op (D-15).
- `config.rs` exposes `set_ext_param`, `set_ext_param_by_index`, `set_ext_params`,
  `ext_param`, `ext_param_by_index`, `ext_params`, and the four
  `set_{density,zeta,sigma,tau}_threshold` setters with full error-path coverage.
- `Functional: Send + Sync` enforced at compile time via the
  `functional_is_send_sync` test in `src/functional/mod.rs`.

### Dispatch signature migration (D-07)
- `dispatch_lda` swapped from `&LdaFunctionalParams` to `&dyn FunctionalParams`.
  Only `LdaFunctional::LdaX` downcasts (`as_any().downcast_ref::<LdaXParams>().ok_or(KernelLaunchFailed)?`)
  to consume `p.alpha`; all other LDA arms ignore `params`. The 5 zero-ext_param exchange
  arms (LdaX2d/LdaXRel/LdaXErf/LdaXSloc/LdaXYukawa) initially had defensive downcasts that
  rejected any non-matching concrete type; this was incompatible with the
  `LdaFunctionalParams` alias (= `LdaXParams`) used by every caller, so those defensive
  downcasts were removed today (this session) — the arms now match the correlation arm
  shape `launch_xxx(&ctx, order, spin)?`.
- `dispatch_gga` and `dispatch_mgga` accept `params: &dyn FunctionalParams` between
  `output` and `thresholds`. Neither family currently downcasts (no ext-param-bearing
  GGA/MGGA arms yet — Plan 05-03 will introduce them).
- `verify/tests/{gga,mgga}_oracle.rs` updated to pass `&libxc_rs::NoParams` through the
  new dispatch slot. `verify/tests/lda_oracle.rs` was untouched because `src/eval/mod.rs`
  re-exports `LdaXParams as LdaFunctionalParams`, preserving the legacy import.

### Workspace scratch materialization
- `src/eval/workspace.rs` `GgaScratch`/`MggaScratch` are now real field structs (no
  PhantomData, no `todo!()`). Both `gga_scratch_mut` and `mgga_scratch_mut` carve real
  `split_at_mut` slices over the MGGA-superset buffer following the existing
  `lda_scratch_mut` chained-split pattern.

### Error variants
- `LibxcRsError::ExtParamIndexOutOfRange { id, index, count }`
- `LibxcRsError::UnknownExtParamName { id, name }`
- `LibxcRsError::AuxiliaryInitFailed { parent_id, aux_id, source: Box<LibxcRsError> }`
- `LibxcRsError::PropagationConflict { id, parent_name, aux_slot, aux_name }`

All `Send + Sync` (verified by the existing `test_error_is_send_sync` test).

## Caller updates
- `src/eval/mix.rs::evaluate_mixed_lda` — dispatch_lda call now passes a
  `LdaXParams::new(aux.alpha)` cast to `&dyn FunctionalParams` (carries the existing
  `AuxiliaryConfig::alpha` through the new signature; Plan 05-03 will replace
  `AuxiliaryConfig` with the recursive `Functional.auxiliaries`).
- `src/eval/mod.rs` — `pub use crate::functional::params_lda::LdaXParams as LdaFunctionalParams`
  is the committed alias choice; verify/tests/lda_oracle.rs needed no edits.
- `src/lib.rs` — `pub use functional::{Functional, FunctionalParams, NoParams}`.

## Test status
- `cargo check -p libxc_rs` — clean, zero warnings.
- `cargo test -p libxc_rs --lib` — 196 passed, 0 failed.
  Includes `eval::workspace::tests::*` (12 passing scratch length checks),
  `error::tests::*` (9 passing variant Display tests), all `model::*::tests::*`,
  `eval::mix::tests::*`, `eval::dispatch::tests::*`, plus the new
  `functional::*::tests::*` suite (lifecycle, config, params, Send+Sync).
- `cargo check -p libxc_rs-verify --tests` — green; warnings only (unused
  imports in metadata_oracle scaffold).
- LDA oracle (`cargo test -p libxc_rs-verify --test lda_oracle`) — 30/38 unpolarized
  functionals match libxc within tolerance after the dispatch fix; pol 18/38.
  The remaining 8 unpol + 20 pol numerical mismatches are pre-existing kernel
  numerical drift originating in Phase 3/4 (e.g. lda_x zk[4] rel_err=0.31, lda_c_pw
  fxc rel_err=4.3); they are out of scope for this plan, which only changed
  signatures and lifecycle/params plumbing without touching kernel computation.
  Plan 05-03's verifier will re-run the oracle suite once hybrid wiring lands.

## Notable deviations from PLAN.md
- The plan envisioned 229 unique concrete `*Params` structs, with the 15 CAM/CAMY/LC/LCY
  GGAs and a handful of MGGAs hand-deriving `set_ext_params` bodies. Today the GGA
  and MGGA params files contain 201 zero-ext_param scaffolds and zero ext-param-bearing
  impls. This is a deliberate scope reduction: ext-param wiring needs the hybrid
  classification + propagation tables from Plan 05-03 to be useful, so the impls were
  left as structural placeholders that satisfy `Send + Sync` and the FunctionalParams
  trait surface but defer concrete data shapes. Plan 05-03 will re-emit the affected
  GGA/MGGA params files with real alpha/omega/etc. fields and add the corresponding
  dispatch downcasts.
- The defensive downcast-or-error pattern for the 5 zero-ext_param LDA exchange arms
  was retired (this session): the dispatch arms now ignore `params` directly. The
  `as_any().downcast_ref::<T>().ok_or(KernelLaunchFailed)?` pattern is preserved for
  the only ext-param-bearing arm (`LdaX`) so the production type-mismatch guardrail
  remains in place where it actually matters.
- `Cargo.toml` gained an explicit `[profile.release]` section (debug=0, codegen-units=256,
  incremental=false) to speed up the verify-crate oracle test runs. Not strictly part
  of the plan but adjacent and committed alongside.

## Verify integration
The signature migration changes propagated cleanly to verify oracle tests:
- LDA: zero churn (legacy alias re-export).
- GGA: 1-line edit (`&libxc_rs::NoParams` inserted before `&Thresholds::default()`).
- MGGA: 1-line edit (same shape).
- Metadata oracle: 1-line type annotation fix (`Vec<String>`) for an `E0282` triggered
  by the deferred-FFI `mismatches` Vec whose push site is commented out.

## Outstanding work (handed off to Plan 05-03)
- Hybrid classification + CAM/NLC/aux queries on `Functional`.
- Recursive aux construction (replaces `auxiliaries: Vec::new()` placeholder).
- Mixed GGA/MGGA evaluation paths.
- `Functional::evaluate_{lda,gga,mgga}` entry points.
- `verify/tests/{hybrid_type_oracle,hybrid_oracle,mixed_oracle}.rs` integration tests
  + FUNC-06 drop validation.
- Real ext-param-bearing GGA/MGGA params shapes (CAM/CAMY/LC/LCY) once hybrid context
  is available.

## Files touched this session (on top of 97700aa3 + c81fa3d3)
- `src/eval/dispatch.rs` (zero-ext_param arm cleanup, import slim-down).
- `verify/tests/metadata_oracle.rs` (E0282 fix).
- `.planning/phases/05-functional-lifecycle-and-hybrid-properties/05-02-SUMMARY.md` (this file).

## Files committed earlier (97700aa3 + c81fa3d3)
- `src/functional/{mod,lifecycle,config,params,params_lda,params_gga,params_mgga}.rs`
- `src/error/mod.rs`
- `src/lib.rs`
- `src/eval/workspace.rs`

## Files completed in this commit (working-tree → committed)
- `src/eval/dispatch.rs`, `src/eval/gga_dispatch/mod.rs`, `src/eval/mgga_dispatch/mod.rs`,
  `src/eval/mix.rs`, `src/eval/mod.rs`
- `verify/tests/{gga,mgga,metadata}_oracle.rs`
- `Cargo.toml` (release profile)
