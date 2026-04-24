# Phase 5: Functional Lifecycle and Hybrid Properties - Research

**Researched:** 2026-04-24
**Domain:** Runtime Functional handle, FFI-free registry metadata snapshotting via xtask, FunctionalParams trait plumbing, hybrid classification port, recursive auxiliary construction, full mixed GGA/MGGA evaluation
**Confidence:** HIGH (primary sources are the in-tree libxc C code and the existing libxc_rs Rust code; Context7 was not consulted because the "ecosystem" for this phase is `libxc-master/src/` itself, which is vendored)

## Summary

Phase 5 is a **plumbing phase**, not a kernel-writing phase. All 649 IDs already have FunctionalMeta skeletons (`src/meta/generated.rs`) and all 229 compiled kernel launches already work via `dispatch_{lda,gga,mgga}` (Phase 4). What's missing is: (a) the runtime `Functional` handle that owns state between calls, (b) the FunctionalMeta hybrid/aux/nlc/ext_params/references fields currently set to `&[]` / `None`, (c) a `FunctionalParams` trait that replaces the Phase 4 "hardcoded libxc defaults at call site" hack with real ext_params plumbing, (d) materialized `GgaScratch`/`MggaScratch` views so `evaluate_mixed_gga` and `evaluate_mixed_mgga` can exist, and (e) a Rust port of `xc_hyb_type()` that classifies every hybrid into `HybridType`. [VERIFIED: libxc-master/src/hybrids.c:82-118], [VERIFIED: src/meta/generated.rs:1-9741]

The primary technical risk is **ext_params propagation semantics**, not kernel correctness: libxc has ~15 named `set_ext_params_*` helper functions in `util.c` (cpy, cam, camy, cam_sr, lc, lcy, omega, exx, and `cpy_*` variants of each) that read ext_params in a defined order and push specific sub-ranges to `hyb_coeff[]`, `hyb_omega[]`, and `func_aux[i]->ext_params`. A propagation map that only supports `Copy` from a parent ext_param slot to an aux ext_param slot will **not** cover `set_ext_params_cam` (which writes derived values into `hyb_coeff[0]` = beta and `hyb_coeff[1]` = alpha, not to an aux). The xtask must snapshot **both** aux ext_params propagation **and** hyb_coeff/hyb_omega/mix_coef *after* calling the functional's `set_ext_params` callback — capturing the derived values rather than the formulas. [VERIFIED: libxc-master/src/util.c:100-285], [VERIFIED: libxc-master/src/hyb_gga_xc_b2plyp.c:94-120]

