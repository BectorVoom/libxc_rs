# Phase 5: Functional Lifecycle and Hybrid Properties - Pattern Map

**Mapped:** 2026-04-24
**Files analyzed:** 20 new/modified files
**Analogs found:** 18 / 20 (2 files = brand new — libxc-sys build crate + xtask subcommand — have partial analogs only)

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|-------------------|------|-----------|----------------|---------------|
| `src/functional/mod.rs` (NEW) | public API / runtime handle | request-response + CRUD (state getters) | `src/eval/mod.rs` (re-export pattern) + `src/eval/dispatch.rs` (public fn doc style) | role-match |
| `src/functional/lifecycle.rs` (NEW) | constructor + Drop | transform (static meta → owned state) | `src/eval/workspace.rs::EvaluationWorkspace::new` + `src/model/mod.rs::FunctionalId::from_raw` | role-match |
| `src/functional/config.rs` (NEW) | state setters (&mut self) | request-response | `src/model/mod.rs::Thresholds::default` + `src/model/mod.rs::FunctionalId::name` | role-match |
| `src/functional/params.rs` (NEW) | trait + blanket impl | transform / CRUD on ext_params | `src/eval/dispatch.rs::LdaFunctionalParams` (existing params struct) | role-match |
| `src/functional/params_lda.rs` / `params_gga.rs` / `params_mgga.rs` (NEW, optional split) | per-functional impls (229 total) | transform | `src/eval/dispatch.rs` match arms (37 arm shape); `src/model/lda_functional.rs` pattern for enum-per-id | role-match |
| `src/functional/hybrid.rs` (NEW) | classifier + introspection queries | transform (data-only) | `libxc-master/src/hybrids.c:82-157` (primary spec) + `src/model/mod.rs::HybridType` enum | exact-spec |
| `libxc-sys/build.rs` (NEW) | build-time FFI generator | file-I/O + transform | `verify/build.rs` (factor verbatim) | exact |
| `libxc-sys/src/lib.rs` (NEW) | thin FFI include | transform | `verify/src/oracle_ffi.rs` (3-line include! pattern) | exact |
| `libxc-sys/Cargo.toml` (NEW) | package manifest | config | `verify/Cargo.toml` [build-dependencies] block | exact |
| `xtask/src/generate_metadata.rs` (NEW) | code-generator binary module | file-I/O + transform (FFI → static Rust) | `xtask/src/main.rs::generate_meta_file` + `generate_by_id_file` | role-match |
| `xtask/src/main.rs` (MODIFY) | CLI dispatch | request-response | self (add subcommand arm) | exact |
| `xtask/Cargo.toml` (MODIFY) | package manifest | config | self (add libxc-sys path dep) | exact |
| `verify/build.rs` (MODIFY) | thin re-export | config | self (strip to empty / delegate to libxc-sys) | exact |
| `verify/Cargo.toml` (MODIFY) | package manifest | config | self (swap build-deps for libxc-sys path dep) | exact |
| `verify/tests/metadata_oracle.rs` (NEW) | D-04 round-trip test | request-response (FFI call vs static compare) | `verify/tests/lda_oracle.rs` (per-id loop + FFI call pattern) | role-match |
| `verify/tests/hybrid_type_oracle.rs` (NEW) | HYB-01 three-way compare | request-response | `verify/tests/lda_oracle.rs` | role-match |
| `verify/tests/mixed_oracle.rs` (NEW) | FUNC-04 integration | request-response | `verify/tests/lda_oracle.rs` + `src/eval/mix.rs::tests` | role-match |
| `verify/tests/hybrid_oracle.rs` (NEW) | HYB-02 / HYB-03 coefficient queries | request-response | `verify/tests/lda_oracle.rs` | role-match |
| `src/meta/mod.rs` (MODIFY) | struct extension | — | self (add `hybrid_type` field beside existing `hybrid_terms`) | exact |
| `src/meta/generated.rs` (REWRITE) | xtask-emitted static data | — | self (keep shape; fill fields currently `&[]` / `None`) | exact |
| `src/meta/generated_hybrid.rs` (NEW) | xtask-emitted static data | — | `src/registry/by_id.rs` pattern (xtask-generated table) | role-match |
| `src/meta/generated_propagation.rs` (NEW) | xtask-emitted static data | — | `src/registry/by_id.rs` + `src/registry/removed.rs` pattern | role-match |
| `src/eval/dispatch.rs` (MODIFY) | LDA dispatch signature migration | request-response | self (migrate `&LdaFunctionalParams` → `&dyn FunctionalParams`) | exact |
| `src/eval/gga_dispatch/mod.rs` (MODIFY) | GGA dispatch signature migration | request-response | self + `src/eval/dispatch.rs` params arg | exact |
| `src/eval/mgga_dispatch/mod.rs` (MODIFY) | MGGA dispatch signature migration | request-response | self + `src/eval/dispatch.rs` params arg | exact |
| `src/eval/mix.rs` (MODIFY, add 2 fns) | mixed evaluators | CRUD (iterate auxiliaries, accumulate) | `src/eval/mix.rs::evaluate_mixed_lda` (lines 55-184) | exact |
| `src/eval/workspace.rs` (MODIFY) | materialize GgaScratch / MggaScratch | transform (split_at_mut chain) | `src/eval/workspace.rs::lda_scratch_mut` (lines 197-239) | exact |
| `src/error/mod.rs` (MODIFY) | error enum extension | — | self (append 4 new thiserror variants) | exact |
| `src/lib.rs` (MODIFY) | public re-export | — | self (add `pub mod functional;` + re-export `Functional`) | exact |
| `src/registry/mod.rs` (MAYBE MODIFY) | `lookup_all_ids` already exists as `all_functional_ids()` (line 72-77) | — | self | exact (likely no change needed) |

---

## Pattern Assignments

### `src/functional/mod.rs` (controller/public-API, request-response)

**Analog:** `src/eval/mod.rs` (re-export pattern) + `src/eval/dispatch.rs::dispatch_lda` (public-fn doc style)

**Re-export pattern** (from `src/eval/mod.rs:1-10`):
```rust
pub mod dispatch;
pub mod gga_dispatch;
pub mod mgga_dispatch;
pub mod mix;
pub mod workspace;
pub use dispatch::{dispatch_lda, LdaFunctionalParams};
pub use gga_dispatch::dispatch_gga;
pub use mgga_dispatch::dispatch_mgga;
pub use mix::{add_to_mix, evaluate_mixed_lda, AuxiliaryConfig};
pub use workspace::EvaluationWorkspace;
```

