# Phase 10: Workspace-Level Modular Split — Pattern Map

**Mapped:** 2026-05-07
**Phase:** 10 — Workspace-Level Modular Split
**Bias:** This is a **pure refactor**. Almost nothing is net-new code; the bulk of the work is `git mv` of existing trees plus three new `Cargo.toml` + `lib.rs` skeletons, a thin facade reduction at root, 7 xtask path-string edits, and (conditionally) a hand-written C header. The pattern map is therefore biased toward **structural analogs** for the 3 new crate skeletons and toward **mechanical edit recipes** for the rest.

**Files analyzed:**
- 3 new Cargo.toml (`crates/libxc-{core,eval,compat}/Cargo.toml`)
- 3 new lib.rs (`crates/libxc-{core,eval,compat}/src/lib.rs`)
- 1 modified root lib.rs (`src/lib.rs` → thin facade)
- 1 modified root Cargo.toml (`Cargo.toml` deps + members)
- 7 modified xtask path strings (4 in `xtask/src/main.rs`, 3 in `xtask/src/generate_metadata.rs`)
- 1 conditionally new C header (`crates/libxc-compat/include/xc_rs.h`)
- ~133 cross-module imports rewritten via mechanical pattern (described, not enumerated)

**Analogs found:** 11 / 11 distinct file roles (all have strong in-repo analogs).

---

## File Classification

