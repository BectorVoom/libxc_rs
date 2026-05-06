# Phase 6: Public API and C Compatibility - Research

**Researched:** 2026-05-06
**Domain:** Rust FFI / C ABI surface design for libxc 7.0.0 drop-in replacement
**Confidence:** HIGH (all 85 ABI signatures and existing-code touchpoints verified by direct file reads against `libxc-master/src/xc.h` and `src/`)

## Summary

Phase 6 wraps the Phase-5 `Functional` runtime handle in two outer rings: (a) `src/api/{batch,builder,evaluate}.rs` — pure Rust ergonomic Layer-3, zero `unsafe` — and (b) `src/compat/*` — the **85** extern "C" entry points that constitute the Layer-1 drop-in replacement for `libxc.so`. CONTEXT.md locks the major design moves: `xc_func_type` is opaque (forward-declared in C; `Box<FunctionalSlot>` in Rust); `BatchEvaluator` owns one `EvaluationWorkspace` (no Functional, fixed `np_max`, `BatchOverflow` on grow); `evaluate()` auto-dispatches via a sealed `EvaluateInput` trait with three impls; the compat boundary returns `int` errno codes everywhere (one `void → int` signature departure from strict drop-in) and uses `catch_unwind` at every entry point.

The 85 functions cleave cleanly into 11 families [VERIFIED: `grep` of `libxc-master/src/xc.h`]: lifecycle (5), thresholds (4), ext_params getter/setter (5), info accessors (10), reference accessors (4), discovery (8), library version (5), hybrid + aux + NLC (7), LDA evaluate (12), GGA evaluate (14, includes 2 ak13 helpers), MGGA evaluate (11). The 33 `xc_lda*/xc_gga*/xc_mgga*` evaluate functions all delegate to a single per-family `xc_*_new(p, order, np, rho, …, out)` that takes an explicit derivative `order` and a pointer-of-pointers `xc_*_out_params` struct — Phase 6's compat shim builds this same `Option<&mut [f64]>`-bundle pattern from raw `*mut f64` arguments using the existing typed `LdaOutput`/`GgaOutput`/`MggaOutput`.

**Primary recommendation:** Hand-write every extern "C" entry point and the C header. The surface is small (~85 signatures, ~100 lines of header), stable (libxc 7.0.0 is the pin), and benefits from a uniform `extern_c_wrapper!` macro that captures `catch_unwind` + thread-local errno + `int` return uniformly. Codegen is more machinery than the surface justifies.

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Builder chain (`FunctionalBuilder`) | Layer-3 `api::builder` | — | Pure Rust ergonomics; no FFI, no `unsafe` |
| Batch driver (`BatchEvaluator`) | Layer-3 `api::batch` | — | Owns workspace; consumes Phase-5 evaluate methods |
| Auto-dispatch (`EvaluateInput` trait) | Layer-3 `api::evaluate` | — | Sealed trait; per-family impls own `dispatch()` |
| Opaque handle lifecycle (`xc_func_*`) | Layer-1 `compat::raw_handle` | Phase-5 `Functional::new`/`Drop` | `unsafe` confined to compat (BUILD-04, COMPAT-03) |
| C-ABI evaluation (`xc_lda_*`, `xc_gga_*`, `xc_mgga_*`) | Layer-1 `compat::legacy_eval` | Phase-5 `Functional::evaluate_*` | Compat builds typed Output bundle from raw ptrs, forwards |
| Discovery (`xc_functional_get_*`, `xc_family_from_id`) | Layer-1 `compat::ids` | Phase-1 registry | Wraps `lookup_by_id`, `lookup_by_name`, `all_functional_ids` |
| Info-struct accessors (`xc_func_info_get_*`) | Layer-1 `compat::info` | `&'static FunctionalMeta` | Opaque info pointer = `&'static FunctionalMeta` cast |
| Hybrid + aux + NLC (`xc_hyb_*`, `xc_nlc_coef`, `xc_aux_func_*`) | Layer-1 `compat::hybrid` | Phase-5 `functional::hybrid` | Wraps `cam_coefficients`, `nlc_coefficients`, `auxiliary_functionals` |
| Library version + reference (`xc_version*`, `xc_reference*`) | Layer-1 `compat::library` | Phase-1 `registry::version*` | Trivial constant returns |
| Errno + panic capture | Layer-1 `compat::errno` | — | Thread-local CString + i32 discriminant |
| C header file | Layer-1 build artifact (`include/xc.h`) | — | Hand-written, committed, mirrors `libxc-master/src/xc.h` 1:1 minus `void→int` |

<phase_requirements>

## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| **API-01** | `FunctionalBuilder` with chained configuration of spin, thresholds, ext_params | Phase-5 `Functional::new(id, spin)` + `set_density_threshold` / `set_zeta_threshold` / `set_sigma_threshold` / `set_tau_threshold` / `set_ext_param[s][_by_index]` provide every primitive. Builder is 100% Rust sugar over these. |
| **API-02** | `BatchEvaluator` with reusable workspace across repeated evaluations | `EvaluationWorkspace::new(np, spin)` (Phase 3 + Phase 5) is the wrappee. Single contiguous `Vec<f64>` sized for MGGA-superset; `lda_scratch_mut`/`gga_scratch_mut`/`mgga_scratch_mut` already materialized. CONTEXT D-A2-2 fixes `np_max` at construction; `BatchOverflow` error on grow. |
| **API-03** | Ergonomic `evaluate()` that dispatches by family automatically | CONTEXT D-A3-1: sealed `EvaluateInput` trait with three impls (`LdaInput`/`GgaInput`/`MggaInput`); each impl owns its `dispatch()` call to `Functional::evaluate_{lda,gga,mgga}`. Family mismatch returns `LibxcRsError::FamilyMismatch` (already in `error/mod.rs:39-44`). |
| **COMPAT-01** | All 85 public C API functions implemented as `extern "C"` | Verified count: 85 functions in `libxc-master/src/xc.h` (`grep` output below). Grouped into 11 families. Each is a thin shim from raw C buffers/ints into typed Phase-5 Rust calls. |
| **COMPAT-02** | C-compatible struct layouts that pass size/alignment assertions | CONTEXT D-A1-1 + D-A1-4: `xc_func_type` and `xc_func_info_type` are **opaque** (forward declarations in C, zero-sized markers in Rust). Layout assertions reduce to: (a) opaque struct compile-time `size_of` = 0 marker; (b) `repr(C)` enums for `Family`/`Kind`/`HybridType` match libxc's `XC_FAMILY_*`/`XC_HYB_*` int constants; (c) function signatures match. |
| **COMPAT-03** | Unsafe code confined to `compat/` | Already true for `src/compat/{c_layout,ids,legacy_eval,raw_handle,removed}.rs` (placeholder modules — unsafe will arrive with implementation). Layer-3 `api/*` stays zero-`unsafe`. BUILD-04 is the project-wide invariant. |

</phase_requirements>

## Project Constraints (from CLAUDE.md)

| Directive | Phase-6 Implication |
|-----------|---------------------|
| Pure Rust + CubeCL 0.9.0; no C/Fortran in production path | Compat layer uses **only** `std::ffi`, `std::panic`, `std::os::raw` — no FFI calls **out** to libxc. Compat is the `extern "C"` provider, not consumer. `libxc-sys` stays a verify-tier-only dep. |
| Precision: f64 only | All buffers in compat ABI are `*const/*mut f64`. No f32 path. |
| Production deps frozen at thiserror 2.0, bitflags 2.10, bytemuck 1.25, cubecl 0.9 | Phase 6 adds **no production deps**. Builder uses hand-rolled chain (no `typed-builder`/`derive_builder`). Layout assertions use `const _: () = assert!(...)` (no `static_assertions` crate needed). |
| Drop-in extern "C" replacement for libxc | The 85-function ABI is the contract. Two conscious deviations (D-A4-1 + D-A1-1): (a) `void → int` for setters and evaluators, (b) `xc_func_type`/`xc_func_info_type` are opaque-only (no field access from C). |
| Operation order preservation for bit-level oracle parity | Phase 6 is wrapper-only — does **not** touch kernel math. Existing per-functional bit-equivalence is preserved by construction. |
| Edition 2024, MSRV 1.85+ | Rust 1.95.0 toolchain present (`rustc --version` confirms). All Phase-6 surface compiles under edition 2024. |
| GSD: no direct file edits outside a workflow | Phase 6 work happens through `/gsd-execute-phase`. |

## Domain Context

### What each requirement actually demands

**API-01 (FunctionalBuilder):** Sugar over Phase-5's existing constructor + 4 threshold setters + 3 ext_param setter shapes. The cost question is whether `.build()` does a `Functional::new()` followed by a sequence of `set_*` calls, OR whether the builder accumulates raw values and constructs in one shot. CONTEXT leaves the chain shape (owned `self` vs `&mut self`) to the planner. Recommendation: owned `self` chain, `.build() -> Result<Functional>` does `Functional::new` then applies accumulated thresholds + ext_params in a defined order; on any setter error, surface with `?`. No allocation cost concern — `Vec<(name, value)>` for ext_param overrides is O(few).

**API-02 (BatchEvaluator):** Ownership locked to **workspace only** (CONTEXT D-A2-1). The ergonomic shape is `BatchEvaluator::new(spin, np_max)` → `be.evaluate(&functional, &input, order, &mut output)`. Family is irrelevant at construction since the workspace sizes for MGGA-superset (Phase 3 D-12 — verified in `src/eval/workspace.rs:170-179`); a single workspace can drive any Functional that fits within `np_max`. `BatchOverflow { requested, capacity }` is a new `LibxcRsError` variant.

**API-03 (evaluate auto-dispatch):** The sealed `EvaluateInput` trait pattern (CONTEXT D-A3-1) has three impls, each calling the right Phase-5 method. Existing code already proves this works: `Functional::evaluate_{lda,gga,mgga}` (`src/functional/evaluate.rs:34-102`) take typed Input/Output bundles plus a workspace. The trait is a thin dispatch hub; no transmute, no `Any` downcast — pure type-driven.

**COMPAT-01 (85 extern "C" functions):** Grouped surface (verified by `grep` of `xc.h`):