New `src/functional/mod.rs` mirrors this exactly: declare child modules (`lifecycle`, `config`, `params`, `params_{lda,gga,mgga}`, `hybrid`), then `pub use` the small public surface (`Functional`, `FunctionalParams`, `NoParams`, `CamCoefficients`, etc.).

**Public-fn rustdoc style** (from `src/eval/dispatch.rs:66-79`):
```rust
/// # Arguments
/// * `functional` - Selects which LDA kernel module to launch
/// * `input` - Validated LDA input bundle
/// * `order` - Maximum derivative order to compute
/// * `output` - Output bundle with optional buffers for each derivative level
/// * `params` - Per-functional scalar parameters (currently `alpha` for
///   exchange functionals; all other functionals use libxc defaults)
/// * `thresholds` - Numerical thresholds for evaluation stability
///
/// # Errors
/// * `UnsupportedDerivativeOrder` if `order == Exc` for a `_vxc`-only
///   functional like `LdaXcTih`.
/// * `KernelLaunchFailed` on CubeCL launch failure.
```

Every `Functional` method copies this structure: `# Arguments` bullet list, `# Errors` bullet list with concrete `LibxcRsError` variants.

---

### `src/functional/lifecycle.rs` (component, transform)

**Analog:** `src/eval/workspace.rs::EvaluationWorkspace::new` (lines 75-84) + `src/model/mod.rs::FunctionalId::from_raw` (lines 92-94)

**Static-to-owned construction pattern** (from `src/eval/workspace.rs:75-84`):
```rust
pub fn new(np: usize, spin: Spin) -> Self {
    let dims = Dimensions::mgga(spin);
    let total = dims.total_output_components() * np;
    Self {
        scratch: vec![0.0; total],
        np,
        spin,
        dims,
    }
}
```

`Functional::new` follows the same shape: compute derived fields (dims, ext_params default-box, aux recursion) and construct via struct literal.

**Registry-lookup-then-validate pattern** (from `src/model/mod.rs:92-94`):
```rust
pub fn from_raw(id: u16) -> Result<Self, crate::LibxcRsError> {
    crate::registry::lookup_by_id(id).map(|meta| meta.id)
}
```

`Functional::new(id, spin)` starts with `let meta = registry::lookup_by_id(id.raw())?;` — same lookup idiom. **Return `Result<Self, LibxcRsError>`** — never panic on bad id; propagate `UnknownFunctionalId` / `RemovedFunctionalId` as produced by `lookup_by_id`.

**Drop pattern** (none — use default-generated):
```rust
// No-op Drop; all fields are owned Rust types (Box, Vec, &'static) that auto-drop.
// Do NOT implement Drop unless there is a specific non-trivial resource.
// CONTEXT.md D-15: "Drop is a no-op beyond the automatic Vec<Functional> recursive drop."
// If a Drop impl is added for symmetry/docs, it should be empty:
// impl Drop for Functional { fn drop(&mut self) { /* no-op; documented */ } }
```

---

### `src/functional/config.rs` (service, request-response)

**Analog:** `src/model/mod.rs::FunctionalId::name` (lines 107-113) + `src/model/mod.rs::Thresholds::default` (lines 162-170)

**Getter pattern over static data** (from `src/model/mod.rs:107-113`):
```rust
pub fn name(self) -> &'static str {
    crate::registry::lookup_by_id(self.0)
        .map(|m| m.name)
        .unwrap_or("UNKNOWN")
}
```

`Functional::ext_params()` returns `Option<&[f64]>` from the owned `Option<Box<[f64]>>` field: `self.ext_params.as_deref()`.

**Setter error-path pattern** (from `src/error/mod.rs:53-63`):
```rust
#[error("external parameter '{name}' not found for functional {id}")]
ExtParamNotFound {
    id: FunctionalId,
    name: String,
},

#[error("external parameter count mismatch for {id}: expected {expected}, got {actual}")]
ExtParamCountMismatch {
    id: FunctionalId,
    expected: usize,
    actual: usize,
},
```

`set_ext_param(name, value)` returns `Err(ExtParamNotFound{id, name})` when the name is absent from `meta.ext_params`. `set_ext_params(&[f64])` returns `Err(ExtParamCountMismatch{..})` on length mismatch. Both are already in `LibxcRsError`; Phase 5 extends with `UnknownExtParamName`, `ExtParamIndexOutOfRange`, etc. (see shared patterns below).

**Defaults-from-static pattern** (from `src/model/mod.rs:162-170`):
```rust
impl Default for Thresholds {
    fn default() -> Self {
        Self {
            density: 1e-15,
            zeta: 1e-10,
            sigma: 1e-24,
            tau: 1e-20,
        }
    }
}
```

`Functional`'s per-instance `thresholds` field starts at `Thresholds::default()` (same values); `set_density_threshold(&mut self, v: f64)` just writes `self.thresholds.density = v`. No validation of value range — matches Phase 3 philosophy (pass through; kernels apply thresholds).

---

### `src/functional/params.rs` (service, transform)

**Analog:** `src/eval/dispatch.rs::LdaFunctionalParams` (lines 30-53)

**Existing concrete params struct pattern** (from `src/eval/dispatch.rs:42-53`):
```rust
#[derive(Debug, Clone, Copy)]
pub struct LdaFunctionalParams {
    /// Slater exchange scaling for `lda_x`. Ignored by every other
    /// functional (their parameters come from libxc defaults).
    pub alpha: f64,
}

impl Default for LdaFunctionalParams {
    fn default() -> Self {
        Self { alpha: 1.0 }
    }
}
```

After Phase 5, this struct either (a) becomes one concrete `FunctionalParams` impl (renamed `LdaXParams` and gaining `as_any`), or (b) is deleted in favor of xtask-generated per-functional structs. The **import surface** from `src/eval/mod.rs:6` (`pub use dispatch::{dispatch_lda, LdaFunctionalParams};`) is a public re-export that verify tests import (see `verify/tests/lda_oracle.rs:24`: `use libxc_rs::eval::{dispatch_lda, LdaFunctionalParams};`). **Preserve or rename-with-re-export** — do not silently remove without verify-test update.

**Target trait shape** (from RESEARCH.md Pattern 1, confirmed against CONTEXT.md D-08):
```rust
pub trait FunctionalParams: Send + Sync {
    fn ext_param_count(&self) -> usize;
    fn raw_ext_params(&self) -> &[f64];
    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError>;
    fn as_any(&self) -> &dyn Any;
}

pub struct NoParams;
impl FunctionalParams for NoParams {
    fn ext_param_count(&self) -> usize { 0 }
    fn raw_ext_params(&self) -> &[f64] { &[] }
    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(0), expected: 0, actual: vals.len(),
            });
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any { self }
}
```