| File / Edit | Role | Closest Analog | Match Quality |
|-------------|------|----------------|---------------|
| `crates/libxc-core/Cargo.toml` | rlib crate root (data + meta, no compute) | `crates/kernel-math/Cargo.toml` | role-match (both rlib, simple deps; ours has no cubecl) |
| `crates/libxc-eval/Cargo.toml` | rlib orchestration crate (depends on core + many kernel-* crates) | `crates/kernel-mgga/Cargo.toml` + root `Cargo.toml:6-14` | exact (both aggregate path-deps on kernel-* tree) |
| `crates/libxc-compat/Cargo.toml` | rlib + cdylib + staticlib FFI shim | **none — net-new shape** (no current crate declares cdylib) | structural pattern from CONTEXT D-07 specifics block |
| `crates/libxc-core/src/lib.rs` | rlib facade declaring child modules | `crates/kernel-math/src/lib.rs` | exact |
| `crates/libxc-eval/src/lib.rs` | rlib facade declaring child modules | `crates/kernel-math/src/lib.rs` | exact |
| `crates/libxc-compat/src/lib.rs` | rlib facade re-publishing the moved compat module tree | `src/compat/mod.rs` (current) | exact (same `pub mod` shape) |
| `src/lib.rs` (post-reduction) | **thin re-export facade** of three child crates | `src/eval/mod.rs:6-13` (in-repo "module aggregator" via `pub use`) and current `src/lib.rs:23-38` (today's re-export block) | partial (no current crate is a 100%-re-export facade; closest is `src/eval/mod.rs`) |
| `Cargo.toml` (root, post-edit) | thinned-out workspace root | current `Cargo.toml:6-14, 21-24, 25-196` | self-modification (analog is its own current shape) |
| `xtask/src/main.rs:291,329,355,387` | path-string edit (4 sites, identical pattern) | the lines themselves | self-pattern (mechanical s/src/crates\/libxc-core\/src/) |
| `xtask/src/generate_metadata.rs:445,595,643` | path-string edit (3 sites, identical pattern) | the lines themselves | self-pattern |
| `crates/libxc-compat/include/xc_rs.h` (conditional) | hand-written C declarations mirroring extern fns | `libxc-master/src/xc.h:1-80` (pre-image) + `src/compat/raw_handle.rs:67-149` (extern fn signatures to mirror) | **pre-image only** — no in-repo `.h` exists today (verified: `find . -name '*.h' -not -path './target/*' -not -path './libxc-master/*'` returned empty) |
| `~133 cross-module imports` | mechanical rewrite (`use crate::X` → `use libxc_core::X`) | `src/eval/gga_dispatch/batch14.rs:1-2`, `src/functional/evaluate.rs:1-4`, `src/eval/mgga_dispatch/mod.rs:1-5` | exact (all 105 cross-module imports follow one of three shapes) |

---

## A. New Cargo.toml files (3)

### `crates/libxc-core/Cargo.toml`

**Role:** rlib data-layer crate (no compute, no CubeCL).

**Closest analog:** `crates/kernel-math/Cargo.toml:1-13`

**Excerpt (verbatim):**
```toml
[package]
name = "libxc-kernel-math"
version = "0.1.0"
edition = "2024"

[dependencies]
cubecl = { version = "0.9.0", default-features = false, features = ["cpu"] }

[dev-dependencies]
approx = "0.5.1"
bytemuck = { version = "1.25.0", features = ["derive"] }
libm = "0.2"
```

**Notes for planner:**
- **Mirror:** `[package]` block (name, version, edition 2024) verbatim, just change `name = "libxc-core"`.
- **Diverge — drop `cubecl`:** RESEARCH §2 dependency partition (lines 84-93) and CONTEXT D-criterion 2 say libxc-core has zero cubecl deps. Verification: `cargo tree -p libxc-core --depth 1 | grep -E "cubecl|libxc-kernel"` MUST be empty.
- **Add the three deps that move from root `Cargo.toml:6-10`** that libxc-core actually consumes:
  - `bitflags = "2.10.0"` — used by `model::FunctionalFlags` and `output::OutputMask`.
  - `bytemuck = { version = "1.25.0", features = ["derive"] }` — used by Pod/Zeroable derives on input/output structs (RESEARCH §4 notes zero current derives, but bytemuck is also re-imported via path through libxc-eval; safe to keep here for input/output types).
  - `thiserror = "2.0.18"` — required by `LibxcRsError` (already imported via `#[derive(thiserror::Error)]` at `src/error/mod.rs:3`).
- **Convention:** Use **literal version strings**, not `workspace = true`. RESEARCH §2 line 79: "**No `[workspace.dependencies]` table is in use.** Each leaf crate declares its dep with a literal version string ... Don't introduce `workspace = true` patterns in this phase — that's scope creep."
- **Dev-deps:** None required for libxc-core (existing test modules inside `src/error/`, `src/model/`, etc. don't pull dev-deps today; verify with `cargo check -p libxc-core --tests`).

---

### `crates/libxc-eval/Cargo.toml`

**Role:** rlib orchestration crate (depends on libxc-core + the 4 kernel-* aggregator crates).

**Closest analog:** Root `Cargo.toml:6-14` (today's `[dependencies]` block — this **IS** what the libxc-eval Cargo.toml's deps block becomes after partitioning).

**Excerpt (verbatim) — root Cargo.toml lines 6-14:**
```toml
[dependencies]
bitflags = "2.10.0"
bytemuck = { version = "1.25.0", features = ["derive"] }
cubecl = { version = "0.9.0", default-features = false, features = ["cpu"] }
thiserror = "2.0.18"
libxc-kernel-math = { path = "crates/kernel-math" }
libxc-kernel-lda = { path = "crates/kernel-lda" }
libxc-kernel-gga = { path = "crates/kernel-gga" }
libxc-kernel-mgga = { path = "crates/kernel-mgga" }
```

**Secondary structural analog (kernel-* aggregator shape):** `crates/kernel-mgga/Cargo.toml:1-15` shows the path-dep pattern for the kernel- aggregator crates. libxc-eval is structurally similar but at one level higher.

**Notes for planner:**
- **Mirror the path-deps for the four kernel aggregators** verbatim (`libxc-kernel-{math,lda,gga,mgga}`); their paths from `crates/libxc-eval/` are `../kernel-math`, `../kernel-lda`, etc. (one extra `../` because libxc-eval is at `crates/libxc-eval/` not the workspace root).
- **Add the new path-dep:** `libxc-core = { path = "../libxc-core" }` (consumed via `use libxc_core::error::LibxcRsError;` etc.).
- **Drop `bitflags` + `thiserror`:** per RESEARCH §2 partition table — neither is used inside libxc-eval directly (LibxcRsError lives in libxc-core; eval uses it via the path-dep, which transitively brings thiserror's runtime trait impl).
- **Keep `cubecl` and `bytemuck`:** RESEARCH §2 marks both YES for libxc-eval (cubecl: `kernel/launch.rs` uses `ComputeClient`; bytemuck: `bytemuck::cast_slice` invocation in `kernel/launch.rs`).
- **Trap:** the existing `[dev-dependencies]` block at root (`Cargo.toml:16-19`) — `approx`, `libm`, `libxc_rs-verify` — does NOT move into libxc-eval. Those test deps stay at root because they're for root's own integration tests. libxc-eval's per-module `#[cfg(test)] mod tests` likely needs `approx` if any moved test asserts numerics; planner verifies with `cargo check -p libxc-eval --tests` and adds dev-deps as warnings surface.

---

### `crates/libxc-compat/Cargo.toml`

**Role:** rlib + cdylib + staticlib FFI shim. **Net-new crate-type shape** — no other crate in this workspace declares cdylib today.

**Closest analog:** None in repo. Use the exact shape pinned by **CONTEXT.md specifics block lines 209-223** (D-07 reference Cargo.toml).

**Excerpt (verbatim from CONTEXT D-07):**
```toml
[package]
name = "libxc-compat"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["rlib", "cdylib", "staticlib"]
# default name = "libxc_rs" (no override) → libxc_rs.so / libxc_rs.a / libxc_rs.rlib

[dependencies]
libxc-core = { path = "../libxc-core" }
libxc-eval = { path = "../libxc-eval" }
thiserror = "2.0"
```

**Notes for planner:**
- **No `[lib] name = "..."` override** (D-08). Default name = the package name with `-` → `_`, i.e. `libxc_compat`. **WAIT — re-read D-08: "cdylib name = `libxc_rs` (Rust default — no `[lib] name = "..."` override)."** That phrasing is ambiguous because Rust's actual default is the package name (`libxc-compat` → `libxc_compat`), NOT `libxc_rs`. **PLANNER MUST RECONCILE:** to get `libxc_rs.so`, `[lib] name = "libxc_rs"` override IS required, despite D-08's "no override" phrasing. Cross-check with CONTEXT line 27: "default name `libxc_rs` (so output is `libxc_rs.so` / `libxc_rs.a`)" — strongly implies they want override `name = "libxc_rs"`. **Recommend planner add `[lib] name = "libxc_rs"` and flag this in plan 10-03 as a reconciliation note.**
- **Add `thiserror = "2.0.18"` to match** the version pin elsewhere in the workspace (CONTEXT specifics block writes `thiserror = "2.0"` but root `Cargo.toml:10` pins `2.0.18` — match the root pin for consistency).
- **No dev-deps** mentioned in D-07 spec; existing `#[cfg(test)] mod tests` inside `src/compat/macros.rs:51-74` and `src/compat/raw_handle.rs:177-332` use only `std::ffi::CStr` and `crate::*` — no external test crate needed. Planner confirms with `cargo check -p libxc-compat --tests`.
- **Bytemuck NOT needed here** (RESEARCH §2 partition table — libxc-compat row, all `—`).

---

## B. New lib.rs files (3)

### `crates/libxc-core/src/lib.rs`

**Role:** Module aggregator for the data layer.

**Closest analog:** `crates/kernel-math/src/lib.rs:1-21`

**Excerpt (verbatim):**
```rust
#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]

pub mod constants;
pub mod powers;
pub mod piecewise;
pub mod polynomials;
pub mod erf;
pub mod spin;
pub mod dft_quantities;
pub mod bspline;
pub mod lambert_w;
pub mod expint_e1;
pub mod special;
pub mod integrate;
pub mod br89;
pub mod mbrxc;
```

**Secondary analog for the `#![deny(warnings)]` line:** `src/lib.rs:1` (current root sets `#![deny(warnings)]`).

**Notes for planner:**
- **Mirror the structure exactly**, but use the libxc-core module list from CONTEXT.md `### What Phase 10 Creates` lines 175-177:
  ```rust
  #![deny(warnings)]
  #![allow(clippy::excessive_precision)]
  #![allow(clippy::needless_late_init)]
  #![allow(clippy::too_many_arguments)]

  pub mod model;
  pub mod meta;
  pub mod registry;
  pub mod input;
  pub mod output;
  pub mod layout;
  pub mod dims;
  pub mod error;
  ```
- **CRITICAL — R1 from RESEARCH §Plan Sequencing & Risk:** add a `pub use` re-export for `PROPAGATION_RULES` so `libxc-eval`'s `lifecycle.rs` can resolve it across the crate boundary. Either:
  - **(recommended)** in `crates/libxc-core/src/meta/mod.rs`, change line 3 from `pub(crate) mod generated_propagation;` and add at the bottom: `pub use generated_propagation::{PropagationRule, PROPAGATION_RULES};`
  - or widen line 3 to `pub mod generated_propagation;`
- **`#![deny(warnings)]` carries through verbatim from `src/lib.rs:1`.** Both `kernel-math/src/lib.rs` and `kernel-lda/src/lib.rs` opt **out** of deny(warnings) by NOT setting it (they only have `#![allow(...)]`); this is acceptable. RESEARCH §6 line 141 says "Each new leaf crate's `lib.rs` should mirror this — same line at top, plus the same three `#![allow(clippy::...)]` exemptions copied from current root." **Follow RESEARCH §6, not the kernel-math precedent.**
- **`#![allow(non_snake_case)]` and `#![allow(unused_assignments)]`** from kernel-math/lib.rs are **NOT needed** in libxc-core (those are CubeCL `#[cube]`-expansion warnings; libxc-core has zero CubeCL).

---

### `crates/libxc-eval/src/lib.rs`

**Role:** Module aggregator for the orchestration layer.

**Closest analog:** `crates/kernel-math/src/lib.rs:1-21` (same shape as libxc-core).

**Notes for planner:**
- Module list per CONTEXT.md `### What Phase 10 Creates` line 179:
  ```rust
  #![deny(warnings)]
  #![allow(clippy::excessive_precision)]
  #![allow(clippy::needless_late_init)]
  #![allow(clippy::too_many_arguments)]

  pub mod eval;
  pub mod functional;
  pub mod kernel;
  pub mod workspace;
  ```
- **Note:** today's `src/eval/mod.rs:1-13` includes `pub mod workspace;` *as a sub-module of eval*, AND there's a separate top-level `src/workspace/` (a placeholder per `src/workspace/mod.rs:1`). The planner must verify whether **both** workspaces survive the move or whether they're merged. **Confirmed via read:** `src/eval/mod.rs:5` is `pub mod workspace;` (eval's own scratch-buffer workspace) and `src/workspace/` at top level is the separate planner/host/scratch_map module. Both are in libxc-eval's domain per CONTEXT line 12 ("`eval/`, `functional/`, `kernel/` glue, `workspace/`") so both move; the top-level `pub mod workspace;` in libxc-eval/lib.rs refers to the **outer** `src/workspace/` tree.

---

### `crates/libxc-compat/src/lib.rs`

**Role:** Re-publishes the moved compat-module tree through the cdylib's crate root (so `#[macro_export]` macros and `extern "C"` symbols sit at the cdylib's symbol table top).

**Closest analog:** `src/compat/mod.rs:1-10` (current — the file becomes `crates/libxc-compat/src/lib.rs`'s body, with one tweak).

**Excerpt (verbatim):**
```rust
//! C-ABI compatibility layer for libxc_rs (phase 6 plan 02a/02b/03).

pub mod c_layout;
pub mod errno;
pub mod ids;
pub mod legacy_eval;
pub mod macros;
pub mod raw_handle;
pub mod removed;
```

**Notes for planner:**
- **Add `#![deny(warnings)]` + the three `#![allow(clippy::...)]` exemptions** at the top (same as libxc-core/libxc-eval). RESEARCH §6 line 141.
- **Mirror the `pub mod` list verbatim** — the 8 child modules already exist in `src/compat/` (verified via `ls src/compat/`: `c_layout.rs errno.rs ids.rs legacy_eval.rs macros.rs mod.rs raw_handle.rs removed.rs`). After `git mv src/compat/* crates/libxc-compat/src/`, the original `mod.rs` becomes the new `lib.rs` (rename `mod.rs` → `lib.rs` after move) — or delete `mod.rs` and create a fresh `lib.rs` with the deny+allow lines + the 8 `pub mod` declarations. Both options work; the second is cleaner because it doesn't lose the module declarations and the `git mv mod.rs lib.rs` preserves blame.
- **`#[macro_export] macro_rules! extern_c_wrapper` survives the move at `crates/libxc-compat/src/macros.rs:9-21`** with no rewrite. RESEARCH §Plan Sequencing R3 line 295: "After move that resolves to `libxc_compat::extern_c_wrapper` — works because `#[macro_export]` exports at the crate root."
- **However:** the macro body (`src/compat/macros.rs:11-49`) references `$crate::compat::errno::LIBXC_RS_NULL_HANDLE`, `$crate::LibxcRsError`, etc. After the move, `$crate` resolves to `libxc_compat`, but `compat::errno::*` no longer exists at that path — it's now `errno::*` (the `compat::` prefix dropped because the whole compat tree IS the libxc-compat crate root). **PLANNER MUST REWRITE the macro body's `$crate::compat::errno::*` → `$crate::errno::*` and `$crate::LibxcRsError` → `libxc_core::error::LibxcRsError`** (the typed error enum lives in libxc-core post-D-01). Sample diff:
  ```rust
  // Before (src/compat/macros.rs:12-13):
  $crate::compat::errno::set_error(
      $crate::compat::errno::LIBXC_RS_NULL_HANDLE,
  // After (crates/libxc-compat/src/macros.rs:12-13):
  $crate::errno::set_error(
      $crate::errno::LIBXC_RS_NULL_HANDLE,
  ```
  Same fix at lines 16, 33, 35 (`$crate::compat::errno::*` 4 occurrences total) and at lines 28, 38 (`$crate::LibxcRsError` 2 occurrences) — use `libxc_core::error::LibxcRsError` instead, or expose it as `pub use libxc_core::error::LibxcRsError;` near the top of `crates/libxc-compat/src/lib.rs` so `$crate::LibxcRsError` keeps resolving.
- **Internal callers of the macro:** `src/compat/raw_handle.rs:13` is `use crate::extern_c_wrapper;` — survives unchanged because after move, `crate::extern_c_wrapper` resolves at the new crate root (the macro is `#[macro_export]`, so it's published at `libxc_compat::` and `crate::` inside libxc-compat IS `libxc_compat::`).

---

## C. Modified files

### `src/lib.rs` (root — reduced to thin facade)

**Role:** Curated re-export facade preserving today's public surface.

**Closest analog (in-repo):** `src/eval/mod.rs:1-19` — the only file in this codebase that is bulk `pub use` re-exports of submodules/child crates. The current root `src/lib.rs:23-38` re-export block is also a self-analog for the *flat* re-exports.

**Excerpt 1 — the existing eval aggregator pattern (`src/eval/mod.rs:1-19`):**
```rust
pub mod dispatch;
pub mod gga_dispatch;
pub mod mgga_dispatch;
pub mod mix;
pub mod workspace;
pub use dispatch::dispatch_lda;
pub use gga_dispatch::dispatch_gga;
pub use mgga_dispatch::dispatch_mgga;
pub use mix::{
    add_to_mix, evaluate_mixed_gga, evaluate_mixed_lda, evaluate_mixed_lda_functional,
    evaluate_mixed_mgga, AuxiliaryConfig,
};
pub use workspace::EvaluationWorkspace;

// Alias kept for backward compat with verify/tests/lda_oracle.rs and other
// external callers that imported `LdaFunctionalParams` from the old
// dispatch module. New code should reference `LdaXParams` directly from
// `crate::functional::params_lda::LdaXParams`.
pub use crate::functional::params_lda::LdaXParams as LdaFunctionalParams;
```

**Excerpt 2 — current root re-export list to preserve (`src/lib.rs:23-38`):**
```rust
pub use model::{
    Family, Kind, Spin, DerivativeOrder, FunctionalId, FunctionalFlags,
    HybridType, HybridTermKind, Dimensionality, Thresholds,
    LdaFunctional, GgaFunctional, MggaFunctional,
};
pub use meta::{FunctionalMeta, Reference, ExtParamSpec, HybridTerm};
pub use error::LibxcRsError;
pub use dims::Dimensions;
pub use registry::{lookup_by_id, lookup_by_name, functional_count, version, version_string};
pub use input::{LdaInput, GgaInput, MggaInput};
pub use output::{LdaOutput, GgaOutput, MggaOutput, OutputMask};
pub use eval::{dispatch_lda, dispatch_gga, dispatch_mgga};
pub use functional::{
    classify_hybrid, CamCoefficients, Functional, FunctionalParams, NlcCoefficients, NoParams,
};
pub use api::{BatchEvaluator, EvaluateInput, FunctionalBuilder};
```

**Notes for planner:**
- **Use RESEARCH §7's "shape (c) split-by-module" verbatim** (RESEARCH lines 174-211). It is the only shape that preserves both the namespace-shape paths (`libxc_rs::math::constants::*`, `libxc_rs::eval::dispatch_lda`) AND the flat top-level re-exports.
- **Specific path requirements** (each path used externally per RESEARCH §7 survey lines 152-170):
  | External path | Source crate after split |
  |---|---|
  | `libxc_rs::LibxcRsError` | libxc-core |
  | `libxc_rs::eval::EvaluationWorkspace` | libxc-eval |
  | `libxc_rs::eval::{dispatch_gga, dispatch_lda, dispatch_mgga}` | libxc-eval |
  | `libxc_rs::eval::LdaFunctionalParams` | libxc-eval (re-export of `libxc-eval::functional::params_lda::LdaXParams`) |
  | `libxc_rs::functional::Functional` | libxc-eval |
  | `libxc_rs::functional::classify_hybrid` | libxc-eval |
  | `libxc_rs::input::{LdaInput, GgaInput, MggaInput}` | libxc-core |
  | `libxc_rs::math::constants::{KF_CONST, RS_CONST}` | **libxc-kernel-math** (via `pub mod math { pub use libxc_kernel_math::*; }`) |
  | `libxc_rs::math::{dft_quantities, erf, powers, spin}::*` | libxc-kernel-math |
  | `libxc_rs::meta::{ExtParamSpec, FunctionalMeta, HybridTerm, Reference}` | libxc-core |
  | `libxc_rs::model::{Spin, FunctionalId, ...}` (12 names) | libxc-core |
  | `libxc_rs::output::{LdaOutput, GgaOutput, MggaOutput, OutputMask}` | libxc-core |
  | `libxc_rs::registry::{lookup_by_id, lookup_by_name, all_functional_ids}` | libxc-core |
- **R2 trap:** the `pub mod math { pub use libxc_kernel_math::*; }` re-export is required because `tests/math_integration.rs` uses `libxc_rs::math::*` paths (RESEARCH §7 line 162-166), but `src/math/mod.rs` is being deleted under D-02. Easy to forget — **must be in the new root lib.rs**. Today's `src/math/mod.rs:1-12` is exactly this re-export pattern (`pub use libxc_kernel_math::constants;` x12), so the planner can copy that file's body into `pub mod math { ... }` block as-is.
- **Drop these `pub mod` declarations** from current `src/lib.rs:9-21`: `model, meta, error, dims, registry, kernel, input, output, eval, functional, compat`. Replace with `pub mod model { pub use libxc_core::model::*; }` etc. **Keep:** `pub mod api;` (root still owns `src/api/`).
- **Drop `pub mod compat;`** entirely — libxc-compat is the cdylib and root must not depend on it (success criterion 4, RESEARCH §2 partition table line 93).
- **Drop `pub mod kernel;`** — it's a re-export shim (`src/kernel/mod.rs:1-3`: `pub use libxc_kernel_lda as lda;` etc.); decide whether to keep it through root or let downstream consumers go direct to `libxc_kernel_lda`. RESEARCH §7's external-survey grep returned no hits on `libxc_rs::kernel::*`, so it's safe to drop. Planner verifies.

---

### `Cargo.toml` (root — workspace + thinned deps)

**Role:** Workspace declaration + thin facade deps.

**Closest analog (self-modification):** Current root `Cargo.toml` itself.

**Excerpts to modify:**

**Lines 6-14 (`[dependencies]` block) — partition per RESEARCH §2 lines 84-93:**
```toml
[dependencies]
bitflags = "2.10.0"
bytemuck = { version = "1.25.0", features = ["derive"] }
cubecl = { version = "0.9.0", default-features = false, features = ["cpu"] }
thiserror = "2.0.18"
libxc-kernel-math = { path = "crates/kernel-math" }
libxc-kernel-lda = { path = "crates/kernel-lda" }
libxc-kernel-gga = { path = "crates/kernel-gga" }
libxc-kernel-mgga = { path = "crates/kernel-mgga" }
```

**Replace with (post-split):**
```toml
[dependencies]
libxc-core = { path = "crates/libxc-core" }
libxc-eval = { path = "crates/libxc-eval" }
# libxc-kernel-math is needed by the root's `pub mod math { pub use libxc_kernel_math::*; }`
# re-export shim that preserves tests/math_integration.rs paths (D-02a, RESEARCH §7).
libxc-kernel-math = { path = "crates/kernel-math" }
# Note: NO libxc-compat — success criterion 4 requires that nothing depends on libxc-compat
# except the cdylib output itself.
```

**Lines 21-24 (`[workspace] members`):**
```toml
[workspace]
members=[    "xtask",
    "verify",
    "libxc-sys",]
```

**Replace with:**
```toml
[workspace]
members=[
    "xtask",
    "verify",
    "libxc-sys",
    "crates/libxc-core",
    "crates/libxc-eval",
    "crates/libxc-compat",
]
```

**Lines 25-196 (`default-members`):** APPEND the three new crate paths to the existing kernel-* enumeration. Per CONTEXT discretion line 81: "Add `crates/libxc-core`, `crates/libxc-eval`, `crates/libxc-compat`. Optional cleanup: collapse the kernel-* enumeration via `crates/kernel-*` glob (Cargo doesn't support glob in default-members today — workaround is the explicit list, so leave alone)."

**Lines 197-234 (resolver, profile blocks):** **DO NOT TOUCH.** RESEARCH §Open Questions §7 line 370: "Workspace inheritance: `[profile.*]` blocks at workspace root apply to ALL members. Confirmed by Cargo docs ... New leaf crates need NO profile blocks."

---

### `xtask/src/main.rs` — 4 path-string edits

**Role:** Mechanical s/src/crates\/libxc-core\/src/.

**Closest analog:** The lines themselves.

**Excerpts (verbatim):**

```
xtask/src/main.rs:291:    let path = root.join("src/meta/generated.rs");
xtask/src/main.rs:329:    let path = root.join("src/registry/by_id.rs");
xtask/src/main.rs:355:    let path = root.join("src/registry/by_name.rs");
xtask/src/main.rs:387:    let path = root.join("src/registry/removed.rs");
```

**Replacement:** prefix each path with `crates/libxc-core/`:

```
xtask/src/main.rs:291:    let path = root.join("crates/libxc-core/src/meta/generated.rs");
xtask/src/main.rs:329:    let path = root.join("crates/libxc-core/src/registry/by_id.rs");
xtask/src/main.rs:355:    let path = root.join("crates/libxc-core/src/registry/by_name.rs");
xtask/src/main.rs:387:    let path = root.join("crates/libxc-core/src/registry/removed.rs");
```

**Notes for planner:**
- **No xtask logic change.** D-06 / RESEARCH §xtask Verification Recipe (lines 336-337): the emitted `use crate::model::...` and `use crate::meta::...` headers in the generated files (verified at `xtask/src/main.rs:296-297` and `xtask/src/generate_metadata.rs:454-455`) resolve correctly inside libxc-core's source tree because `crate::` refers to the containing crate (libxc-core), and `model`/`meta` are sibling modules in libxc-core after the move.
- **Verification recipe** (RESEARCH §xtask Verification Recipe lines 304-333): byte-equivalent diff of pre/post regeneration — should produce zero diff if path edits are correct.
- **`find_workspace_root()` (`xtask/src/main.rs:280-288`) is unchanged.** D-06a: `if dir.join("libxc-master").exists() { return Ok(dir); }` — still finds the workspace root the same way.

---

### `xtask/src/generate_metadata.rs` — 3 path-string edits

**Role:** Same as above.

**Excerpts (verbatim):**

```
xtask/src/generate_metadata.rs:445:    let path = root.join("src/meta/generated.rs");
xtask/src/generate_metadata.rs:595:    let path = root.join("src/meta/generated_hybrid.rs");
xtask/src/generate_metadata.rs:643:    let path = root.join("src/meta/generated_propagation.rs");
```

**Replacement:** same prefix:

```
xtask/src/generate_metadata.rs:445:    let path = root.join("crates/libxc-core/src/meta/generated.rs");
xtask/src/generate_metadata.rs:595:    let path = root.join("crates/libxc-core/src/meta/generated_hybrid.rs");
xtask/src/generate_metadata.rs:643:    let path = root.join("crates/libxc-core/src/meta/generated_propagation.rs");
```

**Notes for planner:**
- The `super::PropagationRule` reference emitted at `xtask/src/generate_metadata.rs:653` (verified) resolves correctly inside the moved file because `super::` is `libxc-core::meta`, where `PropagationRule` is defined at `src/meta/mod.rs:41`. **No xtask emission change needed.**
- The emitted `use crate::model::FunctionalId;` (line 654) likewise resolves inside libxc-core.

---

### `src/main.rs` — optional deletion (planner discretion)

**Role:** Vestigial 3-line "Hello, world!" binary.

**Excerpt (full file, `src/main.rs:1-3`):**
```rust
fn main() {
    println!("Hello, world!");
}
```

**Notes for planner:** CONTEXT discretion item line 82. RESEARCH §Open Questions item 1 line 358: "Recommend deletion in plan 10-03 task 0 cleanup; no compile/test impact either way." If kept, it produces a `libxc_rs` binary alongside the rlib facade — cosmetic only.

---

## D. C header (`crates/libxc-compat/include/xc_rs.h`)

**Role:** Hand-written C header mirroring the `extern "C"` surface, **conditional on Phase 6 not having shipped one** (D-09a).

**Closest analog in-repo:** **None — no `.h` file is committed today.** Verified:

```
$ find /home/user/Documents/workspace/libxc_rs -name '*.h' \
    -not -path '*/target/*' -not -path '*/libxc-master/*' -not -path '*/.git/*'
(empty output)
```

**Pre-image (per D-09):** `libxc-master/src/xc.h` (607 lines).

**Excerpt 1 — `libxc-master/src/xc.h:1-30` (top-level shape to mirror):**
```c
/*
 Copyright (C) 2006-2007 M.A.L. Marques

 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

#ifndef _XC_H
#define _XC_H

#ifdef __cplusplus
extern "C" {
#endif

/* Get the literature reference for libxc */
const char *xc_reference();
/* Get the doi for the literature reference for libxc */
const char *xc_reference_doi();
/* Get the key for the literature reference for libxc */
const char *xc_reference_key();

/* Get the major, minor, and micro version of libxc */
void xc_version(int *major, int *minor, int *micro);
/* Get the version of libxc as a string */
const char *xc_version_string();

#include <xc_version.h>
#include <stddef.h>

#define XC_UNPOLARIZED          1
#define XC_POLARIZED            2
```

**Excerpt 2 — `libxc-master/src/xc.h:42-70` (constants the header must include):**
```c
#define XC_FAMILY_UNKNOWN      -1
#define XC_FAMILY_LDA           1
#define XC_FAMILY_GGA           2
#define XC_FAMILY_MGGA          4
#define XC_FAMILY_LCA           8
#define XC_FAMILY_OEP          16

/* flags that can be used in info.flags. ... */
#define XC_FLAGS_HAVE_EXC         (1 <<  0) /*     1 */
#define XC_FLAGS_HAVE_VXC         (1 <<  1) /*     2 */
#define XC_FLAGS_HAVE_FXC         (1 <<  2) /*     4 */
#define XC_FLAGS_HAVE_KXC         (1 <<  3) /*     8 */
...
```

**Excerpt 3 — Rust-side extern fn signatures the header declares (`src/compat/raw_handle.rs:67,79,112,129,149`):**
```rust
pub extern "C" fn xc_func_alloc() -> *mut xc_func_type;
pub unsafe extern "C" fn xc_func_init(p: *mut xc_func_type, functional: i32, nspin: i32) -> i32;
pub unsafe extern "C" fn xc_func_end(p: *mut xc_func_type) -> i32;
pub unsafe extern "C" fn xc_func_free(p: *mut xc_func_type);
pub unsafe extern "C" fn xc_func_get_info(p: *const xc_func_type) -> *const xc_func_info_type;
```

**Notes for planner:**
- **Per D-09a, verify Phase 6 status at plan time before deciding to write this file.** RESEARCH §Open Questions item 3 line 362: "no header file is present in the repo [VERIFIED via `find . -name "xc_rs.h" -o -name "xc.h"` returning only libxc-master sources]. Phase 6's plan 06-03 owns the header." Re-run the find at plan execution time.
- **Required deviations from libxc xc.h** (per CONTEXT D-08 / Phase 6 D-A4-1):
  - libxc's `void xc_func_end(...)` → ours is `int xc_func_end(...)` (signature widened to allow errno reporting).
  - Same widening for any `void` extern Phase 6 widened to `int`.
  - Rename guard `_XC_H` → `_XC_RS_H` (avoid collision when both headers are on a system).
  - **No `#include <xc_version.h>`** — drop or stub; we don't ship that file.
- **`xc_func_type` and `xc_func_info_type`:** opaque forward declarations only (`typedef struct xc_func_type xc_func_type;`). Per `src/compat/c_layout.rs` (forward decls in Rust are `#[repr(C)] pub struct xc_func_type { _private: [u8; 0] }` per Phase 6 D-A1-2; in C they're `struct xc_func_type;`).
- **Add `xc_rs_last_error_code()` and `xc_rs_last_error_message()` declarations** — these are libxc_rs-specific extension entry points (not in libxc xc.h). Their signatures live at `src/compat/errno.rs` (read full file at plan time to get exact signatures).
- **Total declaration count:** ~85-100 (CONTEXT line 28 says ~85 entry points; D-09 line 71 says ~100 declarations). Manageable for a single hand-written file.
- **File location finalized at `crates/libxc-compat/include/xc_rs.h`** per D-09 / D-09a. Renamed `xc.h → xc_rs.h` to avoid collision with libxc's own `xc.h`.

---

## E. Bulk import rewrites (~133 callsites)

**Role:** Mechanical `use crate::X` → `use libxc_core::X` rewrite across the moved files.

**Scope (from RESEARCH §1 line 69 + verified):** ~133 cross-module imports across `eval/`, `functional/`, `kernel/`, `workspace/`, `api/`, `compat/`. Re-verified during pattern mapping:
- `grep -rE "^use crate::(error|model|meta|dims|input|output|registry|layout)" src/{eval,functional,kernel,workspace,api,compat}/ | wc -l` → **105 lines**
- `grep -rE "^use crate::" src/{eval,functional,kernel,workspace,api,compat}/ | wc -l` → **142 lines** (the difference is intra-libxc-eval imports like `use crate::functional::*`, `use crate::eval::*`, `use crate::kernel::*` which remain `crate::` because they resolve inside libxc-eval).

**The mechanical pattern (3 representative excerpts):**

**Excerpt 1 — typical batch dispatcher (`src/eval/gga_dispatch/batch14.rs:1-2`):**
```rust
use crate::error::LibxcRsError;
use crate::model::{DerivativeOrder, Spin};
```

**After move into libxc-eval:**
```rust
use libxc_core::error::LibxcRsError;
use libxc_core::model::{DerivativeOrder, Spin};
```

**Excerpt 2 — multi-module consumer (`src/functional/evaluate.rs:1-4`):**
```rust
use crate::error::LibxcRsError;
use crate::input::{GgaInput, LdaInput, MggaInput};
use crate::model::{DerivativeOrder, GgaFunctional, LdaFunctional, MggaFunctional};
use crate::output::{GgaOutput, LdaOutput, MggaOutput};
```

**After move:**
```rust
use libxc_core::error::LibxcRsError;
use libxc_core::input::{GgaInput, LdaInput, MggaInput};
use libxc_core::model::{DerivativeOrder, GgaFunctional, LdaFunctional, MggaFunctional};
use libxc_core::output::{GgaOutput, LdaOutput, MggaOutput};
```

**Excerpt 3 — dispatch entry-point (`src/eval/mgga_dispatch/mod.rs:1-5`):**
```rust
use crate::dims::Dimensions;
use crate::error::LibxcRsError;
use crate::input::MggaInput;
use crate::model::{DerivativeOrder, MggaFunctional, Spin, Thresholds};
use crate::output::MggaOutput;
```

**After move:**
```rust
use libxc_core::dims::Dimensions;
use libxc_core::error::LibxcRsError;
use libxc_core::input::MggaInput;
use libxc_core::model::{DerivativeOrder, MggaFunctional, Spin, Thresholds};
use libxc_core::output::MggaOutput;
```

**Mechanical recipe (single sed-friendly transform that catches all 105 lines):**

For files moved into `crates/libxc-eval/src/` and `crates/libxc-compat/src/`:
```
s|^use crate::error::|use libxc_core::error::|
s|^use crate::model::|use libxc_core::model::|
s|^use crate::meta::|use libxc_core::meta::|
s|^use crate::dims::|use libxc_core::dims::|
s|^use crate::input::|use libxc_core::input::|
s|^use crate::output::|use libxc_core::output::|
s|^use crate::registry::|use libxc_core::registry::|
s|^use crate::layout::|use libxc_core::layout::|
```

For libxc-compat additionally needs:
```
s|^use crate::eval::|use libxc_eval::eval::|
s|^use crate::functional::|use libxc_eval::functional::|
s|^use crate::kernel::|use libxc_eval::kernel::|
s|^use crate::workspace::|use libxc_eval::workspace::|
```

**Notes for planner:**
- **Imports within the SAME crate stay as `crate::*`.** Inside libxc-eval, `use crate::functional::*` and `use crate::eval::*` are fine because they reference siblings inside the same crate after the move. Inside libxc-compat, `use crate::compat::errno::*` becomes `use crate::errno::*` (drop the `compat::` prefix since the compat tree IS the libxc-compat crate root) — this is a separate s-pattern (search `src/compat/` for `crate::compat::` references; only `macros.rs` had them per the macro analysis above).
- **`#[cfg(test)] mod tests`** blocks at the bottom of moved files have their own `use crate::*` lines — same rules apply (verify with `cargo check --workspace --tests`).
- **`use libxc_kernel_*` references inside the moved files** stay unchanged (e.g., `src/eval/dispatch.rs:556: use libxc_kernel_lda::lda_x::*;`) — these are external-crate paths that resolve identically before and after the move.
- **Trap: `pub use crate::functional::params_lda::LdaXParams as LdaFunctionalParams;`** at `src/eval/mod.rs:19` — after move into libxc-eval, this becomes `pub use crate::functional::params_lda::LdaXParams as LdaFunctionalParams;` (UNCHANGED — it's intra-libxc-eval). The root facade re-exports it as `pub use libxc_eval::eval::LdaFunctionalParams;` per RESEARCH §7 line 191.
- **Bisectability invariant** (CONTEXT specifics line 226): every commit MUST leave `cargo check --workspace` green. The planner should structure the rewrite as one commit per moved tree (e.g., `git mv src/error → crates/libxc-core/src/error` + immediate import rewrite + cargo check + commit), not in a single big-bang commit.
- **Watch for `dead_code` warnings post-rewrite** (RESEARCH §6 line 143): an item used cross-module pre-split may become unused inside its new crate if the consumer crossed a boundary. Treat dead_code warnings as load-bearing — they signal a missing `pub use` re-export.

---

## Shared Patterns

### `#![deny(warnings)]` + clippy allowances

**Source:** `src/lib.rs:1-7` (today's root).

**Apply to:** All three new lib.rs files (libxc-core, libxc-eval, libxc-compat).

**Excerpt (verbatim):**
```rust
#![deny(warnings)]
// CubeCL #[cube] macro expansion generates code that triggers these lints.
// The excessive_precision lint is also inappropriate for scientific constants
// where trailing digits are intentional for documentation clarity.
#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]
```

### Edition 2024 + literal version pins

**Source:** All `crates/kernel-*/Cargo.toml` files.

**Apply to:** All three new Cargo.toml files.

**Excerpt:**
```toml
[package]
name = "<crate-name>"
version = "0.1.0"
edition = "2024"
```

Plus literal versions for shared deps (no `[workspace.dependencies]`):
```toml
bitflags = "2.10.0"
bytemuck = { version = "1.25.0", features = ["derive"] }
cubecl = { version = "0.9.0", default-features = false, features = ["cpu"] }
thiserror = "2.0.18"
```

### Cargo verification recipe per commit

**Source:** RESEARCH §Validation Architecture lines 234-261.

**Apply to:** Every commit in this phase.

```bash
# Per-commit (bisect invariant)
cargo check --workspace 2>&1 | tee log/10-NN-NN-task.log
# Per-plan (full suite)
cargo test --workspace 2>&1 | tee log/10-NN-merge-test.log
# Per-plan (no new warnings)
cargo check --workspace --message-format=short 2>&1 | tee log/10-NN-warnings.log
! grep -E "^warning:" log/10-NN-warnings.log
```

### `cargo tree` invariants (success criteria 2-4)

**Source:** RESEARCH §Validation Architecture lines 248-251.

**Apply to:** Phase gate.

```bash
# Criterion 2: libxc-core has no cubecl/kernel-* deps
cargo tree -p libxc-core --depth 1 2>&1 | tee log/10-final-cargo-tree-core.log
! grep -E "cubecl|libxc-kernel" log/10-final-cargo-tree-core.log

# Criterion 3: libxc-eval has libxc-core but NOT libxc-compat
cargo tree -p libxc-eval --depth 2 2>&1 | tee log/10-final-cargo-tree-eval.log
grep -q "libxc-core" log/10-final-cargo-tree-eval.log
! grep -q "libxc-compat" log/10-final-cargo-tree-eval.log

# Criterion 4: libxc-compat has both; nothing depends on libxc-compat
cargo tree -p libxc-compat --depth 2 2>&1 | tee log/10-final-cargo-tree-compat.log
cargo tree -i -p libxc-compat 2>&1 | tee log/10-final-cargo-tree-compat-inverse.log
# inverse tree should show only libxc-compat itself
```

### Symbol-export sanity check (cdylib)

**Source:** RESEARCH §Implementation Knowledge §5 line 136.

**Apply to:** Plan 10-03 verification.

```bash
nm -D --defined-only target/debug/libxc_rs.so | grep ' T ' | sort > log/10-final-cdylib-symbols.log
# Assert ≥85 xc_*/xc_rs_* exported symbols (count varies as Phase 6 lands more)
```

---

## No Analog Found

No file in this phase lacks a usable analog. All 11 distinct file roles map to either:
- An exact in-repo analog (8 files: kernel-math/Cargo.toml, kernel-mgga/Cargo.toml, kernel-math/src/lib.rs (×2 for libxc-core and libxc-eval), src/compat/mod.rs, src/eval/mod.rs, root src/lib.rs:23-38, root Cargo.toml).
- A pre-image outside the workspace (`libxc-master/src/xc.h`) for the C header.
- A self-pattern for the 7 xtask path-string edits.
- A mechanical transform pattern documented in §E for the 105 cross-module imports.

The single net-new shape (cdylib + staticlib + rlib in one crate) has no in-repo analog but is fully specified by CONTEXT D-07 lines 209-223, with the **one outstanding planner reconciliation:** the "default name `libxc_rs`" requirement (CONTEXT line 27, D-08) likely requires `[lib] name = "libxc_rs"` despite D-08's "no override" phrasing — flagged in §A above for plan-time reconciliation.

---

## Metadata

**Analog search scope:**
- `crates/kernel-math/Cargo.toml`, `crates/kernel-lda/Cargo.toml`, `crates/kernel-mgga/Cargo.toml`, `crates/kernel-gga/Cargo.toml`
- `crates/kernel-math/src/lib.rs`, `crates/kernel-lda/src/lib.rs`
- `src/lib.rs`, `Cargo.toml` (root)
- `src/compat/{mod,macros,raw_handle,errno,legacy_eval}.rs`
- `src/eval/{mod,gga_dispatch/batch14,mgga_dispatch/mod,gga_dispatch/batch1a}.rs`
- `src/functional/{mod,evaluate}.rs`
- `src/error/mod.rs`, `src/meta/mod.rs`, `src/math/mod.rs`, `src/main.rs`
- `src/api/mod.rs`, `src/kernel/mod.rs`, `src/workspace/mod.rs`
- `xtask/Cargo.toml`, `xtask/src/main.rs:280-388`, `xtask/src/generate_metadata.rs:435-655`
- `libxc-master/src/xc.h:1-80`

**Files scanned:** 25 (all Read calls were single-pass, no re-reads).

**Verification of "no analog .h":**
```
$ find /home/user/Documents/workspace/libxc_rs -name '*.h' \
    -not -path '*/target/*' -not -path '*/libxc-master/*' -not -path '*/.git/*'
(empty — confirms RESEARCH §Open Questions item 3)
```

**Pattern extraction date:** 2026-05-07