**Primary recommendation:** Snapshot *values* (not *formulas*) at xtask-run time. Call `xc_func_init(id, XC_UNPOLARIZED)`, immediately snapshot the post-init `hyb_coeff`, `hyb_omega`, `hyb_type`, `nlc_b`, `nlc_C`, `mix_coef`, and `func_aux[i]->ext_params` arrays (these reflect libxc's default ext_params application), then for each propagation edge (parent ext_param name → aux slot `i` ext_param name) encode only `Copy` transforms. Derived transforms (CAM-style `alpha = 1 - ext[0]`, Yukawa `omega` copied raw, etc.) are handled by per-functional Rust `set_ext_params` callbacks on the `FunctionalParams` impl — not by a generic propagation table. This bifurcation matches libxc's own architecture: the `func_params_type.set` callback is per-functional, and the xtask-generated table only needs to describe the subset that actually invokes `xc_func_set_ext_params_name(p->func_aux[i], ...)`.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions (D-01 through D-17)

**D-01** Metadata source = xtask links libxc and snapshots `xc_func_type` at xtask-run time. For each of 649 IDs the xtask calls `xc_func_init(&mut t, id, XC_UNPOLARIZED)`, reads `t.info->ext_params`, `t.hyb_number_terms`/`hyb_type`/`hyb_coeff`/`hyb_omega`, `t.func_aux[]` IDs, `t.nlc_b`/`nlc_C`, `t.info->flags`, `t.info->references[]`, and emits static Rust entries. The xtask process is the only thing that links libxc in the Phase 5 workflow — committed generated Rust output keeps `libxc_rs` itself FFI-free at runtime.

**D-02** Snapshot scope = ALL `FunctionalMeta` fields, including `references` (citation / DOI / bibtex / key).

**D-03** xtask location = new subcommand `cargo xtask generate-metadata` inside the existing `xtask/` crate. A new workspace member `libxc-sys` is factored out of the current `verify/build.rs` so that both `verify/` and `xtask` link the same libxc build — one cmake, one source-of-truth bindgen. Existing `generate-registry` subcommand is preserved.

**D-04** Validation gate = verify/ round-trip test `verify/tests/metadata_oracle.rs`. For every id in `lookup_all_ids()`, construct an `xc_func_type` via FFI, compare our static `FunctionalMeta` field-by-field. No version-checksum mechanism.

**D-05** Regen policy = manual. Developer runs `cargo xtask generate-metadata` on libxc version bump; generated files committed.

**D-06** Runtime ext_params storage on `Functional` = `Option<Box<[f64]>>`. `None` when `meta.ext_params.is_empty()` — majority of functionals, preserves EVAL-04 zero-alloc. `Some(Box<[f64]>)` when functional has ext_params; initialized from `meta.ext_params[i].default_value`.

**D-07** Dispatch signatures after Phase 5 = `dispatch_{lda,gga,mgga}` take a `&dyn FunctionalParams` trait object (replacing current typed `LdaFunctionalParams { alpha }` and the GGA/MGGA call-site literal scalar args).

**D-08** Derived-parameter computation = `FunctionalParams` trait with one concrete `impl` per functional. Trait exposes `raw_ext_params()`, `set_ext_params()`, `as_any()` for dispatch downcast. Zero-param functionals get a blanket `NoParams: FunctionalParams`.

**D-09** Wiring rollout = all 229 compiled functionals get real ext_params plumbing in Phase 5 (37 LDA + 106 GGA + 86 MGGA). Deferred functionals (4 LDA + 6 MGGA) continue to return `UnsupportedFunctional`; they get minimal `FunctionalParams` impls for trait cohesion.

**D-10** Module location = new `src/functional/` top-level module (NOT `src/func/`). Internal layout: `functional/mod.rs`, `lifecycle.rs`, `config.rs`, `params.rs` (possibly split per family), `hybrid.rs`.

**D-11** Evaluation API = `Functional::evaluate_{lda,gga,mgga}` methods delegate to the existing free `dispatch_{lda,gga,mgga}` functions. Functional handles mixed detection (non-empty auxiliaries → route to `evaluate_mixed_*`; empty → direct dispatch). Free `dispatch_*` functions stay public.

**D-12** Mixed GGA/MGGA paths = Phase 5 fully materializes both. `evaluate_mixed_gga` and `evaluate_mixed_mgga` mirror `evaluate_mixed_lda`. `GgaScratch` and `MggaScratch` replace `PhantomData` placeholders with real `split_at_mut`-carved slices over the MGGA-superset workspace buffer.

**D-13** Thread-safety = `FunctionalParams: Send + Sync` is mandatory. `Functional` auto-derives `Send + Sync`. `set_*` methods take `&mut self`, no interior mutability.

**D-14** HybridType strategy = both snapshot + Rust port. `FunctionalMeta.hybrid_type: HybridType` populated by xtask calling `xc_hyb_type(p)`; Rust port lives in `src/functional/hybrid.rs`. Verify test compares both across all 649 IDs.

**D-15** Auxiliary construction = eager, recursive, at `Functional::new` call-time. `Functional.auxiliaries: Vec<Functional>`. Empty Vec for non-hybrids, length 1–5 for hybrids (B3LYP = 4 aux, mgga_c_b94 = 2). Drop is no-op beyond automatic `Vec<Functional>` recursive drop.

**D-16** Auxiliary ext_params propagation = xtask-generated static propagation map, NOT per-functional Rust callback code. Committed as a static `&'static [(parent_id, parent_idx, aux_slot, aux_param_name)]` table. `Functional::new` reads this map and copies values after constructing auxiliaries.

**D-17** Aux depth = static bound of 2. xtask walks aux graph at snapshot time and asserts `max_aux_depth ≤ 2`. No runtime cycle detection.

### Claude's Discretion
- Plan decomposition across 3 plans (suggested split in CONTEXT.md §decisions)
- Exact shape of `FunctionalParams` trait (getters by name vs index, error variants, `set_ext_params` mutation semantics)
- Whether per-functional `FunctionalParams` impls are hand-written, macro-generated, or xtask-emitted
- Internal file layout of `src/functional/` (single `params.rs` vs per-family split)
- Error variant names/messages for the 4 new Phase 5 errors
- Exact `GgaScratch`/`MggaScratch` field layout inside `EvaluationWorkspace`'s contiguous buffer
- Whether free `dispatch_*` functions stay `pub` or become `pub(crate)`
- Whether to fold `xc_hyb_exx_coef` / `xc_hyb_cam_coef` into Functional methods or keep them as free functions

### Deferred Ideas (OUT OF SCOPE)
- Enabling the 4 deferred LDA + 6 deferred MGGA functionals (permanently deferred from Phase 4)
- `FunctionalBuilder` chainable API → Phase 6 (API-01)
- `BatchEvaluator` with reusable workspace → Phase 6 (API-02)
- Ergonomic `evaluate()` auto-dispatch by family → Phase 6 (API-03)
- `extern "C"` compat layer → Phase 6 (COMPAT-01..03)
- GPU backends + f64 capability check → Phase 7 (GPU-01..07)
- Performance benchmarks → Phase 7 (PERF-01..05)
- Runtime `references()` getter API polish → Phase 10
- Non-Copy propagation transforms (if any exist in libxc) — see Open Questions
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| FUNC-01 | `Functional::new(id, spin)` constructs instance with correct metadata, dimensions, thresholds, ext_params | §Architecture Patterns "Functional construction flow"; D-01/D-06/D-15 cover all required data |
| FUNC-02 | External parameter management: set/get by name, by index, bulk set/get | §FunctionalParams trait shape; §design doc 5.6; 5 C functions → 5 Rust methods |
| FUNC-03 | Threshold configuration: density, zeta, sigma, tau thresholds settable | 4 setter methods, `&mut self`; `Thresholds` struct already in `src/model/mod.rs` |
| FUNC-04 | Auxiliary functional initialization for hybrid/mixed functionals (recursive construction) | §Eager Aux Construction; D-15 + D-17; max arity empirically 1..=5 |
| FUNC-05 | FunctionalParams trait for per-functional computed parameters derived from ext_params | D-07/D-08; trait with `raw_ext_params`, `set_ext_params`, `as_any` |
| FUNC-06 | Drop implementation cleans up resources | D-15: Drop is no-op; `Vec<Functional>` and `Box<dyn>` auto-drop; no FFI handles to release |
| HYB-01 | HybridType classification (Semilocal, Hybrid, Cam, CamYukawa, etc.) | Rust port of `xc_hyb_type()` §libxc Source Analysis; D-14 |
| HYB-02 | CAM coefficient extraction (omega, alpha, beta) | Port of `xc_hyb_cam_coef()` + `xc_hyb_exx_coef()` §libxc Source Analysis |
| HYB-03 | NLC coefficient extraction (b, C) | Read directly from `FunctionalMeta.nlc_params: Option<(f64, f64)>` (already a field) |
| HYB-04 | Auxiliary functional iteration (IDs and weights) | Expose `Functional.auxiliaries: &[Functional]` + `mix_coefficients: &[f64]`; ids via `aux.meta().id` |
</phase_requirements>

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|--------------|----------------|-----------|
| Build-time libxc linkage | Build tier (xtask + libxc-sys) | verify/ | Phase 1 D-04 and CONTEXT.md D-01 both require committed generated Rust; main crate stays FFI-free |
| Static metadata storage | Compile-time data (`src/meta/generated.rs`) | Rust compiler `.rodata` | Single source of truth; no runtime loading |
| Functional lifecycle | Runtime control plane (`src/functional/`) | — | Owns ext_params, thresholds, aux Functionals between evaluation calls |
| Per-functional ext_params derivation | Runtime data plane (`FunctionalParams` trait impls) | — | Each of 229 functionals has its own derivation rule (most are `Copy`) |
| Dispatch to kernel | Runtime data plane (`src/eval/dispatch.rs`) | `src/eval/{gga,mgga}_dispatch/` | Trait object downcast + kernel launch; BUILD-04 invariant — all `launch_unchecked` stays here |
| Mixed evaluation accumulation | Runtime data plane (`src/eval/mix.rs`) | `EvaluationWorkspace` | Zero-then-`+=` in scratch, then `output += weight * scratch`. Mirrors libxc `mix_func.c` |
| Hybrid queries (HYB-01..04) | Runtime introspection (`src/functional/hybrid.rs`) | `FunctionalMeta.hybrid_terms` + `Functional.auxiliaries` | Pure-data queries over committed metadata + runtime-constructed aux tree |
| Auxiliary construction | Runtime control plane (`Functional::new` recursion) | — | Eager, done once at construction, no lazy init |
| Oracle verification | Test tier (`verify/tests/metadata_oracle.rs`) | libxc-sys | Only place that holds libxc at runtime; isolated from main crate dep graph |

## Standard Stack

### Core (already in workspace, no new versions)

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| thiserror | 2.0.18 | Library-boundary error enum (`LibxcRsError`) | Already in Cargo.toml. Phase 5 extends the existing enum with 4 new variants; no version bump. [VERIFIED: /workspace/Cargo.toml:8] |
| cubecl (feature=cpu) | 0.9.0 | Kernel dispatch substrate (unchanged from Phase 4) | Already in Cargo.toml. Phase 5 does not touch `#[cube]` code. [VERIFIED: /workspace/Cargo.toml:7] |
| bitflags | 2.10.0 | `FunctionalFlags` bitfield | Already in use. xtask snapshotting needs to read libxc's flag bits and emit `FunctionalFlags::from_bits_retain(...)` initializers. [VERIFIED: /workspace/Cargo.toml:6] |
| bytemuck | 1.25.0 | Unchanged (kernel launch layer only) | N/A for Phase 5 — no new GPU code |

### Build-time / xtask (new workspace dependency)

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| libxc-sys (new workspace crate) | 0.1.0 (internal) | Factored cmake + bindgen for libxc 7.0.0 | Currently `verify/build.rs` has this logic. Factoring enables both `verify/` and `xtask/` to depend on the same `libxc-sys` path dep, avoiding double cmake builds. [CITED: CONTEXT.md D-03] |
| bindgen | 0.72.1 | Generate FFI bindings from `xc.h` | Already in `verify/Cargo.toml` build-deps. Moves to `libxc-sys/build.rs`. |
| cmake | 0.1.58 | Build vendored `libxc-master/` | Already in `verify/Cargo.toml` build-deps. Moves to `libxc-sys/build.rs`. |
| anyhow | 1.0.100 | Application-tier errors in xtask only (never in main crate) | Already in `xtask/Cargo.toml`. [VERIFIED: /workspace/xtask/Cargo.toml:12] |

### Standard library (no external dep needed)

| Item | Purpose |
|------|---------|
| `std::any::Any` | Downcast trait object `&dyn FunctionalParams` to concrete type in dispatch arms. Trait object safety: `as_any(&self) -> &dyn Any` works because `Any: ?Sized` bound is lifted when called. [VERIFIED: https://doc.rust-lang.org/std/any/trait.Any.html] |
| `std::sync::OnceLock` (potential) | If a singleton `NoParams` `&'static dyn FunctionalParams` is preferred over per-instance allocation. See §FunctionalParams trait |

### NOT Recommended

| Instead of | Could Use | Why Not |
|------------|-----------|---------|
| `&'static dyn FunctionalParams` (with `Box::leak`) | Per-instance `Box<dyn FunctionalParams + Send + Sync>` | D-13 requires Send+Sync; per-instance mutation via `&mut self` on setters. `Box::leak` is irrevocable and makes tests flaky. Standard pattern is `Box<dyn ...>`. (Exception: the `NoParams` singleton for zero-param functionals — see §specifics in CONTEXT.md.) |
| `downcast-rs` crate | `std::any::Any` manually (`as_any()` method) | Downcast-rs is convenient but adds a dependency for ~20 lines of boilerplate. `as_any` is idiomatic Rust. [CITED: CONTEXT.md §specifics] |
| `Arc<FunctionalMeta>` in `Functional` | `&'static FunctionalMeta` | Metadata is `.rodata`; static reference is correct. Arc would imply heap allocation per instance. |
| `Rc<Functional>` for shared aux | `Vec<Functional>` with potential duplication | Aux construction is eager and each parent owns its own aux tree. Shared aux is a Phase 6+ optimization; duplication at depth ≤ 2 costs trivial memory. |
| Global `xc_func_type` cache | Per-construction `xc_func_init` call in xtask | xtask runs at generate-metadata time only (manual, D-05), so no caching needed. |

**Installation:** None required — all dependencies already in the workspace. The only structural change is creating `libxc-sys/` as a new workspace member, moving the existing cmake+bindgen from `verify/build.rs` into `libxc-sys/build.rs`, and adding `libxc-sys` as a path dep to both `verify/Cargo.toml` and `xtask/Cargo.toml`.

**Version verification performed:**
- thiserror 2.0.18, cubecl 0.9.0, bitflags 2.10.0, bytemuck 1.25.0: confirmed in `/workspace/Cargo.toml` (no change needed)
- bindgen 0.72.1, cmake 0.1.58: confirmed in `/workspace/verify/Cargo.toml` (to be moved, not upgraded)
- No new crates introduced.

## Architecture Patterns

### System Architecture Diagram

```
                                ┌─────────────────────────┐
                                │  User code              │
                                │  (verify/, bench/, main)│
                                └────────────┬────────────┘
                                             │
                                             ▼
                            ┌────────────────────────────────┐
                            │  Public API: Functional       │
                            │  ─────────────────────────    │
                            │  Functional::new(id, spin)    │◀──── registry::lookup_by_id
                            │  set_{density,zeta,...}_threshold
                            │  set_ext_param(name,val)      │
                            │  ext_params() / ext_param()   │
                            │  hybrid_type()                │
                            │  cam_coefficients()           │
                            │  nlc_coefficients()           │
                            │  auxiliary_functionals()      │
                            │  evaluate_{lda,gga,mgga}()    │
                            └────────────┬───────────────────┘
                                         │
                      ┌──────────────────┴──────────────────┐
                      │                                     │
              non-mixed path                          mixed path
              (aux.is_empty())                    (aux.len() > 0)
                      │                                     │
                      ▼                                     ▼
          ┌──────────────────────┐         ┌───────────────────────────────┐
          │ dispatch_{lda,gga,mgga} │       │ evaluate_mixed_{lda,gga,mgga} │
          │ (free fn, src/eval/)    │       │ (src/eval/mix.rs)             │
          │ takes &dyn FunctionalParams │    │ loops aux, calls dispatch_*,  │
          │                         │       │ accumulates via add_to_mix    │
          └─────────┬────────────────┘       └─────────────┬─────────────────┘
                    │                                      │
                    │  (both paths)                        │
                    ▼                                      ▼
        ┌──────────────────────────────────────────────────────────┐
        │  #[cube] kernel launch (crates/kernel-{lda,gga,mgga}-*)  │
        │  BUILD-04 invariant: raw launch_unchecked only here      │
        └──────────────────────────────────────────────────────────┘

            ┌────────────────────────────────────────────────────┐
            │  Build-time (xtask + libxc-sys — separate graph)  │
            │                                                    │
            │   cargo xtask generate-metadata                   │
            │     │                                              │
            │     ▼                                              │
            │   libxc-sys (cmake + bindgen libxc-master/)       │
            │     │                                              │
            │     ▼                                              │
            │   foreach id in 0..=maxid:                        │
            │     xc_func_init(&mut t, id, XC_UNPOLARIZED)      │
            │     snapshot { ext_params, hyb_*, nlc_*,          │
            │                func_aux[], flags, references }    │
            │     emit src/meta/generated.rs entry              │
            │     emit src/meta/generated_propagation.rs rule   │
            │     emit src/meta/generated_hybrid.rs hybrid_type │
            │                                                    │
            │   verify/tests/metadata_oracle.rs:                │
            │     foreach id: compare committed meta to live FFI │
            └────────────────────────────────────────────────────┘
```

### Recommended Project Structure

```
src/
├── functional/              # NEW — Phase 5's home
│   ├── mod.rs               # Functional struct + pub re-exports
│   ├── lifecycle.rs         # new(), Drop (trivial), mixed detection
│   ├── config.rs            # threshold setters, ext_param set/get
│   ├── params.rs            # FunctionalParams trait + NoParams
│   ├── params_lda.rs        # 37 LDA-family impls (optional split)
│   ├── params_gga.rs        # 106 GGA-family impls (optional split)
│   ├── params_mgga.rs       # 86 MGGA-family impls (optional split)
│   └── hybrid.rs            # xc_hyb_type/exx/cam ports, CamCoefficients, NlcCoefficients structs
├── meta/
│   ├── generated.rs         # REWRITTEN in place: full field population for 649 IDs
│   ├── generated_hybrid.rs  # NEW — snapshotted HybridType per id
│   ├── generated_propagation.rs  # NEW — parent→aux ext_param Copy edges
│   └── ... (existing stubs populated OR deleted)
├── eval/
│   ├── dispatch.rs          # CHANGED — takes &dyn FunctionalParams
│   ├── gga_dispatch/mod.rs  # CHANGED — takes &dyn FunctionalParams
│   ├── mgga_dispatch/mod.rs # CHANGED — takes &dyn FunctionalParams
│   ├── mix.rs               # CHANGED — gains evaluate_mixed_gga + evaluate_mixed_mgga
│   └── workspace.rs         # CHANGED — GgaScratch/MggaScratch materialized
├── error/mod.rs             # CHANGED — 4 new variants
└── lib.rs                   # CHANGED — pub mod functional; re-export Functional

libxc-sys/                   # NEW workspace member
├── Cargo.toml
├── build.rs                 # cmake + bindgen (moved from verify/build.rs)
└── src/lib.rs               # include!(concat!(env!("OUT_DIR"), "/libxc_bindings.rs"));

verify/
├── Cargo.toml               # CHANGED — libxc-sys path dep, no direct cmake+bindgen
├── build.rs                 # DELETED or stub
└── tests/
    ├── metadata_oracle.rs   # NEW — full round-trip field compare
    ├── hybrid_type_oracle.rs # NEW — Rust port vs snapshotted vs FFI (triangulation)
    └── ... (existing kernel oracle tests unchanged)

xtask/
├── Cargo.toml               # CHANGED — libxc-sys path dep (build dep)
└── src/
    ├── main.rs              # CHANGED — new "generate-metadata" subcommand
    └── generate_metadata.rs # NEW — snapshotting logic
```

### Pattern 1: FunctionalParams trait with Any-based downcast

**What:** A `Send + Sync` trait object held by each `Functional` instance. Each of 229 compiled functionals gets one impl. Dispatch arms downcast via `as_any().downcast_ref::<LdaXParams>()` to extract per-functional derived scalars for the kernel launch.

**When to use:** For every ext_params-bearing functional. Functionals with zero ext_params (most of the 229) share a blanket `NoParams` impl.

**Example (sketch):**
```rust
// src/functional/params.rs
use std::any::Any;
use crate::error::LibxcRsError;
use crate::model::FunctionalId;

pub trait FunctionalParams: Send + Sync {
    /// Number of ext_params this functional exposes. Zero for NoParams.
    fn ext_param_count(&self) -> usize;

    /// Raw ext_params values as currently set.
    fn raw_ext_params(&self) -> &[f64];

    /// Bulk set; validates length matches ext_param_count.
    /// Triggers recomputation of any derived fields (CAM alpha/beta/omega, etc.).
    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError>;

    /// Downcast escape hatch for dispatch arms.
    fn as_any(&self) -> &dyn Any;
}

// Zero-ext_param blanket impl. Singleton-able.
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

// Per-functional impl example (lda_x).
pub struct LdaXParams {
    raw: [f64; 0],           // empty, no ext_params -- libxc default α=1.0 handled differently
    pub(crate) alpha: f64,   // derived — kept for dispatch downcast
}
// ...
```

**Source:** [CITED: docs/design/libxc_rs_detailed_design.md §6.8], [CITED: CONTEXT.md D-07/D-08], [VERIFIED: existing dispatch.rs::LdaFunctionalParams pattern]

### Pattern 2: Eager recursive aux construction

**What:** `Functional::new(id, spin)` recurses into `meta.auxiliaries` constructing each aux Functional, depth ≤ 2 (enforced by xtask at snapshot time, D-17).

**When to use:** Always at construction time — never lazy. This matches design doc §10.1 and libxc's C behavior (`xc_mix_init` calls `xc_func_init` per aux).

**Example:**
```rust
// src/functional/lifecycle.rs
impl Functional {
    pub fn new(id: FunctionalId, spin: Spin) -> Result<Self, LibxcRsError> {
        let meta = registry::lookup_by_id(id.raw())?;  // validates id

        // Dimension calculation per family.
        let dims = match meta.family {
            Family::Lda  => Dimensions::lda(spin),
            Family::Gga  => Dimensions::gga(spin),
            Family::Mgga => Dimensions::mgga(spin),
        };

        // ext_params: None if empty spec, else Box<[f64]> from defaults.
        let ext_params = if meta.ext_params.is_empty() {
            None
        } else {
            Some(
                meta.ext_params
                    .iter()
                    .map(|spec| spec.default_value)
                    .collect::<Vec<f64>>()
                    .into_boxed_slice(),
            )
        };

        // Per-functional FunctionalParams construction (table-dispatched by id).
        let params: Box<dyn FunctionalParams + Send + Sync> =
            construct_params(meta.id, ext_params.as_deref())?;

        // Aux recursion (depth ≤ 2 by D-17).
        let mut auxiliaries = Vec::with_capacity(meta.auxiliaries.len());
        for &(aux_id, _weight) in meta.auxiliaries {
            auxiliaries.push(Functional::new(aux_id, spin)?);
        }

        // After aux are built, push parent's ext_params into aux slots per propagation map.
        let propagation = crate::meta::generated_propagation::rules_for(meta.id);
        for rule in propagation {
            // rule.parent_param_idx → aux_slot[rule.aux_slot].set_ext_param(rule.aux_param_name, val)
            // ... copy logic, see §Ext_params propagation
        }

        let mix_coefficients = meta.auxiliaries.iter().map(|&(_, w)| w).collect();

        Ok(Functional {
            meta,
            spin,
            dims,
            thresholds: Thresholds::default(),
            ext_params,
            params,
            auxiliaries,
            mix_coefficients,
        })
    }
}

impl Drop for Functional {
    fn drop(&mut self) {
        // No-op: Vec<Functional>, Box<[f64]>, Box<dyn> all drop automatically.
        // No FFI handles to release (main crate is FFI-free).
    }
}
```

**Source:** [VERIFIED: docs/design/libxc_rs_detailed_design.md:1294-1316], [CITED: libxc-master/src/mix_func.c:31 xc_mix_init]

### Pattern 3: Mixed GGA evaluation (template — mirrors existing evaluate_mixed_lda)

**What:** For GGA-family mixed functionals (all the HYB_GGA_XC_* like B3LYP, CAM-B3LYP, HSE), evaluate each aux into GGA-shaped scratch, then accumulate weighted sum into caller output across all 15 GGA-applicable derivative fields (1 zk + 2 order-1 + 3 order-2 + 4 order-3 + 5 order-4).

**When to use:** `Functional::evaluate_gga` routes here when `!self.auxiliaries.is_empty()`.

**Example (abbreviated — full fan-out mirrors `evaluate_mixed_lda`):**
```rust
// src/eval/mix.rs (new function)
pub fn evaluate_mixed_gga(
    functional: &Functional,             // carries aux list + mix_coefs
    input: &GgaInput,
    order: DerivativeOrder,
    output: &mut GgaOutput,
    workspace: &mut EvaluationWorkspace,
) -> Result<(), LibxcRsError> {
    // 1. Validate workspace.
    // 2. Zero every caller output slot (zk, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2, …).
    // 3. For each aux in functional.auxiliaries:
    //    a. workspace.zero_scratch()
    //    b. let scratch = workspace.gga_scratch_mut()   // materialized, D-12
    //    c. Build GgaOutput pointing into scratch.zk, scratch.vrho, scratch.vsigma, …
    //    d. Dispatch by aux.meta.family:
    //       - Family::Lda  → dispatch_lda  (widens into GGA scratch? or needs LDA-into-GGA lift)
    //       - Family::Gga  → dispatch_gga
    //       - Family::Mgga → error / unreachable (GGA parent shouldn't have MGGA aux)
    //    e. For each derivative field present at this order AND present in aux's family:
    //       add_to_mix(output.field, aux.mix_coefficient, &scratch.field)
    //       — following mix_func.c:171-308 conditional fan-out (is_gga / is_mgga gates)
    Ok(())
}
```

**Critical libxc detail:** `mix_func.c` tests `aux->info->family` per aux to decide which derivative fields to accumulate. An LDA aux inside a GGA parent contributes only rho-derivatives (no vsigma, v2rhosigma, etc.); a GGA aux contributes rho+sigma derivatives; an MGGA aux contributes rho+sigma+lapl+tau derivatives. The Rust port must replicate this per-aux family gating. [VERIFIED: libxc-master/src/mix_func.c:170-308]

**Source:** [VERIFIED: libxc-master/src/mix_func.c:170-308], [VERIFIED: src/eval/mix.rs::evaluate_mixed_lda template]

### Pattern 4: Rust port of `xc_hyb_type()`

**What:** A pure-Rust, data-only function that matches libxc's classification logic byte-for-byte, operating on `&[HybridTerm]` instead of `xc_func_type*`.

**Example:**
```rust
// src/functional/hybrid.rs
use crate::model::{HybridType, HybridTermKind};
use crate::meta::HybridTerm;

/// Rust port of libxc `xc_hyb_type()` from hybrids.c:82-118.
/// Classifies a hybrid functional based on its hyb_terms array.
pub fn classify_hybrid(terms: &[HybridTerm]) -> HybridType {
    if terms.is_empty() {
        return HybridType::Semilocal;  // XC_HYB_NONE → Semilocal
    }

    if terms.len() == 1 {
        // Note: libxc uses XC_HYB_NONE == 0 both as "no hybrid terms" and
        // "GGA with screening parameter in the hyb_omega slot". This second
        // case returns XC_HYB_SEMILOCAL from xc_hyb_type. Our HybridTerm
        // enum doesn't include a "None" variant (per src/model/mod.rs:60-68),
        // so this screening case must be snapshotted distinctly — but since
        // FunctionalMeta.hybrid_terms is empty for these GGAs anyway (their
        // hyb_number_terms is 1 but hyb_type[0] == XC_HYB_NONE signals
        // semilocal), the empty-array case already covers them.
        match terms[0].kind {
            HybridTermKind::Fock       => HybridType::Hybrid,
            HybridTermKind::ErfSr      => HybridType::Cam,
            HybridTermKind::YukawaSr   => HybridType::CamYukawa,
            HybridTermKind::GaussianSr => HybridType::CamGaussian,
            HybridTermKind::Pt2        => HybridType::Mixture, // unusual single-term PT2
        }
    } else if terms.len() == 2 {
        use HybridTermKind::*;
        match (terms[0].kind, terms[1].kind) {
            (ErfSr,      Fock) => HybridType::Cam,
            (YukawaSr,   Fock) => HybridType::CamYukawa,
            (GaussianSr, Fock) => HybridType::CamGaussian,
            (Pt2,        Fock) => HybridType::DoubleHybrid,
            _                   => HybridType::Mixture,
        }
    } else {
        HybridType::Mixture
    }
}
```

**Source:** [VERIFIED: libxc-master/src/hybrids.c:82-118], [VERIFIED: libxc-master/src/xc.h:86-100]

### Anti-Patterns to Avoid

- **Re-implementing `set_ext_params_cam` etc. as a generic "propagation transform" enum.** There are ~15 named libxc helpers (util.c:100-285) each with distinct semantics. Trying to shoehorn them into a `PropagationTransform::{Copy, Linear{scale, offset}, CamStyle, LcStyle, ...}` enum is a maintenance trap. Instead: per-functional `FunctionalParams::set_ext_params` implementations handle derivation, and the xtask-emitted propagation map handles only the subset that calls `xc_func_set_ext_params_name(p->func_aux[i], ...)` — which is **already** pure `Copy` of a named parent ext_param into an aux's named ext_param (see `hyb_gga_xc_camy_b3lyp.c:51` and `mgga_c_b94.c:79-83`).

- **Putting the LDA→GGA "lifting" logic for cross-family aux in dispatch.** Libxc's `xc_mix_func` calls `xc_lda(aux, ...)` which writes only rho-derivatives into the GGA-shaped scratch; the sigma-derivatives and lapl/tau-derivatives stay zero. Our `evaluate_mixed_gga` must **not** try to "widen" an LDA aux's output — it dispatches to `dispatch_lda` with an LDA-shaped output bundle carved from the GGA scratch, leaving the sigma/lapl/tau slots untouched. The accumulation loop then only `add_to_mix`'s the fields that the aux actually wrote. [VERIFIED: libxc-master/src/mix_func.c:162-168]

- **Making `Functional: Clone` or `Copy`.** Aux recursion + `Box<dyn>` + `Box<[f64]>` means cheap-clone is impossible. Users who need a re-configured instance call `Functional::new(id, spin)` again (~μs cost; Phase 2 measurements confirm this).

- **Global mutable registry.** The registry is `&'static` data; nothing in Phase 5 mutates it. ext_params state lives on `Functional` instances.

- **Silent ext_param defaults.** D-09 forbids "set silently to libxc default if user forgot to set." If `Functional::new` returns successfully, ext_params are already at libxc defaults from `meta.ext_params[i].default_value`; users override via `set_ext_param`.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Runtime downcast from `&dyn FunctionalParams` to concrete impl | Custom type-id scheme or enum dispatch over 229 variants | `std::any::Any` + `as_any() -> &dyn Any` + `downcast_ref::<T>()` | Std-lib primitive; dispatch arms already know the expected concrete type at the match-arm level. [VERIFIED: std::any::Any] |
| Hybrid classification | Hand-written 40-line match re-derived in each query call | Port `xc_hyb_type()` once; pre-snapshot into `FunctionalMeta.hybrid_type` (D-14); query is a single field read | Libxc already solved this; verify/ test confirms the Rust port matches the snapshot which matches live FFI. |
| Propagation transforms | Custom DSL for "subtract 1 from ext_param 0 and push to aux slot 1 as name `_alpha`" | Pure-Copy table from xtask + per-functional `FunctionalParams::set_ext_params` for derivation | All libxc non-Copy transforms live in the functional's own `set_ext_params` callback; the cross-functional data flow is strictly pure-Copy by name. See §Open Questions on non-Copy exceptions. |
| Cycle detection in aux graph | DFS visited-set on every Functional::new | Static assertion at xtask time (D-17) | Aux graph is static (function of libxc version). If depth ≤ 2 is maintained, no cycles exist by construction. |
| GPU buffer management in mixed path | New GPU pathway for each aux | Reuse existing CPU-backed `EvaluationWorkspace` (Phase 3 contract) | Phase 5 is CPU-only feature per CONTEXT.md scope. GPU backends in Phase 7 will revisit. |
| Field-by-field `assert_eq!` for metadata oracle | Thousands of hand-written comparisons | Derive `PartialEq` on `FunctionalMeta` + all its field types; use `assert_eq!(rust_meta, ffi_meta)` on pretty-printed dump | Rust's `PartialEq` + `Debug` formatting gives diff-friendly output. `ExtParamSpec`, `HybridTerm`, `Reference` are `#[derive(Debug, Clone, Copy)]` in `src/meta/mod.rs`; add `PartialEq`. |
| Parsing libxc C source for metadata | Regex or AST parse of `xc_*.c` | FFI call to `xc_func_init` in xtask, snapshot the already-populated `xc_func_type` | libxc's own `_init` functions are the canonical data normalizer; reproducing them via text parsing risks subtle drift. [CITED: CONTEXT.md D-01 rejected "parse _init()"] |

**Key insight:** Phase 5 is a **refactoring + data-plumbing phase**, not a new-code phase. The temptation to hand-roll generic frameworks (propagation transforms, dispatch macros, downcast schemes) is strong. Resist: libxc has already solved each problem per-functional, and Phase 5's job is to port libxc's solutions faithfully, not to abstract over them.

## Runtime State Inventory

Phase 5 is primarily a **new-code** phase; only dispatch.rs signatures and `src/meta/generated.rs` are rewritten in place. The limited rename/refactor scope is listed below.

| Category | Items Found | Action Required |
|----------|-------------|------------------|
| Stored data | None — project has no external datastores (ChromaDB, SQLite, etc.) | None |
| Live service config | None — project has no services | None |
| OS-registered state | None — project has no daemons or scheduled tasks | None |
| Secrets / env vars | None read by any Phase 5 code path. Phase 4 oracle uses `cargo build`-time libxc path only; Phase 5 does not introduce runtime env vars | None |
| Build artifacts | `src/meta/generated.rs` (committed, 9741 lines) rewritten in place by xtask; two NEW generated files `src/meta/generated_hybrid.rs` and `src/meta/generated_propagation.rs` committed. Existing stub files (`auxiliary.rs`, `ext_param.rs`, `functional_meta.rs`, `hybrid.rs`, `library.rs`, `nlc.rs`, `reference.rs` — each 2 lines) may be deleted or populated; planner's choice (CONTEXT.md §Claude's Discretion) | Phase 5 Wave 0 task: decide stub file fate, then xtask regeneration |

**Refactor surface (non-runtime):**
- `dispatch_lda` signature change: last positional `&LdaFunctionalParams` → `&dyn FunctionalParams` (all 37 LDA dispatch arms updated + all callers: verify/tests/lda_oracle.rs, src/eval/mix.rs::evaluate_mixed_lda)
- `dispatch_gga` signature change: GGA dispatch currently has per-arm hardcoded scalars, migrates to `&dyn FunctionalParams` for all 106 arms + all callers
- `dispatch_mgga` signature change: same migration for 86 arms
- `GgaScratch` / `MggaScratch` go from `PhantomData` placeholder structs to real field structs carved from workspace scratch

**The canonical question (refactor):** After Phase 5 lands, what old code patterns remain that reference pre-Phase-5 APIs?
- Every verify/tests/*.rs that constructs `LdaFunctionalParams { alpha: x }` directly must be updated (or we preserve `LdaFunctionalParams` as one concrete `FunctionalParams` impl — planner's choice)
- No OS-level or external-system refactor work

## Common Pitfalls

### Pitfall 1: "pure Copy propagation" doesn't capture `set_ext_params_cam`

**What goes wrong:** xtask generates a propagation table assuming parent→aux ext_param flow is always `Copy`. But `set_ext_params_cam` reads 3 parent ext_params and writes them to `hyb_coeff[]`, `hyb_omega[]` — not to any aux. If we model this as "no propagation rule", the alpha/beta/omega values never reach the aux functionals that implement SR/LR separation internally.

**Why it happens:** The CAM family functionals (`hyb_gga_xc_cam_*`) don't have an aux functional whose ext_params receive alpha/beta — those values stay on the *parent's* hyb_coeff/hyb_omega arrays. The aux functionals (e.g., `GGA_X_SFAT`) do have their own `_omega` that *is* copied across, but it's a subset.

**How to avoid:** The propagation map covers ONLY the calls `xc_func_set_ext_params_name(p->func_aux[i], name, val)` that appear in per-functional `_set_ext_params` callbacks. The derived-values that update the parent's own state (`p->hyb_coeff[*]`, `p->mix_coef[*]`) are computed by the parent's `FunctionalParams::set_ext_params` method, which is per-functional Rust code. This bifurcation must be explicit in the plan.

**Warning signs:** Oracle mismatch on CAM/CAMY/CAMG hybrids where alpha/beta vary across ext_params default vs user-set values.

[VERIFIED: libxc-master/src/util.c:178-202 set_ext_params_cam], [VERIFIED: libxc-master/src/hyb_gga_xc_camy_b3lyp.c:82-100 cam_set_ext_params]

### Pitfall 2: `xtask` accidentally pulls libxc into the main build graph

**What goes wrong:** Naively adding `libxc-sys = { path = "../libxc-sys" }` to `xtask/Cargo.toml` as a regular dependency propagates the build-time cmake+bindgen into every `cargo build` of the workspace if xtask is ever a path-dep target of a workspace member.

**Why it happens:** Cargo workspaces share the dependency graph; adding `libxc-sys` as a `[dependencies]` entry in the `xtask` package makes it a transitive dep of anyone who depends on xtask.

**How to avoid:** xtask is a `[[bin]]` crate that nothing depends on. Its `libxc-sys` dependency lives in `[dependencies]` (not `[build-dependencies]`) because xtask uses libxc-sys at xtask-runtime (not at xtask-build-time). Critically, the root `Cargo.toml` must NOT list `libxc-sys` as a workspace-wide path-dep — only `xtask/Cargo.toml` and `verify/Cargo.toml` list it. The main `libxc_rs` crate stays unaware of `libxc-sys`.

**Warning signs:** `cargo build` in the main crate starts requiring cmake and libxc source; CI `cargo build` jobs break in containers without libxc toolchain.

[VERIFIED: /workspace/Cargo.toml inspects `libxc_rs` [dependencies] — no libxc-sys], [VERIFIED: /workspace/verify/Cargo.toml structure]

### Pitfall 3: Aux arity claim "1-4" is wrong — actual max is 6 in libxc 7.0.0

**What goes wrong:** CONTEXT.md D-15 describes auxiliary Vec length as "1-4 for hybrids (B3LYP = 3 aux)". An empirical grep of `libxc-master/src/*.c` for `xc_mix_init(p, N, ...)` shows N ranges from 1 to 6 (grep result: `{1, 2, 3, 4, 5, 6}`). In particular, `hyb_gga_xc_b3lyp5.c` has 5 aux, and some complex mixtures have 6.

**Why it happens:** Context-gathering estimated from memorable examples; actual libxc corpus has outliers.

**How to avoid:** No fixed-size aux buffer. `Vec<Functional>` with any length works. Just update documentation and any test fixtures that assume max-4. Confirm with xtask at snapshot time: it should log the max aux arity observed.

**Warning signs:** Array bound assertions with hard-coded 4 somewhere in the code.

[VERIFIED: `grep -oE "xc_mix_init\(p, ([0-9]+)" libxc-master/src/*.c | sort -u` → 1..=6]

### Pitfall 4: B3LYP has 4 aux (not 3)

**What goes wrong:** CONTEXT.md §decisions D-15 claims "B3LYP = 3 aux". Actual: `hyb_gga_xc_b3lyp.c` has `funcs_id[4] = {XC_LDA_X, XC_GGA_X_B88, XC_LDA_C_VWN, XC_GGA_C_LYP}` — 4 aux.

**Why it happens:** B3LYP's "3 components" (Slater, Becke88, LYP) maps to 4 aux because LDA exchange is explicitly separated from GGA exchange (to support Slater fraction).

**How to avoid:** No code consequence — `Vec<Functional>` handles any arity. Just note the doc inaccuracy.

**Warning signs:** Test case for B3LYP assuming `auxiliaries.len() == 3`.

[VERIFIED: libxc-master/src/hyb_gga_xc_b3lyp.c funcs_id[4]]

### Pitfall 5: `evaluate_mixed_gga` mixing LDA-aux output with uninitialized sigma-derivatives

**What goes wrong:** When a GGA-parent has an LDA-family aux (e.g., B3LYP's `XC_LDA_X` and `XC_LDA_C_VWN`), the aux writes only `zk`, `vrho`, `v2rho2`, … into scratch. If the naive accumulation loop unconditionally does `add_to_mix(output.vsigma, weight, scratch.vsigma)` without checking the aux's family, the uninitialized (or previously-mixed-in) `scratch.vsigma` corrupts `output.vsigma`.

**Why it happens:** The mix loop must mirror `libxc-master/src/mix_func.c:170-308`'s per-aux family gating — `is_gga(aux_family)` around the `sum_var(vsigma)` call, `is_mgga(aux_family)` + flag check around `sum_var(vtau)` etc.

**How to avoid:** In `evaluate_mixed_gga`, branch on `aux.meta.family` before accumulating family-exclusive fields. Mirror `mix_func.c` literally; don't generalize.

**Warning signs:** Oracle mismatch specifically on `vsigma`, `v2rhosigma`, `v2sigma2` for LDA-aux hybrids like B3LYP.

[VERIFIED: libxc-master/src/mix_func.c:170-308]

### Pitfall 6: `xc_hyb_type` with single-term `XC_HYB_NONE` returns `XC_HYB_SEMILOCAL`, not `XC_HYB_NONE`

**What goes wrong:** The Rust port naively translates `xc_hyb_type` as "empty terms → Semilocal", but libxc also returns `XC_HYB_SEMILOCAL` when `hyb_number_terms == 1 && hyb_type[0] == XC_HYB_NONE` — this is the "GGA with screening parameter stored in the hyb structure" case (e.g., some long-range GGAs).

**Why it happens:** libxc reuses the `hyb_` struct members to carry an `omega` for screened GGAs that are *not* hybrids. `hyb_type[0] = XC_HYB_NONE, hyb_omega[0] = ω`.

**How to avoid:** The Rust `HybridTermKind` enum currently lacks a "None" variant (src/model/mod.rs:60-68). Options: (a) add `HybridTermKind::None = 0` so the port can round-trip; or (b) have the xtask snapshot suppress the term entirely when `hyb_type[0] == XC_HYB_NONE`, leaving `hybrid_terms = &[]` → the empty-arr branch of the Rust port correctly returns `Semilocal`. Option (b) is cleaner and preserves the invariant "hybrid_terms is non-empty iff functional is actually hybrid".

**Warning signs:** Verify/ round-trip test fails for screened-GGA ids like `gga_x_ityh` family members.

[VERIFIED: libxc-master/src/hybrids.c:89-91]

### Pitfall 7: Deferred functionals (10 total) and `FunctionalParams` impl cohesion

**What goes wrong:** D-09 says deferred functionals "continue to return `UnsupportedFunctional` errors; they get `FunctionalParams` impls only to the extent needed to keep the trait cohesive." But `Functional::new` is called *before* any evaluate call, so a deferred functional still needs to successfully construct (or return `UnsupportedFunctional` at `new`-time, losing metadata queryability).

**Why it happens:** Current Phase 4 behavior: `LdaFunctional::from_id` returns `UnsupportedFunctional` for the 4 deferred LDA IDs, causing `dispatch_lda` to fail. But users might reasonably want to call `Functional::new(deferred_id, spin)` just to query `hybrid_type()` or `cam_coefficients()` — metadata works, only evaluation doesn't.

**How to avoid:** `Functional::new` succeeds for all 649 IDs (metadata exists); `evaluate_*` fails for deferred IDs. FunctionalParams impls for deferred IDs can use `NoParams` (they don't evaluate, so ext_params don't matter). The deferred-check moves into `evaluate_{lda,gga,mgga}` (or stays in `dispatch_*`).

**Warning signs:** `Functional::new(deferred_id, …).unwrap()` panics unexpectedly.

[VERIFIED: src/eval/dispatch.rs — deferred handling currently at dispatch time], [VERIFIED: CONTEXT.md §decisions D-09]

## Code Examples

### Example 1: `FunctionalParams` downcast in dispatch arm

```rust
// src/eval/dispatch.rs (post-Phase-5)
pub fn dispatch_lda(
    functional: LdaFunctional,
    input: &LdaInput,
    order: DerivativeOrder,
    output: &mut LdaOutput,
    params: &dyn FunctionalParams,   // was: &LdaFunctionalParams
    thresholds: &Thresholds,
) -> Result<(), LibxcRsError> {
    // ...validation identical to Phase 4...

    // Downcast to concrete type based on dispatch arm.
    match functional {
        LdaFunctional::LdaX => {
            let p = params.as_any().downcast_ref::<LdaXParams>()
                .ok_or(LibxcRsError::KernelLaunchFailed {
                    reason: "FunctionalParams type mismatch: LdaX expects LdaXParams".into(),
                })?;
            // p.alpha is available here
            launch_lda_x(/* ..., p.alpha, ... */)
        }
        // ... 36 more arms
    }
}
```

**Source:** [CITED: CONTEXT.md D-07/D-08], [VERIFIED: /workspace/src/eval/dispatch.rs lines 30-53 for existing struct]

### Example 2: xtask generate-metadata main loop (sketch)

```rust
// xtask/src/generate_metadata.rs (new)
use libxc_sys::{xc_func_type, xc_func_init, xc_func_end, XC_UNPOLARIZED};