**Downcast in dispatch pattern** (new, from RESEARCH.md Code Example 1):
```rust
LdaFunctional::LdaX => {
    let p = params.as_any().downcast_ref::<LdaXParams>()
        .ok_or(LibxcRsError::KernelLaunchFailed {
            reason: "FunctionalParams type mismatch: LdaX expects LdaXParams".into(),
        })?;
    launch_lda_x(&ctx, order, spin, p.alpha)?
}
```

**Critical: never `.unwrap()` or `.expect()` on `downcast_ref`** — return `LibxcRsError::KernelLaunchFailed` (already exists, line 82-83 of `src/error/mod.rs`). Dispatch arms assume correct pairing; a mismatch means a programmer bug but we still recover without panic.

---

### `src/functional/hybrid.rs` (utility, transform)

**Analog:** `libxc-master/src/hybrids.c:82-157` (authoritative C logic) + `src/model/mod.rs::HybridType` (target enum lines 50-69)

**xc_hyb_type port** (from `libxc-master/src/hybrids.c:82-118` — port literally):
```c
int xc_hyb_type(const xc_func_type *p) {
  if(p->hyb_number_terms == 0)
    return XC_HYB_NONE;

  if(p->hyb_number_terms == 1){
    if(p->hyb_type[0] == XC_HYB_NONE)      return XC_HYB_SEMILOCAL;
    if(p->hyb_type[0] == XC_HYB_FOCK)      return XC_HYB_HYBRID;
    if(p->hyb_type[0] == XC_HYB_ERF_SR)    return XC_HYB_CAM;
    if(p->hyb_type[0] == XC_HYB_YUKAWA_SR) return XC_HYB_CAMY;
    if(p->hyb_type[0] == XC_HYB_GAUSSIAN_SR) return XC_HYB_CAMG;
  }
  if(p->hyb_number_terms == 2) {
    if(hyb_type[0] == ERF_SR      && hyb_type[1] == FOCK) return XC_HYB_CAM;
    if(hyb_type[0] == YUKAWA_SR   && hyb_type[1] == FOCK) return XC_HYB_CAMY;
    if(hyb_type[0] == GAUSSIAN_SR && hyb_type[1] == FOCK) return XC_HYB_CAMG;
    if(hyb_type[0] == PT2         && hyb_type[1] == FOCK) return XC_HYB_DOUBLE_HYBRID;
  }
  return XC_HYB_MIXTURE;
}
```

Rust port `classify_hybrid(terms: &[HybridTerm]) -> HybridType` — direct translation, no cleverness. **Beware Pitfall 6:** single-term `XC_HYB_NONE` case — resolved by xtask suppressing that term at snapshot time so `terms.is_empty()` → `Semilocal`.

**xc_hyb_cam_coef port** (from `libxc-master/src/hybrids.c:132-157`):
```c
if(p->hyb_number_terms == 1) {
    if(p->hyb_type[0] == XC_HYB_FOCK) {
      *omega = 0.0;  *beta = 0.0;  *alpha = p->hyb_coeff[0];
    } else {
      *omega = p->hyb_omega[0];  *beta = p->hyb_coeff[0];  *alpha = 0.0;
    }
  } else if(p->hyb_number_terms == 2) {
    *omega = p->hyb_omega[0];  *beta = p->hyb_coeff[0];  *alpha = p->hyb_coeff[1];
  }
```

Rust: `pub fn cam_coefficients(terms: &[HybridTerm]) -> Option<CamCoefficients>` returning `Some((omega, alpha, beta))` for CAM/CAMY/CAMG/Hybrid, `None` otherwise. Mirror the C branches exactly.

**xc_hyb_exx_coef port** (from `libxc-master/src/hybrids.c:123-130`):
```c
double xc_hyb_exx_coef(const xc_func_type *p) {
  assert(xc_hyb_type(p) == XC_HYB_HYBRID);
  return p->hyb_coeff[0];
}
```

Rust: `pub fn exx_coefficient(terms: &[HybridTerm]) -> Option<f64>` — returns `Some(terms[0].coefficient)` iff `classify_hybrid(terms) == HybridType::Hybrid`, else `None`. **Never assert** — return `None` so callers can write `functional.exx_coefficient().ok_or(...)`.

---

### `src/eval/mix.rs::evaluate_mixed_gga` / `evaluate_mixed_mgga` (NEW, CRUD over aux list)

**Analog:** `src/eval/mix.rs::evaluate_mixed_lda` (lines 55-184) — this is the exact template.

**Per-aux loop structure** (from `src/eval/mix.rs:98-181`):
```rust
for aux in auxiliaries {
    workspace.zero_scratch();           // (1) zero scratch
    {
        let scratch = workspace.lda_scratch_mut();
        let mut scratch_output = LdaOutput {
            zk: Some(scratch.zk),
            vrho: if order >= DerivativeOrder::Vxc { Some(scratch.vrho) } else { None },
            v2rho2: if order >= DerivativeOrder::Fxc { Some(scratch.v2rho2) } else { None },
            // ...
        };
        dispatch_lda(LdaFunctional::LdaX, input, order, &mut scratch_output,
                     &LdaFunctionalParams { alpha: aux.alpha }, &aux.thresholds)?;
    }  // scratch_output dropped → mutable borrow on workspace released
    let scratch = workspace.lda_scratch_mut();    // (3) re-borrow for accumulation

    if let Some(ref mut dst) = output.zk {
        add_to_mix(dst, aux.weight, &scratch.zk[..zk_len]);
    }
    if order >= DerivativeOrder::Vxc
        && let Some(ref mut dst) = output.vrho
    {
        add_to_mix(dst, aux.weight, &scratch.vrho[..vrho_len]);
    }
    // ...
}
```

**GGA-specific fan-out** needs per-aux family gating per RESEARCH.md Pattern 3 / Pitfall 5 — mirrors `libxc-master/src/mix_func.c:150-308`:
```rust
for aux in &functional.auxiliaries {
    workspace.zero_scratch();
    // (a) Build scratch_output matching aux.family's output surface (LDA/GGA/MGGA)
    // (b) Dispatch by aux.meta.family:
    match aux.meta.family {
        Family::Lda  => dispatch_lda(...)?,      // writes only rho-derivatives
        Family::Gga  => dispatch_gga(...)?,      // writes rho + sigma derivatives
        Family::Mgga => /* unreachable for GGA parent */,
    }
    // (c) Accumulate with family-gated fan-out:
    //     Unconditional: zk, vrho, v2rho2, v3rho3, v4rho4
    //     is_gga(aux.meta.family) OR is_mgga(aux.meta.family): vsigma, v2rhosigma, v2sigma2, ...
    //     is_mgga(aux.meta.family) + NEEDS_LAPLACIAN: vlapl, v2rholapl, ...
    //     is_mgga(aux.meta.family) + NEEDS_TAU: vtau, v2rhotau, ...
}
```