| Group | Count | Lines in `xc.h` | Notes |
|-------|-------|-----------------|-------|
| Lifecycle | 5 | 390-398 | `xc_func_alloc/init/end/free/get_info` |
| Thresholds | 4 | 401-407 | `set_dens/zeta/sigma/tau_threshold` (currently `void`; we change to `int`) |
| Ext_params | 5 | 410-418 | `set/get_ext_params[_name][_value]` (3 setters/getters) |
| Info accessors | 10 | 305-316 | `xc_func_info_get_{number,kind,name,family,flags,references,n_ext_params,ext_params_name,ext_params_description,ext_params_default_value}` |
| Reference accessors | 4 | 177-180 | `xc_func_reference_get_{ref,doi,bibtex,key}` |
| Discovery | 8 | 370-387 | `xc_functional_get_{number,name}`, `xc_family_from_id`, `xc_number_of_functionals`, `xc_maximum_name_length`, `xc_available_functional_{numbers,numbers_by_name,names}` |
| Library version | 5 | 17-26 | `xc_reference[_doi/_key]`, `xc_version`, `xc_version_string` |
| Hybrid+aux+NLC | 7 | 588-601 | `xc_hyb_{type,exx_coef,cam_coef}`, `xc_nlc_coef`, `xc_num_aux_funcs`, `xc_aux_func_{ids,weights}` |
| LDA evaluate | 12 | 424-565 | `xc_lda[_new/_exc/_vxc/_fxc/_kxc/_lxc + 5 combinations]` |
| GGA evaluate | 14 | 426-585 | Same 12 GGA pattern + 2 AK13 helpers (`xc_gga_ak13_get_asymptotic`, `xc_gga_ak13_pars_get_asymptotic`) |
| MGGA evaluate | 11 | 436-580 | 11 MGGA shapes (no `xc_mgga_new` exposed) |
| **TOTAL** | **85** | | Matches REQUIREMENTS.md COMPAT-01 |

**COMPAT-02 (C struct layouts):** Per CONTEXT D-A1-1/D-A1-4, both `xc_func_type` and `xc_func_info_type` are **opaque** (forward declarations only in the C header). The "layout assertion" therefore reduces to:
1. `repr(C)` enum constants on the Rust side match libxc's `#define`s for the few values the C header DOES expose: `XC_FAMILY_LDA=1`, `XC_FAMILY_GGA=2`, `XC_FAMILY_MGGA=4`, `XC_UNPOLARIZED=1`, `XC_POLARIZED=2`, `XC_EXCHANGE=0`, `XC_CORRELATION=1`, `XC_EXCHANGE_CORRELATION=2`, `XC_KINETIC=3`, `XC_HYB_*` constants, `XC_FLAGS_*` flag bits, `XC_MAX_REFERENCES=5`, `XC_EXT_PARAMS_DEFAULT=-999998888` [VERIFIED: `xc.h` lines 31-102].
2. Phase-5 already has `Family`/`Kind`/`Spin`/`HybridType`/`HybridTermKind`/`FunctionalFlags` with `#[repr(uN)]` ([VERIFIED: `src/model/mod.rs:13-151`]) — Phase 6 just needs to assert `Family::Lda as i32 == 1`, etc.
3. The **opaque struct** assertion is `const _: () = assert!(std::mem::size_of::<xc_func_type>() == 0);` if we model it as `struct xc_func_type { _opaque: [u8; 0] }`. This compile-fails if anyone accidentally adds a field.

**COMPAT-03 (unsafe confined):** BUILD-04 in REQUIREMENTS.md says: "No unsafe code outside `compat/`, `kernel/launch.rs`, and GPU buffer management." Phase-5 already maintains this invariant. Phase 6 must not break it: `api/*` stays zero-`unsafe`; all raw pointer dereferencing happens inside `compat/`. The `extern_c_wrapper!` macro lives in `compat/`.

### libxc lifecycle semantics (verified from `libxc-master/src/functionals.c:224-391`)

```
xc_func_alloc()      -> libxc_malloc(sizeof(xc_func_type))   [no init, raw memory]
xc_func_init(p, id, nspin)
                     -> nullify all fields
                     -> set nspin, info, dim
                     -> dens_threshold = info->dens_threshold
                     -> sigma_threshold = pow(dens_threshold, 4/3)
                     -> zeta_threshold = DBL_EPSILON ≈ 2.22e-16
                     -> tau_threshold = 1e-20
                     -> if info->init: info->init(func)         [allocates aux + hybrid coefs]
                     -> if ext_params.n > 0:
                          ext_params = libxc_malloc(n*sizeof(double))
                          set_ext_params(func, info->ext_params.values)
                     -> returns 0 on success, -1 on init error, -2 on family-not-found
xc_func_end(p)       -> if info->end: info->end(func)
                     -> recursively xc_func_end + libxc_free each func_aux[i]
                     -> free mix_coef, hyb_*, ext_params, params, info
                     -> nullify (so init can be re-run)
xc_func_free(p)      -> libxc_free(p)
```

**Critical observations for Rust port:**
- `xc_func_init` returning `int` is already libxc 7.0.0's contract (-1, -2 on error). Our compat shim returning `int` is an *augmentation* of an already-int-returning contract for this one function.
- The `nullify` step in `xc_func_init` makes re-initialization (init → end → init again) legal. CONTEXT D-A1-2 captures this: `xc_func_init` on an already-initialized slot **overwrites**.
- `xc_func_end` walks aux recursively, mirroring how Phase-5 `Functional` already drops `auxiliaries: Vec<Functional>` recursively. **No FFI cleanup needed** because we own the aux through Rust's `Vec<Functional>`.
- `xc_func_set_dens_threshold` walks aux recursively too (`functionals.c:407-409`). Phase-5 `Functional::set_density_threshold` does NOT currently propagate to auxiliaries [VERIFIED: `src/functional/config.rs:153-167`]. **Pitfall — see Pitfalls section.**

### Evaluation entry-point structure (verified from `libxc-master/src/lda.c:104-235`)

The 33 evaluation entry points across LDA/GGA/MGGA are mostly thin wrappers that:
1. `memset(&out, 0, sizeof(xc_*_out_params))` — zero all pointers in the struct.
2. Assign the caller's raw pointers into the relevant struct fields.
3. Call `xc_*_new(p, order, np, rho, …, &out)` with the right `order`.

E.g. `xc_lda_exc(p, np, rho, *zk)` → `out.zk = zk; xc_lda_new(p, 0, np, rho, &out);`. Pass NULL for `*zk` to skip — the kernel tests `out.zk != NULL` internally. The "old" `xc_lda(p, np, rho, zk, vrho, v2rho2, v3rho3, v4rho4)` derives `order` from "highest non-NULL pointer."

This is a perfect 1:1 fit for Phase-5's `LdaOutput` (each field is `Option<&mut [f64]>`) — the compat shim maps NULL → `None`, builds the typed Output, calls `Functional::evaluate_lda(input, order, &mut output, &mut workspace)`.

### `xc_*_out_params` struct (verified from `xc.h:196-246`)

Each family has a struct of pointers (one per derivative field):
- `xc_lda_out_params`: 5 pointers (zk, vrho, v2rho2, v3rho3, v4rho4)
- `xc_gga_out_params`: 15 pointers (zk + 2 + 3 + 4 + 5)
- `xc_mgga_out_params`: 70 pointers (zk + 4 + 10 + 20 + 35)