pub fn generate_metadata(out_path: &Path) -> Result<()> {
    let mut entries = Vec::new();
    let mut hybrid_types = Vec::new();
    let mut propagation_rules = Vec::new();
    let mut max_aux_depth = 0;

    for id in known_functional_ids() {  // 649 total, from funcs_{lda,gga,mgga}.c enumeration
        let mut t: xc_func_type = unsafe { std::mem::zeroed() };
        let rc = unsafe { xc_func_init(&mut t, id as i32, XC_UNPOLARIZED) };
        if rc != 0 { bail!("xc_func_init failed for id {id}"); }

        // SAFETY: t is populated by libxc per xc.h contract
        let meta = unsafe { FunctionalMeta {
            id: FunctionalId(id),
            name: cstr_to_static(&(*t.info).name),
            kind: map_xc_kind((*t.info).kind),
            family: map_xc_family((*t.info).family),
            flags: FunctionalFlags::from_bits_retain((*t.info).flags as u32),
            references: snapshot_references(&(*t.info).refs),
            ext_params: snapshot_ext_params(&(*t.info).ext_params),
            default_density_threshold: (*t.info).dens_threshold,
            auxiliaries: snapshot_aux(t.func_aux, t.n_func_aux, t.mix_coef),
            hybrid_terms: snapshot_hyb(t.hyb_type, t.hyb_coeff, t.hyb_omega, t.hyb_number_terms),
            nlc_params: if t.nlc_b != 0.0 || t.nlc_C != 0.0 {
                Some((t.nlc_b, t.nlc_C))
            } else { None },
            max_order: infer_max_order((*t.info).flags as u32),
        }};

        // Rust port of xc_hyb_type for cross-check
        let rust_port = classify_hybrid(&meta.hybrid_terms);
        let ffi_class = map_xc_hyb_type(unsafe { xc_hyb_type(&t) });
        assert_eq!(rust_port, ffi_class,
            "xc_hyb_type mismatch at id {id} ({name})", name = meta.name);

        // Propagation rule detection:
        // Re-invoke the functional's set_ext_params with DEFAULT values, then
        // inspect each t.func_aux[i]->ext_params array. Compare to parent's
        // meta.ext_params.default_value entries — any aux slot whose value
        // equals a parent slot's value is a "Copy" rule candidate. Edge case:
        // multiple identical default values → heuristic ambiguity. Fallback:
        // also try a PERTURBED ext_params (e.g., {x+1.0, y+2.0, …}), then
        // parent-idx-for-aux-idx is uniquely determined.

        // Aux depth check
        let depth = aux_depth(&meta);
        max_aux_depth = max_aux_depth.max(depth);

        unsafe { xc_func_end(&mut t); }
        entries.push(meta);
    }

    assert!(max_aux_depth <= 2, "aux depth {max_aux_depth} exceeds D-17 invariant");

    emit_generated_rs(&entries, out_path)?;
    emit_generated_hybrid_rs(&hybrid_types, out_path)?;
    emit_generated_propagation_rs(&propagation_rules, out_path)?;
    Ok(())
}
```

**Source:** [VERIFIED: libxc-master/src/xc.h:327-360 xc_func_type layout], [VERIFIED: libxc-master/src/hybrids.c:82-118]

### Example 3: `GgaScratch` materialized (mirrors existing `LdaScratch` pattern)

```rust
// src/eval/workspace.rs (post-Phase-5)
pub struct GgaScratch<'a> {
    // Order 0
    pub zk: &'a mut [f64],
    // Order 1
    pub vrho: &'a mut [f64],
    pub vsigma: &'a mut [f64],
    // Order 2
    pub v2rho2: &'a mut [f64],
    pub v2rhosigma: &'a mut [f64],
    pub v2sigma2: &'a mut [f64],
    // Order 3
    pub v3rho3: &'a mut [f64],
    pub v3rho2sigma: &'a mut [f64],
    pub v3rhosigma2: &'a mut [f64],
    pub v3sigma3: &'a mut [f64],
    // Order 4
    pub v4rho4: &'a mut [f64],
    pub v4rho3sigma: &'a mut [f64],
    pub v4rho2sigma2: &'a mut [f64],
    pub v4rhosigma3: &'a mut [f64],
    pub v4sigma4: &'a mut [f64],
}