**Mixed-MGGA** adds the lapl/tau fan-out with NEEDS_LAPLACIAN / NEEDS_TAU flag checks — direct translation of `mix_func.c:184-305` (lines read above).

**NEW signature** (align with migration D-07):
```rust
pub fn evaluate_mixed_gga(
    functional: &Functional,
    input: &GgaInput,
    order: DerivativeOrder,
    output: &mut GgaOutput,
    workspace: &mut EvaluationWorkspace,
) -> Result<(), LibxcRsError>
```
Functional-bearing arg replaces `auxiliaries: &[AuxiliaryConfig]`; `functional.auxiliaries: &[Functional]` carries per-aux metadata and per-aux `FunctionalParams`.

---

### `src/eval/workspace.rs::gga_scratch_mut` / `mgga_scratch_mut` (MODIFY — materialize)

**Analog:** `src/eval/workspace.rs::lda_scratch_mut` (lines 197-239)

**split_at_mut chain idiom** (from lines 197-239 — the key shape):
```rust
pub fn lda_scratch_mut(&mut self) -> LdaScratch<'_> {
    let offsets = self.lda_field_offsets();

    let (zk_and_rest, after_zk) = self.scratch.split_at_mut(offsets.zk_len);
    let zk = &mut zk_and_rest[..offsets.zk_len];

    let vrho_local_off = offsets.vrho_off - offsets.zk_len;
    let (_, vrho_start) = after_zk.split_at_mut(vrho_local_off);
    let (vrho, after_vrho) = vrho_start.split_at_mut(offsets.vrho_len);

    let v2rho2_local_off = offsets.v2rho2_off - offsets.vrho_off - offsets.vrho_len;
    let (_, v2rho2_start) = after_vrho.split_at_mut(v2rho2_local_off);
    let (v2rho2, after_v2rho2) = v2rho2_start.split_at_mut(offsets.v2rho2_len);
    // ... continue pattern for v3rho3, v4rho4

    LdaScratch { zk, vrho, v2rho2, v3rho3, v4rho4 }
}
```

**Field-offsets struct pattern** (from lines 34-46):
```rust
struct LdaFieldOffsets {
    zk_off: usize,   zk_len: usize,
    vrho_off: usize, vrho_len: usize,
    v2rho2_off: usize, v2rho2_len: usize,
    v3rho3_off: usize, v3rho3_len: usize,
    v4rho4_off: usize, v4rho4_len: usize,
}
```

Extend to `GgaFieldOffsets` (15 fields: zk, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2, v3rho3, v3rho2sigma, v3rhosigma2, v3sigma3, v4rho4, v4rho3sigma, v4rho2sigma2, v4rhosigma3, v4sigma4) and `MggaFieldOffsets` (70 fields, the full MGGA superset — see `src/eval/mgga_dispatch/mod.rs:217-235` for the canonical 70-field list as exhaustive zero_field! invocations).

The `lda_field_offsets` body (lines 114-189) **already calculates MGGA-superset offsets** skipping over vsigma/vlapl/vtau slots for LDA fields. For GGA scratch, reuse the same calculator but return vsigma/v2rhosigma/v2sigma2/… offsets instead of skipping them. MGGA scratch returns every offset.

**Replace current placeholders** (lines 23-31):
```rust
// DELETE:
pub struct GgaScratch<'a> {
    _marker: std::marker::PhantomData<&'a mut [f64]>,
}
pub struct MggaScratch<'a> {
    _marker: std::marker::PhantomData<&'a mut [f64]>,
}
// REPLACE with field-full structs matching design doc §10.3 output surface.
```

**Panic-on-unimplemented replaced** (lines 244-255):
```rust
// DELETE todo! bodies:
pub fn gga_scratch_mut(&mut self) -> GgaScratch<'_> {
    todo!("GGA scratch accessor not yet implemented -- Phase 4")
}
pub fn mgga_scratch_mut(&mut self) -> MggaScratch<'_> {
    todo!("MGGA scratch accessor not yet implemented -- Phase 4")
}
```

---

### `src/eval/dispatch.rs` (MODIFY — LDA signature migration)

**Analog:** self, lines 80-87 (current signature) → new signature

**Current** (lines 80-87):
```rust
pub fn dispatch_lda(
    functional: LdaFunctional,
    input: &LdaInput,
    order: DerivativeOrder,
    output: &mut LdaOutput,
    params: &LdaFunctionalParams,
    thresholds: &Thresholds,
) -> Result<(), LibxcRsError>
```

**Post-Phase-5** (D-07):
```rust
pub fn dispatch_lda(
    functional: LdaFunctional,
    input: &LdaInput,
    order: DerivativeOrder,
    output: &mut LdaOutput,
    params: &dyn FunctionalParams,   // <-- trait object replaces typed struct
    thresholds: &Thresholds,
) -> Result<(), LibxcRsError>
```

**Per-arm downcast injection** (lines 161-209 existing → extend each arm):
```rust
// Before:
LdaFunctional::LdaX => launch_lda_x(&ctx, order, spin, params.alpha)?,
// After:
LdaFunctional::LdaX => {
    let p = params.as_any().downcast_ref::<LdaXParams>()
        .ok_or_else(|| LibxcRsError::KernelLaunchFailed {
            reason: "FunctionalParams type mismatch: LdaX expects LdaXParams".into(),
        })?;
    launch_lda_x(&ctx, order, spin, p.alpha)?
},
```

Zero-ext_param arms (e.g. `LdaX2d`, `LdaCVwn`) do NOT downcast — they ignore `params` entirely:
```rust
LdaFunctional::LdaX2d => launch_lda_x_2d(&ctx, order, spin)?,  // unchanged
```

**GGA dispatch** (`src/eval/gga_dispatch/mod.rs:308-314`) — current signature has no `params` arg at all. Add `params: &dyn FunctionalParams` between `output` and `thresholds`. Propagate into `GgaLaunchCtx` if per-functional scalars are threaded.

**MGGA dispatch** (`src/eval/mgga_dispatch/mod.rs:191-197`) — same addition.

**Impact on verify tests:** `verify/tests/lda_oracle.rs:24` currently imports `LdaFunctionalParams`. Either (a) keep `LdaFunctionalParams` as a deprecated alias that implements `FunctionalParams`, or (b) update verify tests to use `&LdaXParams::default()` cast to `&dyn FunctionalParams`. Planner's choice; option (a) minimizes verify test churn.

---