These structs are **passed by-pointer** to `xc_lda_new`/`xc_gga_new` (MGGA only has the legacy split-arg APIs in the public header). Phase 6 does **not** need to expose these structs to Rust callers — they are internal to the libxc C API. We may need to declare them in our header file IF we want to expose `xc_lda_new` to C callers (which we do: it's one of the 85). The compat implementation reads the pointers out of the struct and constructs the Phase-5 `LdaOutput` from them.

## Standard Stack

### Core (already in production deps; no changes)

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `std::ffi` (core std) | — | `c_char`, `c_int`, `c_void`, `CString`, `CStr` | Mandatory for any extern "C" surface; zero-cost. [VERIFIED: stdlib] |
| `std::panic` (core std) | — | `catch_unwind`, `AssertUnwindSafe` | Sole UB-safe panic→errno bridge. [VERIFIED: stdlib + RFC 2945](https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html) |
| `std::os::raw` (core std) | — | `c_double`, `c_size_t` (via `core::ffi::c_size_t` since 1.64) | Stable raw C type aliases. |
| `thiserror` | 2.0.18 | Existing `LibxcRsError` enum extension | Already in use; Phase 6 adds 4 new variants (`BatchOverflow`, `FamilyMismatch` (already exists!), `UninitializedHandle`, `Panicked`). [VERIFIED: `Cargo.toml:9`] |

### Verification (dev-only; no changes)

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `libxc-sys` | 0.1 (workspace path) | bindgen FFI to libxc 7.0.0 — verify-tier oracle | Already factored out by Phase 5 D-03. [VERIFIED: `libxc-sys/Cargo.toml`] |
| `approx` | 0.5.1 | f64 oracle comparisons in tests | Existing dev-dep; reused by `verify/tests/compat_smoke.rs` if added. |

### Alternatives Considered (rejected)

| Instead of | Could Use | Why rejected |
|------------|-----------|--------------|
| Hand-rolled builder chain | `typed-builder` 0.23.2, `derive_builder` 0.20.2 [VERIFIED: `cargo search`] | Would add a production dep purely for sugar. Builder is ~30 lines of straight Rust. CLAUDE.md says no new prod deps. |
| `const _: () = assert!(...)` | `static_assertions` 1.1.0 [VERIFIED: `cargo search`] | Edition 2024 / Rust 1.85+ has const assertions in core. No external dep needed. |
| Hand-written C header | `cbindgen` 0.29.2 [VERIFIED: `cargo search`] | The surface is small (~85 declarations), stable (libxc 7.0.0 pin), and we **want** to depart from auto-generation in two specific ways (opaque-only, void→int). Hand-writing produces a smaller, more readable header that mirrors `libxc-master/src/xc.h` line-for-line. cbindgen's value is in churn-heavy APIs; ours is a frozen pin. Recommend hand-write; place at `include/xc.h` under repo root. |
| `extern "C-unwind"` | New `C-unwind` ABI (RFC 2945, stable in 1.71) | Couples our ABI to the caller's runtime (per CONTEXT D-A4-2). C callers compiled with `-fno-exceptions` would still abort on Rust panic. `catch_unwind` keeps the boundary clean. |
| Codegen the 33 evaluate functions | xtask reading `xc.h` | Adds machinery for a one-off translation. Each function is 5-10 lines of NULL-mapping + bundle construction. ~250 lines total — easier to read 250 hand-written lines than 50 lines of codegen + a generator. |

**Installation:** No new dependencies. Phase 6 ships entirely on existing prod deps.

**Version verification:** Verified against the `Cargo.toml`:
```
thiserror = "2.0.18"   # already in [dependencies]
bitflags = "2.10.0"    # already in [dependencies]
```

## Architecture Patterns

### System Architecture Diagram

```
                   ┌─────────────────────────────────────────────────┐
                   │                  C / Fortran Caller              │
                   │   (DFT code: PySCF, NWChem, Quantum ESPRESSO…)   │
                   └──────────────────────┬──────────────────────────┘
                                          │  #include "xc.h"
                                          │  link against libxc_rs.so
                                          ▼
                   ┌─────────────────────────────────────────────────┐
                   │         compat/    (Layer-1 — `unsafe`)          │
                   │                                                  │
                   │   ┌──────────┐ ┌──────────┐ ┌─────────────┐     │
                   │   │ raw_     │ │ ids      │ │ legacy_eval │     │
                   │   │ handle   │ │          │ │ (33 funcs)  │     │
                   │   └────┬─────┘ └────┬─────┘ └──────┬──────┘     │
                   │   ┌────▼─────┐ ┌────▼─────┐ ┌──────▼──────┐     │
                   │   │ info     │ │ hybrid   │ │ library     │     │
                   │   │          │ │          │ │ (version)   │     │
                   │   └──────────┘ └──────────┘ └─────────────┘     │
                   │                                                  │
                   │   errno.rs:  thread_local CString + i32          │
                   │   c_layout.rs: opaque struct + repr asserts     │
                   │   removed.rs: 52 removed-id error mapping       │
                   │                                                  │
                   │   every entry point wrapped by                   │
                   │   `extern_c_wrapper!{ ... }` macro:              │
                   │     catch_unwind → set_errno → return int        │
                   └──────────────────────┬──────────────────────────┘
                                          │
                                          │  read &Functional via Box<FunctionalSlot>
                                          ▼
                   ┌─────────────────────────────────────────────────┐
                   │          api/      (Layer-3 — zero `unsafe`)     │
                   │                                                  │
                   │   ┌─────────────┐ ┌─────────────┐ ┌──────────┐  │
                   │   │ batch       │ │ builder     │ │ evaluate │  │
                   │   │ Batch-      │ │ Functional- │ │ Evaluate │  │
                   │   │ Evaluator   │ │ Builder     │ │ Input    │  │
                   │   │ (workspace) │ │ (chain)     │ │ (sealed) │  │
                   │   └──────┬──────┘ └──────┬──────┘ └─────┬────┘  │
                   └──────────┼───────────────┼──────────────┼───────┘
                              │               │              │
                              ▼               ▼              ▼
                   ┌─────────────────────────────────────────────────┐
                   │       functional/   (Layer-2 — Phase 5 — frozen) │
                   │                                                  │
                   │   Functional::new(id, spin) -> Result            │
                   │   Functional::evaluate_{lda,gga,mgga}            │
                   │   Functional::set_{density,zeta,sigma,tau}_thresh│
                   │   Functional::{set,get}_ext_param[s][_by_index]  │
                   │   Functional::hybrid_type / cam_coefficients /…  │
                   │   Functional::auxiliary_functionals              │
                   └──────────────────────┬──────────────────────────┘
                                          │
                                          ▼
                   ┌─────────────────────────────────────────────────┐
                   │  eval/  +  registry/  +  meta/  +  kernel/       │
                   │  (everything Phases 1-5 already shipped)         │
                   └─────────────────────────────────────────────────┘
```

**Reading the diagram:** A `xc_lda_exc(p, np, rho, zk)` C call enters compat's `legacy_eval::xc_lda_exc`. The wrapper macro starts `catch_unwind`. Inside: read `*p` as `*mut FunctionalSlot`, match on `Initialized(functional)` (set errno + return on `Empty`), build `LdaInput::new(rho_slice, np, functional.spin)?`, build `LdaOutput { zk: Some(slice), ..Default::default() }`, allocate-or-borrow a workspace, call `functional.evaluate_lda(&input, DerivativeOrder::Exc, &mut output, &mut workspace)?`. On Ok → return 0. On Err → set thread-local errno → return discriminant. On panic → catch_unwind catches → set `LIBXC_RS_PANIC` errno → return -1.

### Recommended Project Structure

```
src/
├── api/
│   ├── mod.rs            # add `pub mod evaluate;`, re-export builder/batch/evaluate
│   ├── batch.rs          # BatchEvaluator (rewrite from placeholder)
│   ├── builder.rs        # FunctionalBuilder (rewrite from placeholder)
│   └── evaluate.rs       # NEW: sealed EvaluateInput trait + 3 impls
├── compat/
│   ├── mod.rs            # update: add new submodules below
│   ├── c_layout.rs       # opaque types + #[repr(C)] enum asserts (rewrite)
│   ├── raw_handle.rs     # FunctionalSlot + xc_func_alloc/init/end/free (rewrite)
│   ├── ids.rs            # 8 discovery functions (rewrite)
│   ├── legacy_eval.rs    # 33 evaluation functions + 4 thresholds + 5 ext_params (rewrite)
│   ├── removed.rs        # removed-id errno mapping (rewrite)
│   ├── info.rs           # NEW: 10 xc_func_info_get_* + 4 xc_func_reference_get_*
│   ├── hybrid.rs         # NEW: 7 hybrid+aux+nlc + xc_gga_ak13_*
│   ├── library.rs        # NEW: 5 xc_version*/xc_reference*
│   ├── errno.rs          # NEW: thread-local errno + xc_rs_last_error_*
│   └── macros.rs         # NEW: extern_c_wrapper! + repr_assert!
├── error/
│   └── mod.rs            # extend: BatchOverflow, UninitializedHandle, Panicked + discriminant()
└── lib.rs                # add pub use api::{BatchEvaluator, FunctionalBuilder, EvaluateInput}

include/                  # NEW
└── xc.h                  # hand-written C header (committed)

verify/
└── tests/
    └── compat_smoke.rs   # NEW: minimum-viable FFI integration test
```

### Pattern 1: Opaque type with `Box<T>` round-trip

**What:** Expose a Rust-owned struct to C as an opaque pointer; never dereference fields from C.

**When to use:** When the C ABI doesn't need to peer inside (D-A1-1 makes this libxc_rs's choice).

**Example:**
```rust
// Source: CONTEXT.md Specifics + RFC pattern + libxc xc_func_alloc/init/end/free flow
// Verified against: libxc-master/src/functionals.c:224-391

// In C header (hand-written include/xc.h):
//   typedef struct xc_func_type xc_func_type;        // forward decl only
//   xc_func_type *xc_func_alloc();
//   int           xc_func_init(xc_func_type *p, int functional, int nspin);
//   void          xc_func_end (xc_func_type *p);
//   void          xc_func_free(xc_func_type *p);

// In Rust src/compat/c_layout.rs:
#[repr(C)]
pub struct xc_func_type {
    _opaque: [u8; 0],            // zero-size; compile-asserts no field access
    _marker: std::marker::PhantomData<(*mut u8, std::marker::PhantomPinned)>,
}
// PhantomData with raw pointer + PhantomPinned makes the type !Send/!Sync/!Unpin
// from C's perspective (defensive — actual !Send is enforced by FunctionalSlot).

// Compile-time assertion: opaque struct is zero-sized.
const _: () = assert!(std::mem::size_of::<xc_func_type>() == 0);

// In Rust src/compat/raw_handle.rs:
pub(crate) enum FunctionalSlot {
    Empty,
    Initialized(crate::Functional),
}

#[unsafe(no_mangle)]
pub extern "C" fn xc_func_alloc() -> *mut xc_func_type {
    // Box::into_raw transfers ownership to C. C MUST call xc_func_free.
    let slot = Box::new(FunctionalSlot::Empty);
    Box::into_raw(slot) as *mut xc_func_type
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_init(
    p: *mut xc_func_type,
    functional: i32,
    nspin: i32,
) -> i32 {
    extern_c_wrapper!(p, "xc_func_init", {
        let id = FunctionalId::from_raw(functional as u16)?;
        let spin = match nspin {
            1 => Spin::Unpolarized,
            2 => Spin::Polarized,
            _ => return Err(LibxcRsError::SpinMismatch { /* ... */ }),
        };
        let f = Functional::new(id, spin)?;
        // SAFETY: p is a valid pointer to FunctionalSlot from xc_func_alloc.
        // Overwriting the slot is libxc-compatible (D-A1-2: re-init allowed).
        unsafe { std::ptr::write(p as *mut FunctionalSlot, FunctionalSlot::Initialized(f)); }
        Ok(0_i32)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_end(p: *mut xc_func_type) -> i32 {
    extern_c_wrapper!(p, "xc_func_end", {
        // SAFETY: p is from xc_func_alloc; replace Initialized with Empty,
        // letting the inner Functional drop.
        unsafe { std::ptr::write(p as *mut FunctionalSlot, FunctionalSlot::Empty); }
        Ok(0_i32)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_free(p: *mut xc_func_type) {
    if p.is_null() { return; }
    // SAFETY: p is from xc_func_alloc. Box::from_raw reclaims ownership and drops.
    unsafe { drop(Box::from_raw(p as *mut FunctionalSlot)); }
}
```

### Pattern 2: `extern_c_wrapper!` macro for uniform panic + errno + int return

**What:** A single declarative macro every extern "C" entry point uses to enforce:
1. `catch_unwind` around the body
2. NULL-pointer checks on the `xc_func_type*` argument
3. `Ok(i32)` → return; `Err(LibxcRsError)` → set thread-local errno + return discriminant
4. Caught panic → set `LIBXC_RS_PANIC` errno + return `-1`

**When to use:** Every single extern "C" function in `compat/`. No exceptions.