impl EvaluationWorkspace {
    pub fn gga_scratch_mut(&mut self) -> GgaScratch<'_> {
        // Use identical split_at_mut chain idiom as lda_scratch_mut.
        // Offsets computed from Dimensions::gga() fields, but walked through
        // the MGGA-ordered buffer (skipping vlapl/vtau slots since they're
        // not GGA-applicable).
        //
        // The workspace buffer was sized for MGGA superset (Phase 3 D-12),
        // so all GGA fields fit within it at their MGGA-absolute offsets.
        // We compute each field's MGGA-absolute offset, then thread
        // split_at_mut through those 15 offsets.
        //
        // Full implementation: ~70 lines mirroring lda_scratch_mut, with
        // 10 more "skip" splits for the vlapl/vtau/v*tau fields between
        // GGA-applicable splits.
        todo!("implement via same split_at_mut chain pattern as lda_scratch_mut")
    }
}
```

**Source:** [VERIFIED: src/eval/workspace.rs:197-239 lda_scratch_mut reference], [VERIFIED: libxc-master/src/xc.h:210-223 xc_gga_out_params fields]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Phase 4 hardcoded libxc defaults at dispatch call site (`LdaFunctionalParams::default() -> { alpha: 1.0 }` baked into verify tests) | Phase 5 ext_params flow: `Functional::new` reads `meta.ext_params[i].default_value` → stores in `Option<Box<[f64]>>` → passes through `&dyn FunctionalParams` → dispatch downcasts | Phase 5 | Verify tests must continue to pass; dispatch signature change is non-breaking for verify/ since verify calls `dispatch_*` with `LdaFunctionalParams::default()` equivalent |
| `GgaScratch` / `MggaScratch` as `PhantomData` placeholders | Real structs with split_at_mut-carved mutable slices over workspace scratch | Phase 5 D-12 | Enables B3LYP and every hybrid GGA/MGGA to evaluate correctly (currently they panic at `todo!("GGA scratch accessor not yet implemented -- Phase 4")`) |
| `FunctionalMeta` with empty `auxiliaries: &[]`, `hybrid_terms: &[]`, `nlc_params: None`, `references: &[]` | Fully populated from libxc FFI snapshot via xtask | Phase 5 D-01/D-02 | HYB-01..04 become implementable; doc-surface gains references |
| No `Functional` struct; dispatch called directly via free fns | `Functional` struct owns lifecycle; dispatch methods delegate to free fns (D-11) | Phase 5 | Two-tier API matching libxc (low-level `xc_lda()` + high-level `xc_func_init` + `evaluate_lda`) |
| Verify/build.rs owns cmake+bindgen of libxc (Phase 1) | Factored into `libxc-sys` workspace crate used by both `verify/` and `xtask/` (Phase 5 D-03) | Phase 5 | One cmake invocation; xtask can now snapshot libxc metadata |

**Deprecated/outdated (nothing to remove — Phase 5 is additive):**
- N/A: no existing APIs are deprecated. `LdaFunctionalParams` struct either becomes one concrete `FunctionalParams` impl or is renamed `LdaXParams`; the distinction is planner's discretion.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Max aux graph depth in libxc 7.0.0 is ≤ 2 (D-17 invariant) | §Eager Aux Construction, Pattern 2 | If a depth-3 functional exists, xtask panics at generate time. CONTEXT.md D-17 explicitly handles this as "fail loud, forces conscious decision." But the claim itself is `[ASSUMED]` — no empirical traversal has been done yet. **Mitigation:** Plan 05-01 must include a Wave-0 task that literally runs an aux-depth BFS on a partial FFI snapshot as an early-warning probe before committing to full metadata regeneration. |
| A2 | All libxc parent→aux ext_params flow is pure `Copy` (by name) | §Pattern 1, Pitfall 1 | If a non-Copy transform exists (e.g., `aux[0]._omega = 2 * parent[3]`), the propagation map is insufficient. **Mitigation:** xtask must emit an error on non-Copy detection (via the perturbation-based detection in Example 2); non-Copy cases require hand-written per-functional Rust. CONTEXT.md §deferred permits this scope-out. |
| A3 | `FunctionalMeta` fields (`Reference`, `ExtParamSpec`, `HybridTerm`) are all `Clone, Copy, PartialEq`-derivable for field-by-field round-trip comparison | §Validation Architecture, verify/tests/metadata_oracle.rs | `src/meta/mod.rs` shows `#[derive(Debug, Clone, Copy)]` on all three. `PartialEq` is absent but can be added — requires `&'static str` fields to use by-reference equality which is the Rust default for slice comparison. **Mitigation:** derive `PartialEq` in Wave 0; if a field type resists, write a manual impl. |
| A4 | `cubecl 0.9.0` dispatch signatures don't care about function-pointer identity when a trait object replaces a value parameter | §dispatch migration | Dispatch is plain Rust functions calling `cubecl::launch_unchecked<Runtime>`; the signature of the enclosing `dispatch_*` function is not seen by cubecl macros. Migration is a pure-Rust refactor. [VERIFIED: /workspace/src/eval/dispatch.rs — dispatch_lda is ordinary `fn`, cubecl concern is internal to kernel modules] |
| A5 | `Send + Sync` auto-derivation on `Functional` works given all fields are Send+Sync | §D-13 | `&'static FunctionalMeta`: Sync (all-str+Copy fields). `Box<[f64]>`: Send+Sync. `Box<dyn FunctionalParams + Send + Sync>`: Send+Sync explicitly. `Vec<Functional>`: recursive. **Risk:** only if a future FunctionalParams impl holds `Rc`/`Cell`/`RefCell` — enforce via `+ Send + Sync` in the trait bound, which D-13 already mandates. |
| A6 | 229 is the correct "compiled functional" count (37 LDA + 106 GGA + 86 MGGA per D-09 / CONTEXT.md) | §Validation Architecture test counts | CONTEXT.md D-09 states these numbers. Cross-check: Phase 4 HANDOFF mentions 105 routable GGAs and 25 wired MGGAs with ~60 more via zero-scalar macro. Discrepancy is likely due to counting methodology (routable vs actually-wired). **Mitigation:** Wave-0 count audit confirms actual FunctionalParams impl surface. |
| A7 | Snapshotting at `XC_UNPOLARIZED` is sufficient — polarized invocation produces no additional metadata | §xtask generate-metadata | libxc does distinguish `nspin` in `dim` fields, but `ext_params`, `hyb_*`, `func_aux`, and all `info->*` metadata are spin-independent. [VERIFIED: libxc-master/src/xc.h:327 `xc_func_type.nspin` is separate from `info`]. Metadata snapshot takes UNPOLARIZED safely. |
| A8 | Aux arity distribution is 1..=6, with B3LYP = 4 aux | §Pitfall 4, CONTEXT.md revision | Empirically verified by grep on libxc-master. [VERIFIED: grep mix_init arities], [VERIFIED: hyb_gga_xc_b3lyp.c funcs_id[4]]. CONTEXT.md's "1-4" and "B3LYP = 3 aux" wording should be revised — no code impact, Vec handles any arity. |