### `src/meta/mod.rs` (MODIFY — extend struct)

**Analog:** self (lines 34-53 current `FunctionalMeta`)

**Current shape** (lines 34-53):
```rust
#[derive(Debug)]
pub struct FunctionalMeta {
    pub id: FunctionalId,
    pub name: &'static str,
    pub kind: Kind,
    pub family: Family,
    pub flags: FunctionalFlags,
    pub references: &'static [Reference],
    pub ext_params: &'static [ExtParamSpec],
    pub default_density_threshold: f64,
    pub auxiliaries: &'static [(FunctionalId, f64)],
    pub hybrid_terms: &'static [HybridTerm],
    pub nlc_params: Option<(f64, f64)>,
    pub max_order: DerivativeOrder,
}
```

**Phase 5 addition** (D-14) — one new field:
```rust
pub hybrid_type: HybridType,    // <-- new; populated by xtask via xc_hyb_type FFI snapshot
```

**Derive `PartialEq`** (RESEARCH A3) — required by `verify/tests/metadata_oracle.rs` field-by-field compare. Cascade: add `PartialEq` to `#[derive]` on `FunctionalMeta`, `Reference` (line 8), `ExtParamSpec` (line 17), `HybridTerm` (line 27). `&'static str` and `&[T]` already compare by value structurally, so no custom impls needed.

---

### `src/error/mod.rs` (MODIFY — 4 new variants)

**Analog:** self (existing variants at lines 3-89)

**Existing variant style** (lines 5-13 + 52-63):
```rust
#[error("external parameter '{name}' not found for functional {id}")]
ExtParamNotFound { id: FunctionalId, name: String },

#[error("external parameter count mismatch for {id}: expected {expected}, got {actual}")]
ExtParamCountMismatch { id: FunctionalId, expected: usize, actual: usize },
```