**Example (sketch):**
```rust
// Source: composed from CONTEXT D-A4-1/D-A4-2 + RFC 2945 + libxc-rs error infra
// std::panic::catch_unwind reference: https://doc.rust-lang.org/std/panic/fn.catch_unwind.html

#[macro_export]
macro_rules! extern_c_wrapper {
    ($p:expr, $name:literal, $body:block) => {{
        // Step 1: NULL handle check (only when $p is meaningful — an alternative
        // form skips this for functions that don't take *mut xc_func_type).
        if $p.is_null() {
            $crate::compat::errno::set_error(
                $crate::compat::errno::LIBXC_RS_NULL_HANDLE,
                concat!($name, ": null xc_func_type pointer"),
            );
            return $crate::compat::errno::LIBXC_RS_NULL_HANDLE;
        }

        // Step 2: catch_unwind guard. AssertUnwindSafe acknowledges that
        // captured raw pointers are not statically UnwindSafe; libxc's
        // contract is "single-threaded per handle" (D-A1-3) so the
        // user-observed UB risk is the same as in the original C code.
        let result: Result<i32, $crate::LibxcRsError> = std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| $body),
        )
        .unwrap_or_else(|payload| {
            let msg = if let Some(s) = payload.downcast_ref::<&str>() {
                (*s).to_string()
            } else if let Some(s) = payload.downcast_ref::<String>() {
                s.clone()
            } else {
                "unknown panic in libxc_rs compat layer".to_string()
            };
            $crate::compat::errno::set_error(
                $crate::compat::errno::LIBXC_RS_PANIC,
                &format!("{}: panic — {}", $name, msg),
            );
            Err($crate::LibxcRsError::Panicked { message: msg })
        });

        // Step 3: surface result.
        match result {
            Ok(code) => code,
            Err(e) => {
                let code = $crate::compat::errno::discriminant(&e);
                $crate::compat::errno::set_error(code, &e.to_string());
                code
            }
        }
    }};
}
```

### Pattern 3: NULL-mapping for evaluation buffers

**What:** Map `*mut f64` arguments to `Option<&mut [f64]>` for typed Output bundle construction. NULL = `None` = "skip this derivative."

**When to use:** Every entry point in the 33 LDA/GGA/MGGA evaluate family.

**Example:**
```rust
// Source: libxc-master/src/lda.c:104-125 (xc_lda's NULL-skip semantics)
// Phase 3 D-05 + Phase 6 D-A4-3 + src/output/mod.rs:51-75

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_lda_exc_vxc(
    p: *const xc_func_type,
    np: usize,
    rho: *const f64,
    zk: *mut f64,
    vrho: *mut f64,
) -> i32 {
    extern_c_wrapper!(p, "xc_lda_exc_vxc", {
        let f = unsafe { read_initialized(p)? };  // helper: returns Result<&Functional, _>
        let dims = f.dims();

        // SAFETY: rho is non-null (caller contract); length validated by LdaInput::new.
        let rho_slice = unsafe { std::slice::from_raw_parts(rho, np * dims.rho as usize) };
        let input = LdaInput::new(rho_slice, np, f.spin())?;

        // NULL→None per D-A4-3:
        let zk_opt = if zk.is_null() { None } else {
            Some(unsafe { std::slice::from_raw_parts_mut(zk, np * dims.zk as usize) })
        };
        let vrho_opt = if vrho.is_null() { None } else {
            Some(unsafe { std::slice::from_raw_parts_mut(vrho, np * dims.vrho as usize) })
        };
        let mut out = LdaOutput::new(zk_opt, vrho_opt, None, None, None, np, f.spin())?;

        // Per-call workspace allocation (D-A2-1 BatchEvaluator path is for Rust API,
        // not the C ABI — C callers don't get the BatchEvaluator ergonomics).
        let mut ws = EvaluationWorkspace::new(np, f.spin());

        f.evaluate_lda(&input, DerivativeOrder::Vxc, &mut out, &mut ws)?;
        Ok(0_i32)
    })
}
```

### Pattern 4: Sealed dispatch trait (`EvaluateInput`)

**What:** A trait sealed by a private supertrait so users can't add impls; three known impls dispatch each input type to the right Phase-5 method.

**When to use:** API-03 auto-dispatch entry point on `BatchEvaluator`.

**Example (sketch):**
```rust
// Source: CONTEXT D-A3-1 + Phase-5 src/functional/evaluate.rs:34-102

mod sealed { pub trait Sealed {} }

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

impl sealed::Sealed for LdaInput<'_> {}
impl<'i> EvaluateInput for LdaInput<'i> {
    type Output<'a> = LdaOutput<'a>;
    fn dispatch(/* args */) -> Result<(), LibxcRsError> {
        if functional.meta().family != Family::Lda {
            return Err(LibxcRsError::FamilyMismatch {
                id: functional.meta().id,
                expected: functional.meta().family,
                actual: Family::Lda,
            });
        }
        functional.evaluate_lda(self, order, output, workspace)
    }
}
// (same for GgaInput → GgaOutput / Family::Gga, MggaInput → MggaOutput / Family::Mgga)

pub struct BatchEvaluator { ws: EvaluationWorkspace, np_max: usize, spin: Spin }

impl BatchEvaluator {
    pub fn new(spin: Spin, np_max: usize) -> Self {
        Self { ws: EvaluationWorkspace::new(np_max, spin), np_max, spin }
    }
    pub fn evaluate<I: EvaluateInput>(
        &mut self, functional: &Functional, input: &I,
        order: DerivativeOrder, output: &mut I::Output<'_>,
    ) -> Result<(), LibxcRsError> {
        // BatchOverflow guard
        let np = /* extract input's np via the trait or a sibling getter */ 0;
        if np > self.np_max {
            return Err(LibxcRsError::BatchOverflow { requested: np, capacity: self.np_max });
        }
        if functional.spin() != self.spin {
            return Err(LibxcRsError::SpinMismatch { expected: self.spin, actual: functional.spin() });
        }
        input.dispatch(functional, order, output, &mut self.ws)
    }
}
```

### Anti-Patterns to Avoid