## Open Questions

1. **Does any libxc functional have a non-Copy parent→aux ext_param propagation?**
   - What we know: `hyb_gga_xc_camy_b3lyp` and `mgga_c_b94_hyb` and `hyb_gga_xc_wb2plyp` all use `xc_func_set_ext_params_name(p->func_aux[i], "_name", value)` — pure Copy by name.
   - What's unclear: Haven't audited all 147 hybrid + mixed functionals. Empirical check is a small xtask prototype.
   - Recommendation: Plan 05-01 includes a scan task: grep `xc_func_set_ext_params_name(p->func_aux` and `xc_func_set_ext_params(p->func_aux` across libxc-master/src/*.c, audit each occurrence. If any non-Copy cases exist, either (a) scope them out per CONTEXT.md §deferred "Non-Copy propagation transforms", or (b) hand-write per-functional Rust.

2. **Should `dispatch_*` remain `pub` in `libxc_rs::eval` or move to `pub(crate)`?**
   - What we know: CONTEXT.md §Claude's Discretion leaves this open. Currently `pub` and re-exported at crate root (`/workspace/src/lib.rs:31`). Verify/tests/*.rs calls them directly.
   - What's unclear: Preserving `pub` maintains verify/ compatibility but creates two public ways to evaluate (free `dispatch_*` vs `Functional::evaluate_*`), which can confuse users.
   - Recommendation: Keep `pub` through Phase 5 (verify tests matter), downgrade to `pub(crate)` in Phase 6 when `BatchEvaluator` offers the canonical API and verify tests can be refactored to use `Functional::evaluate_*`.

3. **Single `params.rs` file with 229 impls, or per-family split?**
   - What we know: CONTEXT.md §Claude's Discretion marks this planner's choice.
   - What's unclear: Size estimate — each impl is 5-20 lines (struct + `impl FunctionalParams`), so 229 × 15 ≈ 3500 lines in one file. Compiles fine but painful to navigate.
   - Recommendation: Per-family split (`params_lda.rs`, `params_gga.rs`, `params_mgga.rs`). Each ~1000-1500 lines, family-scoped semantic locality.

4. **Should `FunctionalParams` impls be xtask-generated?**
   - What we know: CONTEXT.md §Claude's Discretion. Metadata is already xtask-generated, so emitting `impl FunctionalParams for LdaXParams { … }` from the same pass is possible.
   - What's unclear: The derived-parameter formulas differ per functional in ways that can't all be expressed as table-driven code (see Pitfall 1 — `set_ext_params_cam` is formula-driven, not table-driven).
   - Recommendation: **Hand-write the derivation logic, xtask-emit the trivial Copy-only impls.** Functionals with zero ext_params get `NoParams`; functionals with ext_params + pure-Copy `set_ext_params_cpy` can be generated; the ~20 functionals with derivation formulas (CAM/CAMY/CAMG/LC/LCY and some MGGA hybrids) are hand-written. Mixes macro-generation with precision where needed.

5. **How does the snapshot round-trip for `FunctionalMeta.flags` handle unknown libxc flag bits?**
   - What we know: `FunctionalFlags::from_bits_retain` preserves unknown bits (bitflags 2.x `retain`-family constructors). [VERIFIED: bitflags 2.10 docs]
   - What's unclear: Does Phase 4's `FunctionalFlags` enum cover all libxc flags, including `MAPLE2C_FLAGS` bundle?
   - Recommendation: Wave-0 audit: enumerate flags emitted by xtask, diff against `src/model/mod.rs::FunctionalFlags` bit definitions. Extend enum if missing.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Rust toolchain (rustc) | all | ✓ | 1.85+ (edition 2024) | — |
| cargo | all | ✓ | bundled with rustc | — |
| cmake | `libxc-sys/build.rs` (new), `verify/build.rs` (existing) | ✓ (already used by `verify/`) | 0.1.58 cargo-cmake crate | — |
| bindgen | `libxc-sys/build.rs`, `verify/build.rs` | ✓ (already used) | 0.72.1 | — |
| libxc-master vendored source | `libxc-sys` compilation | ✓ | 7.0.0 vendored in `/workspace/libxc-master/` | — |
| C compiler (for libxc build) | `libxc-sys` cmake | Assumed ✓ | cc or gcc | — |
| anyhow | xtask (existing) | ✓ | 1.0.100 | — |
| approx (test-time) | verify/tests/metadata_oracle.rs if scalar compares used | ✓ | 0.5.1 in verify/ dev-deps | — |

**Missing dependencies with no fallback:** None. The `verify/` crate already builds libxc as part of `cargo test -p libxc_rs-verify`; Phase 5 reuses that build pipeline.

**Missing dependencies with fallback:** None.

**No new runtime dependencies** for the main `libxc_rs` crate — all Phase 5 work stays on build-time or dev-time tiers.

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust built-in `#[test]` + `approx 0.5.1` for scalar comparisons |
| Config file | `/workspace/verify/Cargo.toml` (dev-deps), individual `#[cfg(test)]` modules |
| Quick run command | `cargo test -p libxc_rs --lib` (unit tests, no FFI) |
| Full suite command | `cargo test --workspace` |
| Oracle round-trip | `cargo test -p libxc_rs-verify --test metadata_oracle` |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| FUNC-01 | `Functional::new(id, spin)` initializes all state | unit | `cargo test -p libxc_rs --lib functional::lifecycle::tests` | ❌ Wave 0 |
| FUNC-01 | `Functional::new` returns error for unknown id | unit | `cargo test -p libxc_rs --lib functional::lifecycle::tests::new_unknown_id` | ❌ Wave 0 |
| FUNC-01 | Dimensions correct for each family/spin | unit | `cargo test -p libxc_rs --lib functional::lifecycle::tests::dims_by_family` | ❌ Wave 0 |
| FUNC-01 | Default ext_params match `meta.ext_params[i].default_value` | unit | `cargo test -p libxc_rs --lib functional::lifecycle::tests::default_ext_params_applied` | ❌ Wave 0 |
| FUNC-02 | Set/get ext_param by name | unit | `cargo test -p libxc_rs --lib functional::config::tests::set_get_by_name` | ❌ Wave 0 |
| FUNC-02 | Set/get ext_param by index | unit | `cargo test -p libxc_rs --lib functional::config::tests::set_get_by_index` | ❌ Wave 0 |
| FUNC-02 | Bulk `set_ext_params` validates length | unit | `cargo test -p libxc_rs --lib functional::config::tests::set_all_length_mismatch` | ❌ Wave 0 |
| FUNC-02 | `ExtParamNotFound` on unknown name | unit | `cargo test -p libxc_rs --lib functional::config::tests::unknown_name_errors` | ❌ Wave 0 |
| FUNC-03 | `set_density_threshold` updates state | unit | `cargo test -p libxc_rs --lib functional::config::tests::threshold_setters` | ❌ Wave 0 |
| FUNC-04 | B3LYP has 4 auxiliaries (LDA_X, GGA_X_B88, LDA_C_VWN, GGA_C_LYP) | unit | `cargo test -p libxc_rs --lib functional::lifecycle::tests::b3lyp_auxiliaries_correct` | ❌ Wave 0 |
| FUNC-04 | mgga_c_b94_hyb has 2 auxiliaries (MGGA_X_BR89, MGGA_C_B94) | unit | `cargo test -p libxc_rs --lib functional::lifecycle::tests::b94_hyb_auxiliaries` | ❌ Wave 0 |
| FUNC-04 | All hybrid IDs have `auxiliaries().len() >= 1` | unit (loop over registry) | `cargo test -p libxc_rs --lib functional::lifecycle::tests::hybrid_ids_have_auxiliaries` | ❌ Wave 0 |
| FUNC-04 | Aux construction depth ≤ 2 | unit | `cargo test -p libxc_rs --lib functional::lifecycle::tests::aux_depth_bounded` | ❌ Wave 0 |
| FUNC-04 | Mixed evaluation against libxc oracle (B3LYP GGA Vxc) | integration | `cargo test -p libxc_rs-verify --test mixed_oracle -- b3lyp_gga_vxc` | ❌ Wave 0 |
| FUNC-04 | Mixed evaluation for mgga_c_b94_hyb (MGGA Vxc) | integration | `cargo test -p libxc_rs-verify --test mixed_oracle -- b94_hyb_mgga_vxc` | ❌ Wave 0 |
| FUNC-04 | CAM-B3LYP evaluation with varying ext_params | integration | `cargo test -p libxc_rs-verify --test mixed_oracle -- cam_b3lyp_ext_param_sweep` | ❌ Wave 0 |
| FUNC-05 | `FunctionalParams::raw_ext_params()` round-trips default values | unit (loop over 229 functionals) | `cargo test -p libxc_rs --lib functional::params::tests::default_params_round_trip` | ❌ Wave 0 |
| FUNC-05 | `FunctionalParams::set_ext_params` triggers derivation | unit (per-functional subset) | `cargo test -p libxc_rs --lib functional::params::tests::setting_ext_triggers_derivation` | ❌ Wave 0 |
| FUNC-05 | Downcast via `as_any().downcast_ref::<T>()` succeeds in dispatch | unit | `cargo test -p libxc_rs --lib eval::dispatch::tests::downcast_succeeds` | ❌ Wave 0 |
| FUNC-05 | `NoParams` is used for zero-ext_param functionals | unit | `cargo test -p libxc_rs --lib functional::params::tests::no_params_for_zero_spec` | ❌ Wave 0 |
| FUNC-06 | Drop doesn't panic on any hybrid | unit | `cargo test -p libxc_rs --lib functional::lifecycle::tests::drop_hybrids_ok` | ❌ Wave 0 |
| FUNC-06 | No leaks per `cargo test` valgrind run (optional, not automated) | manual | `valgrind --leak-check=full cargo test --release` | (manual) |
| HYB-01 | `hybrid_type()` == snapshotted value for all 649 IDs | unit (loop) | `cargo test -p libxc_rs --lib functional::hybrid::tests::rust_port_matches_snapshot` | ❌ Wave 0 |
| HYB-01 | Rust port matches live FFI for all 649 IDs | integration | `cargo test -p libxc_rs-verify --test hybrid_type_oracle` | ❌ Wave 0 |
| HYB-02 | `cam_coefficients()` for CAM-B3LYP matches libxc | integration | `cargo test -p libxc_rs-verify --test hybrid_oracle -- cam_b3lyp_coef` | ❌ Wave 0 |
| HYB-02 | `exx_coefficient()` for B3LYP == 0.20 | unit | `cargo test -p libxc_rs --lib functional::hybrid::tests::b3lyp_exx_020` | ❌ Wave 0 |
| HYB-02 | `cam_coefficients()` returns None for non-CAM functionals | unit | `cargo test -p libxc_rs --lib functional::hybrid::tests::non_cam_returns_none` | ❌ Wave 0 |
| HYB-03 | `nlc_coefficients()` for `gga_xc_vv10` matches libxc | integration | `cargo test -p libxc_rs-verify --test hybrid_oracle -- vv10_nlc` | ❌ Wave 0 |
| HYB-03 | `nlc_coefficients()` returns None for non-NLC functionals | unit | `cargo test -p libxc_rs --lib functional::hybrid::tests::non_nlc_returns_none` | ❌ Wave 0 |
| HYB-04 | `auxiliary_functionals()` returns iterator of (id, weight) | unit | `cargo test -p libxc_rs --lib functional::hybrid::tests::aux_iter_ids_weights` | ❌ Wave 0 |
| HYB-04 | Aux IDs match libxc's `xc_aux_func_ids` output for all hybrid IDs | integration | `cargo test -p libxc_rs-verify --test metadata_oracle -- aux_ids_match` | ❌ Wave 0 |
| — | Metadata round-trip: every `FunctionalMeta` field == FFI snapshot for every id | integration (the big one) | `cargo test -p libxc_rs-verify --test metadata_oracle` | ❌ Wave 0 |
| — | Dispatch signature migration compiles; all existing oracle tests still pass | integration (regression) | `cargo test --workspace` | ✅ existing |

### Sampling Rate

- **Per task commit (< 5 seconds):** `cargo test -p libxc_rs --lib functional` — unit tests on the Functional handle, FunctionalParams trait, hybrid queries. No FFI compilation. Fast enough for every commit.
- **Per wave merge (< 2 minutes):** `cargo test -p libxc_rs --lib` (all unit tests) + `cargo test -p libxc_rs-verify --test metadata_oracle --test hybrid_type_oracle` (FFI round-trip). Ensures metadata hasn't drifted and type classification matches.
- **Per phase gate (< 10 minutes):** Full workspace `cargo test --workspace --release` — runs all 10,312-ish kernel oracle tests plus Phase 5 new tests. Must be green before `/gsd-verify-work`.

### Oracle Comparison Dimensions

For the round-trip test `verify/tests/metadata_oracle.rs` (D-04):

| Dimension | Comparison | Tolerance |
|-----------|------------|-----------|
| `id` | exact u16 | 0 |
| `name` | `&'static str` equality | exact |
| `kind` | enum discriminant | exact |
| `family` | enum discriminant | exact |
| `flags` | bitflags bitwise == | exact |
| `default_density_threshold` | f64 | exact (both sides derive from same libxc constant) |
| `references[i].{citation,doi,bibtex,key}` | `&'static str` | exact — xtask copies C strings verbatim |
| `ext_params[i].{name,description,default_value}` | str + f64 | exact (libxc default values are fixed constants) |
| `auxiliaries[i].{id,weight}` | (FunctionalId, f64) | exact |
| `hybrid_terms[i].{kind,coefficient,omega}` | (enum, f64, f64) | exact |
| `hybrid_type` | enum discriminant | exact (both from `xc_hyb_type`) |
| `nlc_params` | Option<(f64, f64)> | exact |
| `max_order` | DerivativeOrder | exact (derived from flags) |

**Sampling strategy — the key question for 229 FunctionalParams impls:**

Hand-writing 229 dedicated `#[test]` functions is infeasible. CONTEXT.md §decisions mandates "all 229 compiled functionals get real ext_params plumbing" but doesn't say each gets a hand-written test. The right approach is **parametric oracle testing with built-in exhaustive sampling**:

1. **Default-params oracle sweep (automatic, covers all 229):** Unit test `for id in all_functional_ids() { let f = Functional::new(id, Unpolarized)?; let p = f.params.raw_ext_params(); assert_eq!(p.len(), meta.ext_params.len()); for (i, spec) in meta.ext_params.iter().enumerate() { assert_eq!(p[i], spec.default_value); } }` — one test, 229 assertions.

2. **Perturbation oracle sweep (automatic, covers ext_params-bearing subset):** For each ext_params-bearing functional, pick a pseudo-random non-default perturbation (seeded RNG with id-hash seed for determinism). Call `f.set_ext_param_by_index(0, perturbed_value)`. Evaluate against libxc oracle (which also has its ext_params set identically). Compare within 1e-12. This is a single parametric test iterating ext_params-bearing ids.

3. **Hand-written targeted tests for known-tricky functionals:** CAM-B3LYP (non-trivial cam_set_ext_params), B3LYP (4-aux), mgga_c_b94_hyb (MGGA aux propagation), HSE (range-sep), wB97X (range-sep+mix), PBE0 (simple hybrid). ~10-15 hand-written integration tests covering different propagation patterns.

This **three-tier sampling** (exhaustive defaults + parametric perturbation + targeted cases) gives full 229-coverage with ~5 test functions, not 229.

### Wave 0 Gaps

- [ ] `verify/tests/metadata_oracle.rs` — covers all 649-ID field round-trip; uses FFI and Rust registry, expects full equality. (Core D-04 test)
- [ ] `verify/tests/hybrid_type_oracle.rs` — three-way compare: Rust port `classify_hybrid` vs snapshotted `meta.hybrid_type` vs live `xc_hyb_type(t)`. (HYB-01 defense-in-depth per D-14)
- [ ] `verify/tests/mixed_oracle.rs` — B3LYP, CAM-B3LYP, HSE, mgga_c_b94_hyb, wB97X integration tests via `Functional::evaluate_*`. (FUNC-04 + EVAL-05)
- [ ] `verify/tests/hybrid_oracle.rs` — CAM / NLC coefficient queries for each hybrid family member. (HYB-02, HYB-03)
- [ ] `src/functional/lifecycle.rs` tests module — construction, Drop, aux-tree shape
- [ ] `src/functional/config.rs` tests module — threshold setters, ext_param get/set by name/index, error cases
- [ ] `src/functional/params.rs` tests module — NoParams, downcast, default round-trip, perturbation
- [ ] `src/functional/hybrid.rs` tests module — `classify_hybrid` matches snapshot on all 649 IDs (loops registry, compares vs generated_hybrid.rs)
- [ ] Extended `src/eval/workspace.rs` tests — `gga_scratch_mut`/`mgga_scratch_mut` produce correct-length slices for both spin modes (mirror existing `lda_scratch_*` tests)
- [ ] Extended `src/eval/mix.rs` tests — `evaluate_mixed_gga` / `evaluate_mixed_mgga` basic weight=1.0 equivalence to `dispatch_*` (mirror existing `mixed_single_aux_weight_1_matches_dispatch`)
- [ ] Framework install: none needed — `approx` already in verify/ dev-deps, `#[test]` is built-in

## Security Domain

Phase 5 operates on static registry data and floating-point numerics — classic "compute-bound library" threat profile. No authentication, no sessions, no network I/O, no user-authored strings reaching eval. Security considerations are narrow.

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | N/A (library, no auth) |
| V3 Session Management | no | N/A |
| V4 Access Control | no | N/A |
| V5 Input Validation | **yes** | Typed Rust enums (`FunctionalId`, `Spin`, `DerivativeOrder`) constrain inputs at the type system. Buffer sizes validated against `Dimensions` (Phase 3 IO-01). `set_ext_params(&mut self, vals: &[f64])` validates length. NaN/Inf in ext_params values are allowed — libxc tolerates them; numerical thresholding handles extreme values. |
| V6 Cryptography | no | N/A (no crypto) |
| V10 Malicious Code | no | N/A (no user-provided code execution) |
| V11 Business Logic | minor | Double-application of ext_params (e.g., calling `set_ext_param` twice with same value) must be idempotent → covered by `raw_ext_params()` round-trip test. |
| V12 Files and Resources | minor | xtask writes committed generated files; never writes to user paths at library runtime. |
| V14 Configuration | minor | Thresholds are configurable per-instance; defaults are libxc-standard. |

### Known Threat Patterns for Rust scientific-library stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Panic in dispatch downcast (wrong `FunctionalParams` type paired with wrong dispatch arm) | Denial of Service (library consumer crashes) | Return `LibxcRsError::KernelLaunchFailed { reason: "FunctionalParams type mismatch" }` — never `unwrap` or `expect` on `downcast_ref`. Treat as impossible-with-correct-code but don't panic at runtime. |
| Unsafe code in `libxc-sys` FFI callback could corrupt memory | Tampering | Confine `unsafe` to the `libxc-sys` build artifact + the xtask snapshotting logic. Main `libxc_rs` crate imports no `libxc-sys` symbols. BUILD-04 invariant preserved: `unsafe` lives only in `kernel/launch.rs` + `compat/` (Phase 6). xtask's `unsafe` is explicitly scoped to the `xc_func_init`/`xc_func_end` loop. |
| Metadata drift between libxc version and committed Rust registry | Tampering via stale snapshot | D-04 round-trip test catches drift on every `cargo test -p libxc_rs-verify`. If libxc changes ABI, verify CI fails loud. |
| NaN/Inf in ext_params → kernel numerical blow-up | Denial of Service (caller gets NaN back) | Pass-through. Phase 3 EVAL-03 contract: "evaluation is infallible after input validation passes." NaN in ext_params produces NaN in output; caller handles. Not a crash. |
| Registry lookup by untrusted u16 id | Information Disclosure (none — registry is public data) | `lookup_by_id` returns `UnknownFunctionalId` error for invalid ids; no out-of-bounds possible (`by_id::REGISTRY_BY_ID.get(id as usize)`). |
| Generated file injection via malicious libxc patch | Tampering (supply-chain) | libxc-master/ is a vendored copy, reviewed at pull time. No dynamic download. `cargo xtask generate-metadata` runs against known source. |

## Project Constraints (from CLAUDE.md)

These directives MUST be honored by all Phase 5 plans:

1. **Pure Rust + CubeCL 0.9.0; no C/Fortran in production path** → `libxc_rs` main crate must not link libxc. `libxc-sys` is isolated in the xtask + verify workspace members only. [CLAUDE.md:Constraints]
2. **f64 only; energy relative error ≤ 10^-12 vs libxc oracle** → All ext_params and derived values use `f64`. `FunctionalParams` trait uses `&[f64]`. No `f32` fallback. [CLAUDE.md:Constraints]
3. **thiserror 2.0 at library boundary** → New error variants extend `LibxcRsError` via `#[derive(thiserror::Error)]`. No `anyhow` in the public API. [CLAUDE.md:Constraints, /workspace/src/error/mod.rs]
4. **Operation order preserved from maple2c** → Phase 5 doesn't touch `#[cube]` kernels; this constraint continues to hold. [CLAUDE.md:Constraints]
5. **GPU precision: no silent f32 fallback** → N/A for Phase 5 (CPU-only via `cubecl/cpu` feature). Reaffirm in Phase 7.
6. **Edition 2024, MSRV 1.85+** → Already set in workspace root.
7. **`cargo xtask generate-registry` output committed** (Phase 1 D-04) → Phase 5 extends this pattern with `generate-metadata` writing to the same committed file.
8. **BUILD-04: no unsafe outside compat/, kernel/launch.rs, GPU buffer management** → `libxc-sys` FFI unsafe is in a separate workspace member, not in `src/`; xtask unsafe is in `xtask/src/`; neither violates BUILD-04 which applies to the `libxc_rs` main crate.
9. **BUILD-05: no runtime C/Fortran FFI dependency in production library** → Enforced by confining `libxc-sys` to `xtask/Cargo.toml` and `verify/Cargo.toml`. Main crate's `Cargo.toml` has no `libxc-sys` entry.
10. **EVAL-04: non-mixed functionals zero heap allocation in evaluation hot path** → Preserved by D-06 (`Option<Box<[f64]>>` → None for zero-ext_param functionals) and D-07 (`&dyn FunctionalParams` is a fat pointer passed by reference, not allocated per-call).

## Sources

### Primary (HIGH confidence)

- **libxc-master source tree (vendored, Phase 5 authority):**
  - `libxc-master/src/hybrids.c:11-157` — `xc_hyb_init`, `xc_hyb_type`, `xc_hyb_exx_coef`, `xc_hyb_cam_coef` authoritative semantics
  - `libxc-master/src/mix_func.c:15-333` — `xc_mix_init`, `xc_mix_func` accumulation loop with per-aux family gating, `xc_num_aux_funcs`, `xc_aux_func_ids`, `xc_aux_func_weights`
  - `libxc-master/src/util.c:100-285` — `set_ext_params_cpy` / `_cam` / `_camy` / `_cam_sr` / `_lc` / `_lcy` / `_omega` / `_exx` and `_cpy_*` variants — canonical set of propagation helpers
  - `libxc-master/src/util.h:312-326` — declarations of propagation helpers
  - `libxc-master/src/xc.h:86-100, 171-360` — `XC_HYB_*` constants, `xc_func_type`/`xc_func_info_type` struct layouts, `func_reference_type`, `func_params_type`
  - `libxc-master/src/hyb_gga_xc_camy_b3lyp.c:35-100` — canonical CAM-with-aux propagation example
  - `libxc-master/src/hyb_gga_xc_b2plyp.c:24-120` — 2-3 term double-hybrid with PT2 + wB2PLYP aux propagation
  - `libxc-master/src/mgga_c_b94.c:54-106` — MGGA 2-aux hybrid with multi-aux name-based propagation (`_at`, `_gamma`, `_css`, `_cab`)
  - `libxc-master/src/hyb_gga_xc_b3lyp.c` — 4-aux hybrid example (contrary to CONTEXT.md "3 aux")

- **libxc_rs existing Rust code (unmodified reference):**
  - `src/eval/mix.rs:1-184` — `evaluate_mixed_lda` + `add_to_mix` templates
  - `src/eval/workspace.rs:63-256` — `EvaluationWorkspace`, `LdaScratch` split_at_mut pattern, `LdaFieldOffsets`
  - `src/eval/dispatch.rs:30-120` — current `LdaFunctionalParams` + signature
  - `src/meta/mod.rs:1-53` — `FunctionalMeta` struct layout, current field set
  - `src/meta/generated.rs` — 649 functional metadata skeletons
  - `src/model/mod.rs:1-120` — `HybridType`, `HybridTermKind`, `FunctionalId` enums
  - `src/registry/mod.rs:1-77` — `lookup_by_id`, `lookup_by_name`, `all_functional_ids` iterator
  - `src/error/mod.rs:1-90` — `LibxcRsError` current variants
  - `src/lib.rs:1-31` — public re-export surface
  - `verify/build.rs` — current cmake+bindgen logic to be factored
  - `xtask/src/main.rs:1-40` — existing `generate-registry` subcommand scaffold

- **libxc_rs design doc (primary spec, /workspace/docs/design/libxc_rs_detailed_design.md):**
  - §5.4 Functional Lifecycle — API mapping
  - §5.5 Threshold Configuration
  - §5.6 External Parameters
  - §5.7 Evaluation Functions (35 C → 3 Rust)
  - §5.8 Hybrid and Auxiliary
  - §6.8 Functional Runtime State struct
  - §6.9 Reusable Buffer Strategy
  - §9.11 / 9.12 `func/` + `hybrid/` module responsibilities
  - §10.1 / 10.2 / 10.3 Initialization + LDA Eval + Mixed Eval flows
  - §15 Error variants
  - §17 Oracle verification plan

### Secondary (MEDIUM confidence)

- **CONTEXT.md D-01..D-17** — user-locked decisions; treated as inputs, not independently verified
- **ROADMAP.md Phase 5 goal + success criteria** — used to cross-check coverage
- **Phase 4 CONTEXT.md / HANDOFF** — transitive (229-compiled counts, dispatch hardcoded defaults that Phase 5 undoes)

### Tertiary (LOW — flagged for verification during Wave 0)

- **Aux arity distribution (1..=6)** via grep of libxc-master — verified experimentally, but full distribution per-id not enumerated
- **229 compiled functional count (37 LDA + 106 GGA + 86 MGGA)** from CONTEXT.md D-09 — needs Wave-0 audit against actual `FunctionalParams` impl surface

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — all dependencies already in workspace, no new crates, version-verified against `Cargo.toml` files
- Architecture: HIGH — modules + trait shape follow design doc §6.8 + CONTEXT.md decisions verbatim
- Pitfalls: HIGH — enumerated from libxc source analysis and existing Rust code
- libxc semantics (HYB classification, mix_func, propagation helpers): HIGH — direct read of vendored C source
- Aux arity / max depth: MEDIUM — empirical grep verified 1..=6 arity but per-id depth requires xtask BFS (A1)
- Non-Copy propagation existence: LOW — hypothesis is "all Copy", flagged as A2 for Wave-0 verification
- Test coverage strategy: HIGH — derived from CLAUDE.md "oracle-based deterministic" policy, not arbitrary

**Research date:** 2026-04-24
**Valid until:** 2026-05-24 (30 days; libxc is stable, Rust deps are stable, no fast-moving components)