**New Phase 5 variants** (per CONTEXT.md D-13 + RESEARCH §Claude's Discretion — planner confirms names/messages):
```rust
#[error("external parameter index {index} out of range for functional {id} (has {count} params)")]
ExtParamIndexOutOfRange { id: FunctionalId, index: usize, count: usize },

#[error("unknown external parameter name '{name}' for functional {id}")]
UnknownExtParamName { id: FunctionalId, name: String },

#[error("failed to initialize auxiliary functional {aux_id} for parent {parent_id}: {source}")]
AuxiliaryInitFailed {
    parent_id: FunctionalId,
    aux_id: FunctionalId,
    #[source] source: Box<LibxcRsError>,
},

#[error("propagation conflict for functional {id}: parent param '{parent_name}' targets aux {aux_slot} param '{aux_name}' which is not present")]
PropagationConflict {
    id: FunctionalId,
    parent_name: &'static str,
    aux_slot: u8,
    aux_name: &'static str,
},
```

**Pattern:** all new variants are struct-style (not tuple), reuse `FunctionalId` for the id field, use `&'static str` for xtask-emitted names (these come from static propagation map), `String` only for user-supplied names (`UnknownExtParamName`). The `test_error_is_send_sync` test (line 123-126) must continue to pass — new variants must be `Send + Sync` (no `Rc`, no non-`Send` types).

---

### `libxc-sys/build.rs` (NEW — factor from verify/build.rs)

**Analog:** `verify/build.rs` (entire 40-line file — copy verbatim)

**Full pattern** (from `verify/build.rs:1-40`):
```rust
use std::env;
use std::path::PathBuf;

fn main() {
    // Build vendored libxc 7.0.0 via cmake
    let dst = cmake::Config::new("../libxc-master")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("BUILD_TESTING", "OFF")
        .define("ENABLE_FORTRAN", "OFF")
        .define("ENABLE_PYTHON", "OFF")
        .define("DISABLE_VXC", "OFF")
        .define("DISABLE_FXC", "OFF")
        .define("DISABLE_KXC", "OFF")
        .define("DISABLE_LXC", "OFF")
        .define("CMAKE_POLICY_VERSION_MINIMUM", "3.5")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=xc");
    println!("cargo:rustc-link-search=native={}/lib64", dst.display());

    let header_path = dst.join("include").join("xc.h");
    let bindings = bindgen::Builder::default()
        .header(header_path.to_string_lossy())
        .allowlist_function("xc_.*")
        .allowlist_type("xc_.*")
        .allowlist_var("XC_.*")
        .derive_default(true)
        .generate()
        .expect("Failed to generate bindings for libxc");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("libxc_bindings.rs"))
        .expect("Failed to write bindings");
}
```

**Adjust `"../libxc-master"` path:** for `libxc-sys/build.rs` at workspace root, still `../libxc-master` (sibling of `libxc-sys/`). For `verify/build.rs` the relative path is also `../libxc-master`. Since both crates sit as workspace-root children, the path stays `../libxc-master` in both (verify/build.rs thins to empty once libxc-sys owns the build).

---

### `libxc-sys/src/lib.rs` (NEW)

**Analog:** `verify/src/oracle_ffi.rs` (full 11-line file)

**Full pattern** (from `verify/src/oracle_ffi.rs`):
```rust
//! Raw FFI bindings to C libxc 7.0.0, generated by bindgen at build time.
#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    dead_code,
    clippy::all
)]

include!(concat!(env!("OUT_DIR"), "/libxc_bindings.rs"));
```

Copy verbatim into `libxc-sys/src/lib.rs`. `verify/src/oracle_ffi.rs` then becomes `pub use libxc_sys::*;` (one-line re-export — keeps existing verify callers unchanged).

---

### `libxc-sys/Cargo.toml` (NEW)

**Analog:** `verify/Cargo.toml` [build-dependencies] block (lines 16-18)

**Pattern** (from `verify/Cargo.toml:16-18`):
```toml
[build-dependencies]
bindgen = "0.72.1"
cmake = "0.1.58"
```

**New `libxc-sys/Cargo.toml` full shape:**
```toml
[package]
name = "libxc-sys"
version = "0.1.0"
edition = "2024"
publish = false                    # internal workspace crate only

[dependencies]
# (none — this crate only re-exports FFI bindings)

[build-dependencies]
bindgen = "0.72.1"
cmake = "0.1.58"
```

**Workspace-root change:** `/workspace/Cargo.toml` `[workspace] members` array (lines 130-133 onwards) gains `"libxc-sys",`. Pitfall 2 (RESEARCH): do NOT add `libxc-sys` to the main crate's `[dependencies]`; only `xtask/Cargo.toml` and `verify/Cargo.toml` list it as a path-dep.

---

### `xtask/src/generate_metadata.rs` (NEW)

**Analog:** `xtask/src/main.rs::generate_registry` (lines 91-230) + `generate_meta_file` (lines 287-322)

**File-emission pattern** (from `xtask/src/main.rs:287-322`):
```rust
fn generate_meta_file(root: &Path, entries: &BTreeMap<u16, FuncEntry>) -> Result<()> {
    let path = root.join("src/meta/generated.rs");
    let mut out = String::with_capacity(entries.len() * 300);

    out.push_str("//! Auto-generated by xtask generate-registry. DO NOT EDIT.\n");
    out.push_str("#![allow(non_upper_case_globals)]\n\n");
    out.push_str("use crate::model::{DerivativeOrder, Family, FunctionalFlags, FunctionalId, Kind};\n");
    out.push_str("use crate::meta::FunctionalMeta;\n\n");

    for entry in entries.values() {
        out.push_str(&format!(
            "pub(crate) const {name}: FunctionalMeta = FunctionalMeta {{\n\
             \x20   id: FunctionalId({id}),\n\
             ...
             }};\n\n",
            name = entry.define_name, id = entry.id, kind = entry.kind, family = entry.family,
        ));
    }

    fs::write(&path, &out).with_context(|| format!("failed to write {}", path.display()))?;
    eprintln!("Generated {} entries in {}", entries.len(), path.display());
    Ok(())
}
```

**Table-as-generated-rs pattern** (from `xtask/src/main.rs:324-348` for `by_id` table):
```rust
out.push_str("pub(crate) static REGISTRY_BY_ID: [Option<&'static FunctionalMeta>; 1024] = {\n");
out.push_str("    let mut table: [Option<&'static FunctionalMeta>; 1024] = [None; 1024];\n");
for entry in entries.values() {
    out.push_str(&format!("    table[{id}] = Some(&generated::{name});\n", ...));
}
out.push_str("    table\n};\n");
```

**`generated_hybrid.rs` emits a similar static array** keyed by `FunctionalId` → `HybridType`. `generated_propagation.rs` emits `&[PropagationRule]` slice.

**Subcommand wiring** (from `xtask/src/main.rs:10-33`):
```rust
let command = args.get(1).map(|s| s.as_str()).unwrap_or("help");
match command {
    "generate-registry" => generate_registry()?,
    "verify-phase-4" => { ... },
    "help" | "--help" | "-h" => { ... }
    other => bail!("unknown command: {other}"),
}
```

Add one arm: `"generate-metadata" => generate_metadata::run()?,` and a new `mod generate_metadata;` declaration at the top. Follow the existing `verify_phase_4` submodule wiring pattern (xtask/src/main.rs:7, `mod verify_phase_4;`).

**FFI call pattern inside xtask** (new, from RESEARCH Code Example 2 — no existing analog in xtask, so this is authoritative):
```rust
for id in known_functional_ids() {    // from registry::all_functional_ids()
    let mut t: libxc_sys::xc_func_type = unsafe { std::mem::zeroed() };
    let rc = unsafe { libxc_sys::xc_func_init(&mut t, id.raw() as i32, libxc_sys::XC_UNPOLARIZED) };
    if rc != 0 { bail!("xc_func_init failed for id {id}"); }

    // snapshot fields ...
    unsafe { libxc_sys::xc_func_end(&mut t); }
}
```

`unsafe` is scoped to the snapshot loop — matches BUILD-04 (unsafe confined to FFI boundary, not leaking into `src/` main crate).

---

### `xtask/Cargo.toml` (MODIFY)

**Analog:** self (current 12 lines)

**Current** (full file):
```toml
[package]
name = "xtask"
version = "0.1.0"
edition = "2024"

[[bin]]
name = "xtask"
path = "src/main.rs"

[dependencies]
regex = "1"
anyhow = "1"
```

**Post-Phase-5 addition** (one line to `[dependencies]`):
```toml
libxc-sys = { path = "../libxc-sys" }
```

**Critical (Pitfall 2):** add to `[dependencies]` NOT `[build-dependencies]`. xtask **runs against** libxc at runtime (calls `xc_func_init`); it doesn't build against libxc at compile time.

---

### `verify/Cargo.toml` (MODIFY — swap build-deps)

**Analog:** self (current 18 lines)

**Before** (`verify/Cargo.toml:16-18`):
```toml
[build-dependencies]
bindgen = "0.72.1"
cmake = "0.1.58"
```

**After:**
```toml
# [build-dependencies] block removed entirely

[dependencies]
anyhow = "1.0.100"
libxc_rs = { path = ".." }
libxc-sys = { path = "../libxc-sys" }    # <-- new
```

---

### `verify/build.rs` (MODIFY — shrink)

**Analog:** self (40 lines) → empty or minimal

**Post-Phase-5:** delete entirely, OR reduce to:
```rust
fn main() {
    // libxc linkage now provided by libxc-sys workspace crate.
    // This file retained for Cargo idempotence; actual build logic in libxc-sys/build.rs.
}
```

Linking is transitive via `libxc-sys` path-dep; `cargo:rustc-link-lib=static=xc` lives in `libxc-sys/build.rs` and cargo propagates link flags through workspace deps.

---

### `verify/tests/metadata_oracle.rs` (NEW — D-04 round-trip)

**Analog:** `verify/tests/lda_oracle.rs` (per-id loop + FFI call pattern)

**Import pattern** (from `verify/tests/lda_oracle.rs:24-31`):
```rust
use libxc_rs::eval::{dispatch_lda, LdaFunctionalParams};
use libxc_rs::input::LdaInput;
use libxc_rs::model::{DerivativeOrder, FunctionalId, LdaFunctional, Spin, Thresholds};
use libxc_rs::output::LdaOutput;
use libxc_rs_verify::{
    oracle_func_flags, oracle_lda_all, FLAGS_HAVE_EXC, FLAGS_HAVE_FXC, FLAGS_HAVE_KXC,
    FLAGS_HAVE_LXC, FLAGS_HAVE_VXC, LdaOracleOutput,
};
```

For metadata_oracle.rs, imports become:
```rust
use libxc_rs::{lookup_by_id, FunctionalId, FunctionalMeta};
use libxc_rs::registry::all_functional_ids;
use libxc_sys::{xc_func_type, xc_func_init, xc_func_end, XC_UNPOLARIZED};
```

**FFI-init loop pattern** (from `verify/src/lib.rs:20-37`):
```rust
pub fn oracle_func_flags(func_id: i32, spin: i32) -> Option<i32> {
    unsafe {
        let func = oracle_ffi::xc_func_alloc();
        if func.is_null() { return None; }
        let ret = oracle_ffi::xc_func_init(func, func_id, spin);
        if ret != 0 {
            oracle_ffi::xc_func_free(func);
            return None;
        }
        let info = (*func).info;
        let flags = if info.is_null() { 0 } else { (*info).flags as i32 };
        oracle_ffi::xc_func_end(func);
        oracle_ffi::xc_func_free(func);
        Some(flags)
    }
}
```

Per-id loop in metadata_oracle.rs follows the same init→snapshot→end→free cycle. Compare each field of the Rust static `FunctionalMeta` to the FFI-snapshotted equivalent; use `assert_eq!(rust_meta, ffi_meta)` since `PartialEq` is derived (see src/meta/mod.rs modification above).

**Per-id iteration pattern** (from `verify/tests/lda_oracle.rs:46` — const list form is NOT suitable here; use registry iterator):
```rust
// Use this instead:
for id in libxc_rs::registry::all_functional_ids() {
    let rust_meta = libxc_rs::lookup_by_id(id.raw()).unwrap();
    let ffi_meta = snapshot_from_ffi(id);
    assert_eq!(rust_meta, &ffi_meta, "metadata drift at id {id} ({})", rust_meta.name);
}
```

---

### `verify/tests/hybrid_type_oracle.rs` (NEW — HYB-01)

**Analog:** `verify/tests/lda_oracle.rs` + `src/functional/hybrid.rs::classify_hybrid` (new)

**Three-way compare** (D-14):
```rust
for id in libxc_rs::registry::all_functional_ids() {
    let meta = libxc_rs::lookup_by_id(id.raw()).unwrap();
    let rust_port = libxc_rs::functional::hybrid::classify_hybrid(meta.hybrid_terms);
    let snapshot = meta.hybrid_type;
    let ffi_value = unsafe { ffi_xc_hyb_type(id) };  // wrapper around xc_hyb_type
    assert_eq!(rust_port, snapshot, "rust port != snapshot at id {id}");
    assert_eq!(snapshot, map_ffi_hyb(ffi_value), "snapshot != ffi at id {id}");
}
```

---

### `verify/tests/mixed_oracle.rs` (NEW — FUNC-04)

**Analog:** `verify/tests/lda_oracle.rs` comparison loop + `src/eval/mix.rs::tests::mixed_single_aux_weight_1_matches_dispatch` (lines 231-275)

**Target-functional list** (from CONTEXT.md §What Phase 5 Creates):
```rust
// B3LYP (4 aux), CAM-B3LYP, HSE, mgga_c_b94_hyb (2 aux), wB97X
const MIXED_CASES: &[(&str, &str)] = &[
    ("hyb_gga_xc_b3lyp",       "B3LYP"),
    ("hyb_gga_xc_cam_b3lyp",   "CAM-B3LYP"),
    ("hyb_gga_xc_hse03",       "HSE"),
    ("mgga_c_b94_hyb",         "B94-hyb MGGA"),
    ("hyb_gga_xc_wb97x",       "ωB97X"),
];
```

**Evaluation + oracle compare pattern** (per-functional, mirrors `src/eval/mix.rs::tests:231-275` for the direct→mixed equivalence, and `verify/tests/lda_oracle.rs` lines 80+ for oracle comparison with tolerances from `04-CONTEXT D-10-R`).

---

### `verify/tests/hybrid_oracle.rs` (NEW — HYB-02/03)

**Analog:** `verify/tests/lda_oracle.rs`

**Per-case pattern:**
```rust
let f = Functional::new(FunctionalId::from_name("hyb_gga_xc_cam_b3lyp")?, Spin::Unpolarized)?;
let (omega, alpha, beta) = f.cam_coefficients().ok_or(anyhow!("not CAM"))?;
let (ffi_omega, ffi_alpha, ffi_beta) = ffi_cam_coef(id);  // libxc_sys::xc_hyb_cam_coef
assert!((omega - ffi_omega).abs() < 1e-15);
assert!((alpha - ffi_alpha).abs() < 1e-15);
assert!((beta - ffi_beta).abs() < 1e-15);
```

For NLC (vv10): `let (b, c) = f.nlc_coefficients().ok_or(...)?;` compared against `t.nlc_b` / `t.nlc_C` post-`xc_func_init`.

---

## Shared Patterns

### Shared Pattern 1: Zero-then-accumulate contract

**Source:** `src/eval/mix.rs:75-90` (zero caller output) + `src/eval/dispatch.rs:102-106` (zero caller buffers) + `src/eval/mix.rs:99-105` (zero scratch)

**Apply to:** All new `evaluate_mixed_*` paths, all dispatch signature migrations.

```rust
// Zero the caller's output buffers before accumulation
if let Some(ref mut buf) = output.zk { buf.fill(0.0); }
if let Some(ref mut buf) = output.vrho { buf.fill(0.0); }
// ... every output field

// Inside aux loop:
workspace.zero_scratch();
// ... dispatch aux into scratch
// ... add_to_mix from scratch into output
```

**Rationale:** Phase 3 D-11/D-12 contract. `dispatch_*` zero caller buffers; mixed paths zero AGAIN before each aux. Double-zero is intentional per `src/eval/mix.rs:100-104` comment.

---

### Shared Pattern 2: Workspace-mismatch validation

**Source:** `src/eval/mix.rs:63-70`

**Apply to:** Every public `evaluate_mixed_*` entry point (gga, mgga) and every method that takes a workspace.

```rust
if workspace.np() != input.np() || workspace.spin() != input.spin() {
    return Err(LibxcRsError::WorkspaceMismatch {
        expected_np: input.np(),
        actual_np: workspace.np(),
        expected_spin: input.spin(),
        actual_spin: workspace.spin(),
    });
}
```

`WorkspaceMismatch` variant already in `src/error/mod.rs:74-80`.

---

### Shared Pattern 3: Derivative-order gating

**Source:** `src/eval/dispatch.rs:89-95` + `src/eval/mix.rs:120-134` + `src/eval/mgga_dispatch/mod.rs:240-246`

**Apply to:** Every new evaluate/dispatch method.

```rust
if order == DerivativeOrder::Exc && !functional.has_exc() {
    return Err(LibxcRsError::UnsupportedDerivativeOrder {
        id: functional.to_id(),
        order,
        max: DerivativeOrder::Lxc,
    });
}
// Per-order conditional field handling:
if order >= DerivativeOrder::Vxc
    && let Some(ref mut dst) = output.vrho
{
    /* ... */
}
```

Use `order >= DerivativeOrder::Vxc` cascade (Vxc → Fxc → Kxc → Lxc) because `DerivativeOrder` derives `Ord` at `src/model/mod.rs:38-46`.

---

### Shared Pattern 4: Registry lookup idiom

**Source:** `src/model/mod.rs:92-126` + `src/registry/mod.rs:12-35`

**Apply to:** `Functional::new`, every metadata query getter on `Functional`.

```rust
let meta: &'static FunctionalMeta = crate::registry::lookup_by_id(id.raw())?;
// All further metadata access is &'static — never allocates.
```

The `?` surfaces `UnknownFunctionalId` / `RemovedFunctionalId` from `LibxcRsError`. Store `meta: &'static FunctionalMeta` on `Functional` — one pointer, no allocation, `Sync` automatic.

---

### Shared Pattern 5: thiserror v2 error variant shape

**Source:** `src/error/mod.rs:3-89` (existing variants)

**Apply to:** All 4 new Phase 5 error variants.

```rust
#[error("human-readable message with {field} interpolation")]
VariantName {
    id: FunctionalId,           // prefer typed Ids over raw u16
    name: String,               // String only for user-supplied strings
    expected: &'static str,     // &'static str for xtask-emitted strings
    #[source] source: Box<LibxcRsError>,  // for chained errors (AuxiliaryInitFailed)
},
```

**Send+Sync is load-bearing** — see `src/error/mod.rs:123-126` test. Do not introduce `Rc`, `Cell`, or non-`Send` types in variants.

---

### Shared Pattern 6: xtask-generated file headers + commit policy

**Source:** `xtask/src/main.rs:291` + `src/registry/by_id.rs` (a sibling example) + CLAUDE.md Phase 1 D-04

**Apply to:** `src/meta/generated.rs` (rewritten), `src/meta/generated_hybrid.rs` (new), `src/meta/generated_propagation.rs` (new), `src/registry/by_id.rs` (unchanged but sibling).

```rust
//! Auto-generated by xtask generate-metadata. DO NOT EDIT.
#![allow(non_upper_case_globals)]

use crate::model::{DerivativeOrder, Family, FunctionalFlags, FunctionalId, HybridType, Kind};
use crate::meta::{FunctionalMeta, Reference, ExtParamSpec, HybridTerm};
```

**Commit policy:** Files are committed to git (Phase 1 D-04). CI does not regenerate on every build. `cargo xtask generate-metadata` is manual (D-05).

---

### Shared Pattern 7: `&dyn FunctionalParams` plumbing

**Source:** This pattern is new in Phase 5. Defined in `src/functional/params.rs` (per-plan-05-02).

**Apply to:** `dispatch_lda`, `dispatch_gga`, `dispatch_mgga`, `evaluate_mixed_{lda,gga,mgga}`, every test that constructs a Functional.

```rust
// In Functional::new:
let params: Box<dyn FunctionalParams + Send + Sync> =
    crate::functional::params::construct_params(meta.id, ext_params.as_deref())?;

// At dispatch call site:
dispatch_lda(lda_fn, input, order, output, &*self.params, &self.thresholds)?;
//                                            ^^^^^^^^^^^ Box<dyn> → &dyn
```

**Thread-safety:** `FunctionalParams: Send + Sync` bound (CONTEXT D-13). No interior mutability — setters take `&mut self`.

**Zero-param functionals:** construct once as `Box::new(NoParams)` — per-instance allocation is fine per CONTEXT D-13, EVAL-04 still satisfied (no alloc in the **evaluation hot path**; construction is out-of-hot-path).

---

## No Analog Found

| File | Role | Data Flow | Reason |
|------|------|-----------|--------|
| `src/meta/generated_hybrid.rs` (NEW) | xtask-emitted data | — | No existing per-id static table of enum values (`REGISTRY_BY_ID` is a `[Option<&FunctionalMeta>; 1024]` — close but different shape). New file; use `xtask/src/main.rs::generate_by_id_file` (lines 324-348) as structural template. |
| `src/meta/generated_propagation.rs` (NEW) | xtask-emitted data | — | No existing static table of struct records. Use `src/registry/removed.rs` (emitted by `generate_removed_file` at `xtask/src/main.rs:378-411`) as structural template — it emits `&[(u16, u16)]` and `&[(&str, u16)]` slices in the same "static slice of tuples" style. |

---

## Metadata

**Analog search scope:**
- `/workspace/src/**` (main crate)
- `/workspace/verify/**` (oracle harness)
- `/workspace/xtask/**` (code generator)
- `/workspace/libxc-master/src/hybrids.c`, `mix_func.c` (port spec)
- `/workspace/Cargo.toml` (workspace manifest)

**Files scanned:**
- `src/eval/mix.rs` (full read — 453 lines; template for mixed_gga/mgga)
- `src/eval/workspace.rs` (full read — 364 lines; template for GgaScratch/MggaScratch)
- `src/eval/dispatch.rs` (lines 1-210 read; template for signature migration)
- `src/eval/gga_dispatch/mod.rs` (lines 1-200, 300-470 read; signature migration ref)
- `src/eval/mgga_dispatch/mod.rs` (lines 1-280 read; signature migration ref)
- `src/eval/mod.rs` (full read — 11 lines; re-export template)
- `src/meta/mod.rs` (full read — 53 lines; struct extension target)
- `src/meta/{hybrid,auxiliary,ext_param,nlc,reference,functional_meta,library}.rs` (all 2-line stubs — verified empty)
- `src/model/mod.rs` (full read — 271 lines; HybridType + HybridTermKind + FunctionalId)
- `src/registry/mod.rs` (full read — 189 lines; `all_functional_ids` confirmed)
- `src/error/mod.rs` (full read — 142 lines; variant shape + test_error_is_send_sync)
- `src/lib.rs` (full read — 31 lines; pub mod + re-export)
- `verify/build.rs` (full read — 40 lines; copy verbatim to libxc-sys)
- `verify/src/oracle_ffi.rs` (full read — 11 lines; copy verbatim to libxc-sys/src/lib.rs)
- `verify/src/lib.rs` (lines 1-60 read; FFI init/end idiom)
- `verify/Cargo.toml` (full read; build-deps block → libxc-sys swap)
- `verify/tests/lda_oracle.rs` (lines 1-80 read; per-id loop shape for metadata_oracle)
- `xtask/src/main.rs` (full read — 412 lines; subcommand wiring + generate_*_file shapes)
- `xtask/Cargo.toml` (full read; dependency block for libxc-sys addition)
- `Cargo.toml` (lines 128-170 read; workspace members list)
- `libxc-master/src/hybrids.c` (lines 60-157 read; xc_hyb_type + cam_coef + exx_coef port)
- `libxc-master/src/mix_func.c` (lines 150-310 read; per-aux family gating for mixed-gga/mgga)

**Total LOC analyzed:** ~3,800 lines; no duplicate reads.

**Pattern extraction date:** 2026-04-24