- **`#[no_mangle]` directly on impl methods** — extern "C" entry points must be free functions in `compat/*.rs`. Free functions only.
- **`Box::from_raw` on a pointer that may be NULL** — every `xc_func_free`-style function must NULL-check first; see lifecycle pattern above.
- **Returning Rust-allocated `*mut c_char` to C without a `xc_*_free` companion** — C side needs a way to free anything we allocate. For `xc_functional_get_name` (`libxc-master/src/functionals.c:58-74` shows libxc returns `libxc_malloc`'d strings), we have two valid options: (a) return a `*const c_char` into a thread-local cache (lives until next call on same thread; document loudly); (b) provide `xc_rs_free_string()` companion. **Recommendation: option (a)** — matches our errno pattern and avoids adding a 86th function. Document the lifetime in the C header next to the function.
- **Holding a `&mut Functional` across an `extern "C"` boundary** — borrow-checker can't help here; just don't. Instead, use `&Functional` (CONTEXT D-A3-2) and require setters on the Rust side, then re-enter the C ABI for evaluation.
- **`extern "C" fn` body that can panic from arithmetic / indexing without `catch_unwind`** — every entry point goes through `extern_c_wrapper!`; no exceptions.
- **Mirroring libxc's `assert!()` checks (e.g. `xc_hyb_exx_coef`'s assert that the functional is a Hybrid)** — convert to typed errors. The libxc C side aborts; we return `LIBXC_RS_FAMILY_MISMATCH` errno.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Panic-to-errno bridge | A custom `mem::replace`-based panic hook | `std::panic::catch_unwind` | Stdlib primitive; the only UB-safe path. RFC 2945 explicitly warns against custom unwind handling. [CITED: [Rust panic docs](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html)] |
| C string lifetime management | A custom interning arena | `std::ffi::CString` + `thread_local!{ RefCell<CString> }` | Stdlib pattern; documented in `CString` API. Thread-local is the standard solution for "valid until the next call on this thread." [CITED: [CString docs](https://doc.rust-lang.org/std/ffi/struct.CString.html)] |
| Layout assertions | `assert_eq_size!` from `static_assertions` | `const _: () = assert!(...)` (Rust 1.79+) | Rust 1.95 (toolchain present) supports inline const assertions natively. No external dep needed. |
| C header generation | A custom Python/awk script | Either cbindgen OR hand-write | If hand-writing, keep it simple — the surface is small. If using cbindgen, configure for `style = "type"` typedef shape and mark `xc_func_type` / `xc_func_info_type` as opaque. **Recommend hand-write** (CONTEXT § Specifics — strong recommendation). |
| Thread-local mutable state | `static mut` (UB-prone) | `thread_local!` macro + `RefCell` | Standard pattern; safe-by-construction. |
| Builder field-by-field copying | A `derive_builder` macro | Hand-rolled chain | ~30-50 lines for our 7 setters. Adding a proc-macro dep is overkill. |
| Aux propagation of thresholds in compat | A new "walk aux" helper in compat | **Add to Phase-5 `Functional::set_*_threshold`** | libxc's `xc_func_set_dens_threshold` (functionals.c:407-409) walks aux recursively. Phase-5 currently doesn't (`config.rs:153-167`). Two clean fixes: (a) make Phase-5 setters walk aux too (matches libxc semantics for both Rust and C callers); (b) replicate the walk inside compat. **Recommendation (a)** — single source of truth. See Pitfall 4. |

**Key insight:** The compat layer is ~85 thin shims over Phase-5 methods. Don't reach for libraries; the boilerplate IS the contract. The only "framework" we need is one well-tested macro (`extern_c_wrapper!`) applied uniformly.

## Runtime State Inventory

> Phase 6 is greenfield-within-an-existing-project: no rename, no migration, no string replacement. The existing `compat/*` modules are 2-line placeholders with no runtime state.

| Category | Items Found | Action Required |
|----------|-------------|------------------|
| Stored data | None — verified by `grep`. No databases, vector stores, or persisted runtime state in libxc_rs. | None |
| Live service config | None — verified by inspection. No external services. | None |
| OS-registered state | None — verified by inspection. No scheduled tasks, daemons, or system registrations. | None |
| Secrets / env vars | `RUST_MIN_STACK` set in `.cargo/config.toml` (toolchain config, not runtime) — unaffected. No secrets. | None |
| Build artifacts | `target/debug/build/libxc-sys-*/out/` contains pre-Phase-6 bindgen output — irrelevant to compat layer (libxc-sys is verify-tier). | None |

**Nothing found in any category. Phase 6 introduces new state (thread-local errno cell), but does not migrate any existing state.**

## Common Pitfalls

### Pitfall 1: `xc_func_init` re-initialization semantics

**What goes wrong:** Calling `xc_func_init(p, id1, nspin)` then `xc_func_init(p, id2, nspin)` without an intervening `xc_func_end` should overwrite (per libxc; see `functionals.c:268` `xc_func_nullify` zeroes everything). A naive Rust port might leak the previous Functional or panic on a double-init.

**Why it happens:** The Rust pattern `Box::into_raw → ptr::write` cleanly overwrites, but only if the previous `FunctionalSlot::Initialized(_)` is dropped. `std::ptr::write` does **not** drop the existing value; it overwrites bits.

**How to avoid:** Use `std::ptr::replace(p, FunctionalSlot::Initialized(f))` (swaps + drops old) or explicitly: `let old = std::ptr::read(p); drop(old); std::ptr::write(p, new);`. Test this: `init(p, id1) → init(p, id2) → end(p) → free(p)` must not leak Functional 1.

**Warning signs:** Memory growth in long-running C tests that re-init repeatedly. `cargo test` with valgrind or LeakSanitizer flags.

### Pitfall 2: `xc_functional_get_name` ownership / lifetime

**What goes wrong:** libxc's `xc_functional_get_name(int number) -> char*` returns a `libxc_malloc`'d string that the **caller must free** (`libxc-master/src/functionals.c:58-74`). If we return a `&'static str` cast to `*const c_char`, the C signature is `char *` (mutable, owned) but we'd be giving them a static — calling `free()` on it segfaults.

**Why it happens:** Mismatched ownership convention between Rust's static-everything model and libxc's malloc-everything model.

**How to avoid:** Three options:
1. **Match libxc — allocate + transfer ownership.** Document that callers must `xc_rs_free_string(p)`. Adds a 86th function (we've already accepted void→int as a deviation; another API addition is fine).
2. **Thread-local cache returning `*const c_char`.** Document lifetime: "valid until next `xc_functional_get_name` on this thread." C callers that immediately copy the string are fine.
3. **Change signature to `int xc_functional_get_name(int number, char *out, size_t out_len)`.** Caller-provided buffer. Most ABI-correct but a third deviation from drop-in.

**Recommendation:** Option 2 (thread-local cache) — pairs cleanly with the errno mechanism (already thread-local). Document loudly in the C header.

**Warning signs:** Tests that call `xc_functional_get_name` then `free()` the result will segfault. If we go with option 2, tests that hold the pointer past a second `xc_functional_get_name` call will see corrupted data.

### Pitfall 3: `xc_func_init` returns `int` already, but with libxc-incompatible codes

**What goes wrong:** libxc's `xc_func_init` returns `0` on success, `-1` on init error, `-2` on family-not-found (`functionals.c:297`). Our compat needs to either match these exact codes (binary compat for code that explicitly checks `== -2`) OR document the new code map.

**How to avoid:** The CONTEXT D-A4-1 already accepted "type-checked error reporting beats silent failure." Use our own codes (e.g. `LIBXC_RS_UNKNOWN_FUNCTIONAL_ID = -3`) and document the mapping in the C header. C code that just checks `!= 0` continues to work; code that switches on `-1` / `-2` needs to update.

**Warning signs:** Existing C/Fortran integrators that hard-code `if (rc == -2) ...` see different behavior. Highlight in migration notes.

### Pitfall 4: Threshold setters don't recurse into auxiliaries

**What goes wrong:** libxc's `xc_func_set_dens_threshold(p, t)` (functionals.c:400-410) walks `p->func_aux[i]` and sets the threshold recursively. Phase-5 `Functional::set_density_threshold` (`config.rs:153-167`) does **not** walk `self.auxiliaries`. A hybrid functional configured via `set_density_threshold(1e-12)` won't propagate to its 4 aux (B3LYP) — auxiliary functionals will use the Phase-5 default `1e-15`.

**Why it happens:** The Phase-5 design didn't surface this libxc semantic; the existing tests only verify the parent threshold field, not aux propagation.

**How to avoid:** Two valid fixes:
1. **Modify Phase-5 setters to walk aux.** Single source of truth; both Layer-3 and compat get the right behavior. Adds a one-line fanout in each of 4 setters.
2. **Replicate the walk in compat::legacy_eval::xc_func_set_dens_threshold.** Compat-only fix; Layer-3 Rust callers see the bug.

**Recommendation: option (1).** Add to Phase 6 plan as a small fix on `src/functional/config.rs`. CONTEXT explicitly calls Phase-5 surface "frozen," so flag this in the plan's Wave 0 / scope-clarification step.

**Warning signs:** Oracle parity test on a hybrid functional with non-default density threshold — vrho values differ by orders of magnitude between Rust and C.

### Pitfall 5: Spin polarization is set at init, not switchable

**What goes wrong:** libxc requires `nspin` to be passed to `xc_func_init` and locks the functional to that spin mode forever (`functionals.c:266-271`). A C caller doing `xc_func_init(p, id, XC_UNPOLARIZED) → xc_lda_exc(p, np, rho_polarized, zk)` gets undefined behavior in libxc. Our compat must either reject (clean error) or match libxc's UB.

**How to avoid:** Reject with `SpinMismatch` error inside each evaluate shim. The Phase-5 `LdaInput::new(rho, np, spin)` already validates buffer length against `dims.rho` for the requested spin — but if the C caller passes an unpolarized rho buffer (np * 1 elements) with a polarized Functional, `LdaInput::new` will succeed-by-accident on length match for some sizes. Add an explicit `if input.spin() != functional.spin() return Err(SpinMismatch)` check in the Functional/EvaluateInput layer.

**Warning signs:** Oracle parity passes for unpolarized but fails (sometimes silently) for polarized.

### Pitfall 6: `ext_params` count must match `xc_func_info_get_n_ext_params`

**What goes wrong:** `xc_func_set_ext_params(p, vals)` in libxc asserts `info->ext_params.n > 0` (functionals.c:455) — passing the wrong-sized array is UB. Our compat already has `LibxcRsError::ExtParamCountMismatch` (`error/mod.rs:58-63`); just need to thread it through. Phase-5 `set_ext_params` does the count check (`config.rs:65-73`).

**How to avoid:** The compat shim takes `*const f64` with no length argument. We have to **trust** the caller passed a correctly-sized buffer (matching `meta.ext_params.len()`). Construct the slice as `slice::from_raw_parts(vals, meta.ext_params.len())`, then call `Functional::set_ext_params(slice)`. Phase-5 will validate the length.

**Warning signs:** Calling `xc_func_set_ext_params` with a too-short buffer reads uninitialized memory — may not crash but produces garbage. Document the contract in the C header header next to `xc_func_set_ext_params`.

### Pitfall 7: Removed functional IDs

**What goes wrong:** Of 52 "removed" IDs in libxc 7.0.0, only ID 104 is truly gone; the others are aliases or reassignments (PROJECT.md/STATE.md decision log). Phase-5's registry (`registry::lookup_by_id`) handles this correctly via `removed::REMOVED_IDS`. Compat must surface this through the int errno when a C caller hits a removed ID.

**How to avoid:** `xc_func_init` already errors via `Functional::new → lookup_by_id → RemovedFunctionalId` error. Map this to a dedicated errno code (e.g. `LIBXC_RS_REMOVED_FUNCTIONAL_ID = -10`) and stash the replacement-id message in the thread-local errno text. CONTEXT § Claude's Discretion explicitly leaves this open.

**Warning signs:** A C test that calls `xc_func_init(p, 104, 1)` expects success in libxc 6.x and earlier; we return an error code. Document in migration notes.

### Pitfall 8: `xc_lda` (no _exc/_vxc suffix) derives `order` from highest non-NULL pointer

**What goes wrong:** The "old" `xc_lda(p, np, rho, zk, vrho, v2rho2, v3rho3, v4rho4)` (lda.c:104-125) computes `order` as **the highest non-NULL of the 5 outputs**. A naive port might just call `evaluate_lda` with order=Lxc unconditionally and waste work / mis-handle NULL.

**How to avoid:** Replicate the libxc logic verbatim:
```rust
let order = if !v4rho4.is_null() { DerivativeOrder::Lxc }
       else if !v3rho3.is_null() { DerivativeOrder::Kxc }
       else if !v2rho2.is_null() { DerivativeOrder::Fxc }
       else if !vrho.is_null()   { DerivativeOrder::Vxc }
       else if !zk.is_null()     { DerivativeOrder::Exc }
       else { return Ok(0); };  // all NULL: nothing to compute, libxc returns silently
```

**Warning signs:** Tests of `xc_lda` (the catch-all) fail or do unnecessary work. Specialized `xc_lda_exc`, `xc_lda_exc_vxc`, etc. that hardcode `order` are immune.

### Pitfall 9: GGA includes 2 AK13 helpers that aren't part of the evaluation pattern

**What goes wrong:** `xc_gga_ak13_get_asymptotic(homo) -> double` and `xc_gga_ak13_pars_get_asymptotic(homo, *ext_params) -> double` (xc.h:583-585) are functional-specific helper functions, not evaluation entry points. They take no `xc_func_type*`. Easy to miss when grouping the 14 GGA functions.

**How to avoid:** Implement as **standalone shims** in `compat/legacy_eval.rs` (or a small `compat/ak13.rs` if planner prefers). Math is in libxc-master/src/gga_x_ak13.c — port the small `get_asymptotic` formula directly. Counts toward the 85 total.

**Warning signs:** Phase 6 ships with 83 functions; oracle test fails on AK13 functional usage.

### Pitfall 10: `XC_EXT_PARAMS_DEFAULT` magic value

**What goes wrong:** `xc_func_set_ext_params(p, vals)` substitutes any value equal to `-999998888` (the magic constant `XC_EXT_PARAMS_DEFAULT`) with the per-spec default (functionals.c:457-460 + xc.h:72). If we forget this, callers that explicitly pass `-999998888` to mean "use default" get garbage.

**How to avoid:** In compat::xc_func_set_ext_params, pre-process the input slice: clone into a Vec, replace any element equal to `-999998888.0` with `meta.ext_params[i].default_value`, then forward to Phase-5 `Functional::set_ext_params`. Add a const `pub const LIBXC_EXT_PARAMS_DEFAULT: f64 = -999998888.0;` to `compat/c_layout.rs`.

**Warning signs:** Tests using the default-marker pattern produce numerically wrong results.

### Pitfall 11: Threading — `Functional` is `Send + Sync`, but the C-ABI handle is not

**What goes wrong:** `FunctionalSlot` lives behind a `Box<>` whose pointer is the `*mut xc_func_type`. CONTEXT D-A1-3 locks the contract to "single-threaded per handle." But Rust's borrow checker can't enforce that across an FFI boundary; if a multi-threaded C program shares a single `xc_func_type*` between threads and calls `set_ext_params` concurrently with `xc_lda_exc`, that's a data race (both Rust UB and libxc UB).

**How to avoid:** Document in the C header: "Each `xc_func_type*` may be used by only one thread at a time. Use multiple handles for parallel evaluation." Match libxc's de-facto contract. No runtime check (would cost atomics in the hot path); rely on documentation.

**Warning signs:** Reports of intermittent NaN or segfault under concurrent C-side usage. Add ThreadSanitizer to a `verify/tests/compat_concurrent.rs` test if Phase 7 needs to confirm.

## Code Examples

### Example 1: `xc_func_alloc/init/end/free` round-trip (lifecycle)

See Pattern 1 above. Source verified against `libxc-master/src/functionals.c:224-391`.

### Example 2: `xc_lda_exc` (typical evaluate)

See Pattern 3 above. Source verified against `libxc-master/src/lda.c:130-137`.

### Example 3: `xc_func_info_get_name` (info accessor)

```rust
// Source: libxc-master/src/func_info.c:22-25 + opaque info pattern (CONTEXT D-A1-4)

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_func_info_get_name(
    info: *const xc_func_info_type,
) -> *const std::os::raw::c_char {
    if info.is_null() { return std::ptr::null(); }
    // SAFETY: info is &'static FunctionalMeta cast to *const xc_func_info_type.
    let meta: &'static FunctionalMeta = unsafe { &*(info as *const FunctionalMeta) };
    // Use a thread-local CString cache to give C a NUL-terminated string.
    crate::compat::errno::cache_cstring(meta.name)
}
```

### Example 4: `xc_hyb_cam_coef` (hybrid query)

```rust
// Source: libxc-master/src/hybrids.c:134-157 + Phase-5 src/functional/hybrid.rs:112-150

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xc_hyb_cam_coef(
    p: *const xc_func_type,
    omega: *mut f64,
    alpha: *mut f64,
    beta: *mut f64,
) -> i32 {
    extern_c_wrapper!(p, "xc_hyb_cam_coef", {
        let f = unsafe { read_initialized(p as *mut xc_func_type)? };
        match f.cam_coefficients() {
            Some(c) => {
                if !omega.is_null() { unsafe { *omega = c.omega; } }
                if !alpha.is_null() { unsafe { *alpha = c.alpha; } }
                if !beta.is_null()  { unsafe { *beta  = c.beta; } }
                Ok(0_i32)
            }
            None => Err(LibxcRsError::FamilyMismatch { /* not a CAM/hybrid */ }),
        }
    })
}
```

### Example 5: `EvaluateInput` impl for LdaInput (Layer-3)

See Pattern 4 above.

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `extern "C" fn` + abort-on-panic (Rust 1.32 default) | `catch_unwind` at every entry point + `LIBXC_RS_PANIC` errno | Rust 1.33 made panic-through-FFI UB by default | We retain UB-safety AND give the caller a recoverable error path. [CITED: [Rust unwind through FFI internals discussion](https://internals.rust-lang.org/t/unwinding-through-ffi-after-rust-1-33/9521)] |
| `extern "C-unwind"` ABI | `extern "C"` + `catch_unwind` | RFC 2945 stable in Rust 1.71 (2023) | We deliberately reject `C-unwind` (CONTEXT D-A4-2) because it couples our ABI to caller's C++ runtime. [CITED: [RFC 2945](https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html)] |
| `static_assertions` 1.1.0 crate | `const _: () = assert!(...)` | Rust 1.79 stabilized `assert!` in const context | Layout assertions need no external dep. [VERIFIED: cargo search] |
| cbindgen-generated full struct layouts | Hand-written `typedef struct xc_func_type xc_func_type;` forward declaration only | CONTEXT D-A1-1 / D-A1-4 design choice | Smaller header, accessor-only C API, all `unsafe` confined to compat. [CITED: [cbindgen docs](https://github.com/mozilla/cbindgen/blob/master/docs.md)] |
| `Box::into_raw` + `Box::from_raw` for opaque types | Same — still the canonical pattern | Stable since Rust 1.0 | This is the universally-accepted pattern; no successor crate has emerged. [CITED: [CString docs](https://doc.rust-lang.org/std/ffi/struct.CString.html)] |
| `#[no_mangle]` | `#[unsafe(no_mangle)]` (Rust 1.82+) | Edition 2024 | Project is on Edition 2024 (Cargo.toml). Must use the new spelling. |

**Deprecated/outdated (training data may show these):**
- Some older Rust FFI tutorials use `extern crate libc;` for `c_int`/`c_char`/etc. — replaced by `core::ffi` (stable since Rust 1.64). Use `std::os::raw` or `core::ffi`. No external `libc` dep required for compat.

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust built-in `#[test]` + cargo test (no external test framework) |
| Config file | None — Cargo conventions; `verify/Cargo.toml` defines verify-tier deps |
| Quick run command | `cargo test -p libxc_rs api::` (filters Layer-3 tests) and `cargo test -p libxc_rs compat::` (filters compat unit tests) |
| Full suite command | `cargo test --workspace --no-fail-fast` |
| Test discovery | `find -name '*.rs' -path '*/tests/*'` and `#[test]` blocks in src |

[VERIFIED: `verify/Cargo.toml`, existing tests in `verify/tests/{lda,gga,mgga,hybrid,mixed,metadata,hybrid_type}_oracle.rs`]

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| API-01 | FunctionalBuilder constructs Functional via chained config | unit | `cargo test -p libxc_rs api::builder::tests` | ❌ Wave 0 |
| API-01 | Builder validation: unknown id, bad ext_param, etc. surface as `LibxcRsError` | unit | `cargo test -p libxc_rs api::builder::tests::error_paths` | ❌ Wave 0 |
| API-02 | BatchEvaluator round-trips through 100+ evaluations on same workspace without realloc | unit | `cargo test -p libxc_rs api::batch::tests::workspace_reuse_no_realloc` | ❌ Wave 0 |
| API-02 | BatchEvaluator with `np > np_max` returns `BatchOverflow` | unit | `cargo test -p libxc_rs api::batch::tests::overflow_returns_error` | ❌ Wave 0 |
| API-02 | Single BatchEvaluator drives multiple Functionals with same (family, spin, np_max) | unit | `cargo test -p libxc_rs api::batch::tests::shared_across_functionals` | ❌ Wave 0 |
| API-03 | `EvaluateInput::dispatch` for LdaInput on LDA Functional matches direct evaluate_lda call | unit | `cargo test -p libxc_rs api::evaluate::tests::lda_dispatch_bit_equivalent` | ❌ Wave 0 |
| API-03 | LdaInput on GGA Functional returns `FamilyMismatch` | unit | `cargo test -p libxc_rs api::evaluate::tests::family_mismatch_lda_input_gga_func` | ❌ Wave 0 |
| API-03 | All three impls (Lda, Gga, Mgga) compile under sealed-trait constraint | unit | `cargo test -p libxc_rs api::evaluate::tests::sealed_trait_compiles` (compile-time only) | ❌ Wave 0 |
| COMPAT-01 | Each of 85 extern "C" functions exists with correct ABI signature | smoke (build) | `cargo build -p libxc_rs --release && nm target/release/liblibxc_rs.so \| grep -c 'T xc_'` ≥ 85 | ❌ Wave 0 |
| COMPAT-01 | Lifecycle round-trip: alloc → init → exc → end → init → exc → end → free | integration | `cargo test --test compat_smoke lifecycle_round_trip` | ❌ Wave 0 |
| COMPAT-01 | Each evaluate function (33 entries) callable via FFI signature, returns 0, fills output | integration (parametrized) | `cargo test --test compat_smoke evaluate_all_orders` | ❌ Wave 0 |
| COMPAT-01 | NULL output pointer skips that derivative (libxc parity) | integration | `cargo test --test compat_smoke null_skips_derivative` | ❌ Wave 0 |
| COMPAT-01 | Discovery functions match Phase-1 registry (`xc_number_of_functionals == 649`, `xc_functional_get_number("lda_x") == 1`) | integration | `cargo test --test compat_smoke discovery_matches_registry` | ❌ Wave 0 |
| COMPAT-01 | Hybrid functions return correct CAM coefficients for B3LYP / CAM-B3LYP (oracle parity) | integration | `cargo test --test compat_smoke hybrid_oracle_b3lyp` | ❌ Wave 0 |
| COMPAT-02 | Opaque struct compile-time `size_of == 0` | unit | `cargo test -p libxc_rs compat::c_layout::tests::opaque_size_zero` | ❌ Wave 0 |
| COMPAT-02 | `Family::Lda as i32 == XC_FAMILY_LDA == 1` and parallel for all enums/flags | unit | `cargo test -p libxc_rs compat::c_layout::tests::repr_constants_match_libxc` | ❌ Wave 0 |
| COMPAT-02 | C header compiles under `gcc -Wall -Werror -c xc.h` (header sanity) | smoke | `gcc -fsyntax-only -Wall -Werror include/xc.h` (manual or build script) | ❌ Wave 0 (may defer to Phase 7) |
| COMPAT-03 | No `unsafe` outside `compat/`, `kernel/launch.rs`, `kernel/buffer.rs` | static (grep) | `! find src -name '*.rs' -not -path '*/compat/*' -not -path '*/kernel/launch*' -not -path '*/kernel/buffer*' \| xargs grep -l 'unsafe '` | ❌ Wave 0 |
| Panic safety | Forced panic inside compat returns `LIBXC_RS_PANIC` errno, no UB | unit | `cargo test -p libxc_rs compat::macros::tests::catch_panic_returns_errno` | ❌ Wave 0 |
| Errno | `xc_rs_last_error_code()` returns last code; `xc_rs_last_error_message()` returns matching CString | integration | `cargo test --test compat_smoke errno_round_trip` | ❌ Wave 0 |
| Re-init | `xc_func_init(p, id1, ns) → xc_func_init(p, id2, ns)` correctly drops Functional 1 (no leak) | unit + miri | `cargo test -p libxc_rs compat::raw_handle::tests::reinit_drops_previous` (run under `cargo +nightly miri test` if available) | ❌ Wave 0 |
| Threshold aux propagation (Pitfall 4) | `xc_func_set_dens_threshold(b3lyp_p, 1e-12)` propagates to all 4 aux | unit | `cargo test -p libxc_rs functional::config::tests::threshold_propagates_to_aux` | ❌ Wave 0 (also adds Phase-5 fix) |
| `XC_EXT_PARAMS_DEFAULT` substitution (Pitfall 10) | `xc_func_set_ext_params` with -999998888 substitutes default | unit | `cargo test -p libxc_rs compat::legacy_eval::tests::ext_params_default_marker` | ❌ Wave 0 |

### Sampling Rate (Nyquist coverage)

The Nyquist principle: every externally-observable signal must have at least one test. The signals Phase 6 introduces are:

1. **85 extern "C" symbol exports** — covered by symbol-presence smoke test (single test parameterized across 85 names, OR a single `nm` grep count)
2. **5 lifecycle states** (Empty / Initialized / re-init overwrite / panic-during-init / NULL handle) — 5 unit tests in `compat::raw_handle`
3. **Each of the 33 evaluate functions** — covered by parametrized `compat_smoke::evaluate_all_orders` (one assert per function: callable, returns 0, fills zk in non-NULL case)
4. **NULL-handling for each evaluate output pointer** (33 functions × ~5 outputs each — but the same pattern; sample 3 representative functions per family covers the pattern uniformly)
5. **Each error code path** (BatchOverflow, FamilyMismatch, UninitializedHandle, Panicked, RemovedFunctionalId, UnknownFunctionalId, UnknownExtParamName, ExtParamCountMismatch, etc.) — one test per discriminant (~10 tests)
6. **Builder + EvaluateInput trait shapes** — bit-equivalence with direct Phase-5 calls (3 tests, one per family)
7. **Layout assertions** — compile-time (`const _:() = assert!`); failures are build failures, not test failures
8. **Thread-local errno round-trip** — one test that triggers an error, reads errno, confirms code + message match

- **Per task commit:** `cargo test -p libxc_rs api:: compat::` (~30s with sccache)
- **Per wave merge:** `cargo test --workspace --no-fail-fast` (~5-10min including verify/oracle tests)
- **Phase gate:** Full suite green + `cargo build -p libxc_rs --release && nm` symbol count check + `gcc -fsyntax-only include/xc.h` before `/gsd-verify-work`

### Wave 0 Gaps

- [ ] `src/api/builder.rs` body — currently 2-line placeholder
- [ ] `src/api/batch.rs` body — currently 2-line placeholder
- [ ] `src/api/evaluate.rs` — file doesn't exist; needed for sealed trait
- [ ] `src/api/mod.rs` update to add `pub mod evaluate;`
- [ ] `src/compat/{c_layout,raw_handle,ids,legacy_eval,removed}.rs` bodies — all 2-line placeholders
- [ ] `src/compat/{info,hybrid,library,errno,macros}.rs` — files don't exist; needed
- [ ] `src/compat/mod.rs` update to register new submodules
- [ ] `src/error/mod.rs` extension: `BatchOverflow`, `UninitializedHandle`, `Panicked`, `discriminant()` method
- [ ] `include/xc.h` — file doesn't exist; needed (~100 declarations, ~250 lines)
- [ ] `verify/tests/compat_smoke.rs` — file doesn't exist; needed for FFI integration
- [ ] Phase-5 `src/functional/config.rs` threshold setters — extend to walk `self.auxiliaries` (Pitfall 4 fix)
- [ ] `tests/compat_concurrent.rs` (optional; adds ThreadSanitizer coverage if Phase 7 wants it)

*(All gaps are net-new code; no existing test infrastructure modifications needed.)*

## Security Domain

> Required per default `security_enforcement` (no override in `.planning/config.json`). Phase 6 has a non-trivial threat surface because it crosses the Rust↔C trust boundary.

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | Library, no auth surface |
| V3 Session Management | no | Library, no sessions |
| V4 Access Control | no | Library, no access control |
| V5 Input Validation | **yes** | Buffer length checks (`Dimensions::lda/gga/mgga`), NULL checks, integer-to-enum validation (`Spin::try_from(nspin)`), ext_param count validation (Phase-5 already enforces) |
| V6 Cryptography | no | No crypto |
| V7 Error Handling | **yes** | `LibxcRsError` enum + thread-local errno; never `panic!` across FFI boundary; `catch_unwind` is mandatory |
| V10 Malicious Code | **yes** | All Rust-side `unsafe` confined to compat (BUILD-04, COMPAT-03) — auditable by `find … -path '!compat' \| xargs grep unsafe` |

### Known Threat Patterns for Rust ↔ C ABI

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Double-free of `xc_func_type*` | Tampering | `xc_func_free` on already-freed pointer is UB. Cannot fully prevent without ref-counting. **Mitigation:** document loudly in C header; tests include `valgrind`/`miri` round-trip. Match libxc's contract (libxc doesn't protect against this either). |
| NULL pointer dereference | Denial of Service | `extern_c_wrapper!` macro NULL-checks `*mut xc_func_type` at every entry point. NULL output pointers are interpreted as "skip" (D-A4-3). |
| Buffer overflow on output | Tampering | Buffer length is computed from `np` × `Dimensions::<field>(spin)`. Caller must pass correctly-sized buffers. **Mitigation:** the C ABI cannot enforce this (no length argument); document in header. Add `cargo +nightly miri` runs in CI for compat tests. |
| Integer overflow on `np * dim` | Tampering | `np: usize` × `dim: u8` cast to `usize`. For `np = usize::MAX / 2`, the multiplication can overflow. **Mitigation:** `usize::checked_mul` in slice construction; return `LibxcRsError::OutputBufferSizeMismatch` on overflow. |
| Panic across FFI boundary (UB pre-Rust 1.33; aborts post-1.33) | DoS / Information Disclosure | `catch_unwind` at every entry point per `extern_c_wrapper!`. Captured panic message goes to thread-local errno (does not leak to default `panic = unwind`'s stderr). |
| Use-after-free of returned `*const c_char` | Tampering / Information Disclosure | Thread-local `CString` cache lifetime is "until next call on this thread." **Mitigation:** documented in C header; matches libxc's `libxc_malloc + caller-frees` pattern with our spin on it. |
| Concurrent mutation of shared `xc_func_type*` | Tampering | CONTEXT D-A1-3: single-threaded per handle. **Mitigation:** documented in C header; matches libxc's de-facto contract. No runtime atomic guard (cost in hot path). |
| Re-init leaking previous Functional | Resource Exhaustion | `std::ptr::replace` (or read+drop+write) ensures the previous Functional drops on re-init. Test under valgrind/LeakSanitizer. |

## Open Questions

1. **Should `xc_functional_get_name` return a freed-by-caller string (libxc-compatible) or a thread-local cached pointer (libxc_rs convention)?**
   - What we know: libxc returns `libxc_malloc`'d strings; the magic constant `XC_EXT_PARAMS_DEFAULT` and the `xc_func_alloc/free` pair already establish a "Rust allocates, C frees via xc_*_free" pattern as feasible.
   - What's unclear: Whether existing C/Fortran integrators rely on the `free()`-able return value or just copy + ignore.
   - Recommendation: Thread-local cache (matches errno pattern, no 86th function), document loudly. Re-evaluate at Phase 6 verify time if a real C harness reveals friction.

2. **C-harness integration test: hand-rolled `cc`-built C file or pure-Rust FFI exercise?**
   - What we know: CONTEXT § Specifics suggests a "single Rust test that calls our extern "C" functions through their FFI signature" as minimum-viable.
   - What's unclear: Whether the planner will additionally want a 50-line C `.c` file compiled via `cc` build script (true header-compile test) for COMPAT-02 confidence.
   - Recommendation: Start with Rust-only FFI test (`verify/tests/compat_smoke.rs`); add `cc`-built C harness in a follow-on plan if header-compile confidence is needed. Both are valid.

3. **C header committed location: `include/xc.h`, `compat/include/xc.h`, or `target/include/xc.h`?**
   - What we know: PROJECT.md says "drop-in replacement"; users `#include "xc.h"`. CONTEXT mentions both options. No precedent in this repo (currently no `include/` directory).
   - What's unclear: Convention preference — `include/` at repo root is most discoverable.
   - Recommendation: `include/xc.h` at repo root. Easy to find; aligns with C convention; no build-time generation means it can ship in source tree.

4. **`removed.rs` errno code: dedicated `LIBXC_RS_REMOVED` or map to a richer payload via `RemovedFunctionalId`?**
   - What we know: Phase-1 already returns `LibxcRsError::RemovedFunctionalId { removed_id, replacement_id, replacement_name }` with the replacement payload.
   - What's unclear: Whether C callers care about the replacement_id or just want a "this was removed" signal.
   - Recommendation: Dedicated `LIBXC_RS_REMOVED_FUNCTIONAL_ID = -10` errno code; format the message string as "ID 104 removed; use 1 (XC_LDA_X) instead" in the thread-local message. Best of both worlds.

5. **Aux-propagation of threshold setters (Pitfall 4) — fix in Phase 5 surface or duplicate in compat?**
   - What we know: libxc's `set_dens_threshold` walks aux recursively; Phase-5 currently doesn't.
   - What's unclear: Whether anyone has shipped Layer-3 Rust code relying on the non-propagating behavior (likely no — Phase-5 is brand new and has no released downstream consumers).
   - Recommendation: Fix in Phase 5 surface (one line per setter). Single source of truth. Update Phase-5 unit tests to verify aux propagation.

6. **`xc_lda_new` / `xc_gga_new` exposure of `xc_*_out_params` struct — define in Rust `compat/c_layout.rs` or treat as opaque?**
   - What we know: These structs (`xc.h:196-246`) are part of the C ABI; C callers can construct them, fill in pointers, pass them. We must define them in the C header at minimum.
   - What's unclear: Whether to also `#[repr(C)]` define them in Rust for direct field access in `xc_lda_new` shim, or read field-by-field as raw pointers via offset.
   - Recommendation: `#[repr(C)]` define them in `compat/c_layout.rs` with all-pointer fields. Layout is well-specified (struct of `*mut f64`); `repr(C)` is required for any C ABI struct we receive by-pointer.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Rust toolchain | Phase 6 build | ✓ | rustc 1.95.0 (edition 2024 OK) | — |
| cargo | Phase 6 build | ✓ | 1.95.0 | — |
| sccache | `.cargo/config.toml` (build wrapper) | ✓ (referenced; assumed installed) | — | Disable in `.cargo/config.toml` if missing |
| cmake | libxc-sys verify-tier build | (assumed; Phase 5 already uses) | — | — |
| bindgen 0.72.1 | libxc-sys (verify-tier) | (workspace dep) | 0.72.1 | — |
| gcc | C-header sanity check (`gcc -fsyntax-only include/xc.h`) | (assumed on dev/CI hosts) | — | Skip header-compile test; rely on Rust unit tests |
| valgrind / miri (optional) | Memory-leak verification on lifecycle | optional | — | Skip leak-specific test; rely on Drop logic + code review |

**Missing dependencies with no fallback:** None blocking. All Phase 6 work uses Rust stdlib + existing workspace.

**Missing dependencies with fallback:** valgrind / miri are optional polish for the lifecycle test. Skip if not on the target machine.

## Sources

### Primary (HIGH confidence)

- `libxc-master/src/xc.h` (lines 1-607) — Authoritative ABI: 85 function declarations, struct definitions, `XC_*` constants. [VERIFIED]
- `libxc-master/src/functionals.c` (lines 25-510) — Lifecycle implementation: alloc / init / end / free, threshold setters with aux walk, ext_params with `XC_EXT_PARAMS_DEFAULT` substitution. [VERIFIED]
- `libxc-master/src/lda.c` (lines 100-235) — All 12 LDA evaluation entry points; NULL-skip semantics; "old API" `xc_lda` derivative-order inference. [VERIFIED]
- `libxc-master/src/hybrids.c` (lines 75-157) — `xc_hyb_type` classification logic; `xc_hyb_exx_coef` and `xc_hyb_cam_coef` semantics. [VERIFIED]
- `libxc-master/src/mix_func.c` (lines 310-333) — `xc_num_aux_funcs`, `xc_aux_func_ids`, `xc_aux_func_weights`. [VERIFIED]
- `libxc-master/src/func_info.c` (lines 1-75) — All 10 info accessors + 4 reference accessors (in companion file). [VERIFIED]
- `src/lib.rs`, `src/api/{mod,batch,builder}.rs`, `src/compat/{mod,c_layout,ids,legacy_eval,raw_handle,removed}.rs` — Existing scaffolding (placeholders). [VERIFIED by direct read]
- `src/functional/{mod,lifecycle,config,evaluate,hybrid,params,params_lda,params_gga,params_mgga}.rs` — Phase-5 surface; the wrappee. [VERIFIED]
- `src/eval/workspace.rs` — `EvaluationWorkspace` + LdaScratch/GgaScratch/MggaScratch (already materialized). [VERIFIED]
- `src/error/mod.rs` — Existing `LibxcRsError` enum (24 variants); `FamilyMismatch` already exists. [VERIFIED]
- `src/output/mod.rs`, `src/input/mod.rs` — Typed bundles with `Option<&mut [f64]>` per-derivative. [VERIFIED]
- `src/registry/mod.rs` — `lookup_by_id`, `lookup_by_name`, `all_functional_ids`, `version`, `version_string`. [VERIFIED]
- `Cargo.toml` (root + verify + libxc-sys + xtask) — Confirmed prod deps + workspace shape. [VERIFIED]
- `.planning/phases/06-public-api-and-c-compatibility/06-CONTEXT.md` — User-locked decisions A1-1..A4-4. [VERIFIED]
- `.planning/phases/05-functional-lifecycle-and-hybrid-properties/05-CONTEXT.md` — Phase-5 decisions Phase 6 builds on. [VERIFIED]
- `CLAUDE.md` — Project constraints. [VERIFIED]

### Secondary (MEDIUM confidence)

- [Rust panic::catch_unwind documentation](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html) — `UnwindSafe` requirements; AssertUnwindSafe wrapper. [CITED]
- [Rust CString documentation](https://doc.rust-lang.org/std/ffi/struct.CString.html) — Lifetime semantics; `into_raw`/`from_raw` ownership transfer. [CITED]
- [RFC 2945 (C-unwind ABI)](https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html) — Why `extern "C"` + catch_unwind is preferred over `extern "C-unwind"` for libraries that don't want runtime coupling. [CITED]
- [Internals discussion: Unwinding through FFI after Rust 1.33](https://internals.rust-lang.org/t/unwinding-through-ffi-after-rust-1-33/9521) — Historical context for why catch_unwind is mandatory at extern "C" boundaries. [CITED]
- [cbindgen documentation](https://github.com/mozilla/cbindgen/blob/master/docs.md) — Opaque types, `cbindgen:no-export`, repr/style options. Reviewed but not adopted. [CITED]
- `cargo search` output — current crates.io versions for typed-builder (0.23.2), derive_builder (0.20.2), static_assertions (1.1.0), cbindgen (0.29.2). [VERIFIED]

### Tertiary (LOW confidence)

- "Wrapping Unsafe C Libraries in Rust" Medium article — general FFI patterns; consistent with stdlib docs but not authoritative. [Reference only]

## Metadata

**Confidence breakdown:**
- ABI surface (85 functions, signatures, semantics): **HIGH** — every function verified by direct read of `libxc-master/src/xc.h` and the implementation files.
- Phase-5 wrappee (Functional, EvaluationWorkspace, error types): **HIGH** — every method signature verified by direct read.
- Opaque-type pattern, catch_unwind, errno mechanism: **HIGH** — stdlib + RFC primitives; well-documented.
- Builder/BatchEvaluator/EvaluateInput shape: **HIGH** — CONTEXT D-A1..D-A4 lock the design; pattern code is straightforward Rust.
- Pitfalls (especially threshold aux propagation, ext_params default, name lifetime): **HIGH** — derived from direct reading of libxc reference C code.
- Header generation strategy (hand-write vs cbindgen): **MEDIUM** — recommendation is a judgment call from CONTEXT § Specifics; cbindgen would also work.
- Threading semantics (single-thread per handle): **MEDIUM** — matches libxc's de-facto contract; cannot be runtime-enforced cheaply. Documented as caller responsibility per CONTEXT D-A1-3.

**Research date:** 2026-05-06
**Valid until:** 2026-06-06 (30 days; libxc 7.0.0 is a stable pin, Phase-5 surface frozen — no fast-moving dependencies).

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | The `xc_lda_funcs_variants` / `xc_gga_funcs_variants` / `xc_mgga_funcs_variants` typedefs in `xc.h:248-275` are libxc-internal (not part of the 85 entry points we must wrap) — they are function-pointer-table types, not callable functions. | Domain Context | If wrong: count may be off by 3 typedefs that aren't in the function set; signature parsing still finds 85 callable extern "C" functions. Verified count is 85, matches REQUIREMENTS COMPAT-01. **LOW risk.** |
| A2 | `xc_lda_new` / `xc_gga_new` are exposed as part of the new API and must be implemented (xc.h:424-427 marks them "New API"). MGGA does not have a corresponding `xc_mgga_new` in `xc.h`. | Library/ABI Surface | If wrong: we'd implement something not in the contract (harmless) or skip something we should have implemented (1-2 functions missing from the 85). Verified by `grep` count = 85. **LOW risk.** |
| A3 | The thread-local CString cache for `xc_functional_get_name` and similar string-returning functions is acceptable semantics for typical DFT-code C callers (they copy the string immediately). | Pitfall 2 | If wrong: a caller that holds the pointer across multiple calls gets corrupted data. Recommend documenting clearly in C header. **MEDIUM risk** if real-world caller expects libxc's `free()`-able semantics. |
| A4 | `Functional::set_density_threshold` etc. should be modified to walk `self.auxiliaries` for libxc parity (Pitfall 4 recommendation). | Pitfall 4 | If wrong: oracle parity tests on hybrid functionals with non-default thresholds will fail. Plan must include explicit Phase-5 setter modification. **MEDIUM risk** — exists today regardless of what we choose; just need to fix in one place. |
| A5 | Hand-writing the C header is preferred over cbindgen for this surface (~85 stable functions). | Standard Stack — Alternatives | If wrong: cbindgen would also work; switching adds the cbindgen dev-dep and a build script. **LOW risk** — both are valid. |
| A6 | `BatchEvaluator::new(spin, np_max)` doesn't need a `family` argument because the workspace sizes for MGGA-superset (Phase 3 D-12). | Pattern 4 | If wrong: workspace would need re-sizing for non-MGGA families. Phase 3 D-12 is documented and verified; re-sizing is unnecessary. **LOW risk.** |
| A7 | A single `cargo test --workspace` command (~5-10min) is sufficient as the phase-gate, given existing oracle tests cover the wrappee. | Validation Architecture | If wrong: may need to add a long-running integration tier. Existing verify/oracle tests already cover Phase-5 correctness; Phase-6 wraps don't add new oracle coverage. **LOW risk.** |

**Confirmation needed before plan execution:** A3 (thread-local cache lifetime), A4 (Phase-5 setter modification scope). Both should be raised in plan-discuss / Wave 0 review.

## RESEARCH COMPLETE

Phase 6 wraps Phase-5's `Functional` in two outer rings — Layer-3 ergonomic Rust API (`api::{builder, batch, evaluate}`) and Layer-1 C-ABI compat (`compat::*`). The 85 extern "C" functions cleave into 11 named groups, every signature verified directly against `libxc-master/src/xc.h`. CONTEXT.md locks all major design moves (opaque `xc_func_type` over `Box<FunctionalSlot>`, workspace-only `BatchEvaluator` with fixed `np_max`, sealed `EvaluateInput` trait with three impls, `int` errno + `catch_unwind` everywhere). Existing Phase-5 surface — `Functional::evaluate_{lda,gga,mgga}`, threshold and ext_param setters, hybrid/aux/NLC queries, `EvaluationWorkspace` with materialized scratch — provides every primitive Phase 6 needs to wrap. The plan is to hand-write everything (header, 85 entry points, builder), use one uniform `extern_c_wrapper!` macro for panic+errno, and ship a single `verify/tests/compat_smoke.rs` integration test that exercises the FFI surface end-to-end. Two libxc-parity gotchas surfaced that need a Phase-5 micro-fix: (1) threshold setters should propagate to auxiliaries; (2) `XC_EXT_PARAMS_DEFAULT = -999998888` magic substitution must happen in compat. Eight assumptions are flagged for confirmation before plan execution; the rest is locked by CONTEXT or verified against the libxc reference.
