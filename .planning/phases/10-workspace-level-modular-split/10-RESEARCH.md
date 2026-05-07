# Phase 10: Workspace-Level Modular Split — Research

**Researched:** 2026-05-07
**Domain:** Cargo workspace refactor; rlib/cdylib/staticlib boundary; cross-crate visibility
**Confidence:** HIGH (most findings verified against current source; cdylib re-export risk verified against rust-lang issues)

## Phase Goal Restated

Split the monolithic root `libxc_rs` crate into a layered Cargo workspace:
- `crates/libxc-core` — pure data layer (model, meta, registry, input, output, layout, dims, error). Zero CubeCL imports.
- `crates/libxc-eval` — orchestration (eval, functional, kernel glue, workspace). Depends one-way on libxc-core.
- `crates/libxc-compat` — extern "C" shim with `crate-type = ["rlib", "cdylib", "staticlib"]`. Depends on both. Output artifact name `libxc_rs.so` / `libxc_rs.a`.
- Root `libxc_rs` — thin facade (`api/`) + curated re-exports preserving today's surface.

Per CONTEXT.md, decisions D-01 through D-09 are LOCKED. This research surfaces *implementation knowledge* — pitfalls, ordering, validation mechanics — that the planner needs to write executable plans.

## Locked Decisions Recap

These come from CONTEXT.md. Quick tag list, not re-litigated:

- **D-01:** `src/error/` → `crates/libxc-core/src/error/`. Verified [VERIFIED via grep `crates/`]: zero kernel-* sources construct `LibxcRsError`. Stub files `ffi.rs`, `internal.rs`, `public.rs` (each 2 lines) may be deleted.
- **D-02:** Delete `src/math/mod.rs` (12-line dead re-export shim). Pre-delete grep confirmed [VERIFIED]: zero `use crate::math::*` callsites in `src/`. **However:** `tests/math_integration.rs` has 5 `use libxc_rs::math::*` paths — root facade MUST re-export `libxc_kernel_math` to preserve them. See §Implementation Knowledge §7 for exact paths.
- **D-03:** xtask writes generated outputs into `crates/libxc-core/src/...`. 7 path-string updates: `xtask/src/main.rs:291,329,355,387` + `xtask/src/generate_metadata.rs:445,595,643`.
- **D-04, D-05, D-06:** No generated file crosses crate boundaries; no second xtask target needed; xtask stays a string emitter without typed deps.
- **D-07:** `crates/libxc-compat/Cargo.toml` declares `crate-type = ["rlib", "cdylib", "staticlib"]`.
- **D-08:** cdylib name = `libxc_rs` (Rust default). Output: `libxc_rs.so` / `libxc_rs.a`.
- **D-09:** Hand-written C header at `crates/libxc-compat/include/xc_rs.h`, committed.

## Phase Requirements

(No REQ-IDs assigned — coverage is via the 8 success criteria in ROADMAP.)

| Success Criterion | Research Support |
|----|------------------|
| 1. Four target crates exist | Mechanical from CONTEXT — see §Implementation Knowledge §2 |
| 2. `cargo tree -p libxc-core` shows zero CubeCL/kernel-* | Validated via `cargo tree`; see §Validation Architecture |
| 3. `cargo tree -p libxc-eval` shows libxc-core but NOT libxc-compat | Same |
| 4. `cargo tree -p libxc-compat` shows both; nothing depends on libxc-compat | Same |
| 5. Root `libxc_rs::...` paths still resolve | §7 surveys exact paths used by verify/tests/benches |
| 6. `cargo test --workspace` matches pre-refactor pass/fail set | §Validation Architecture: pre-capture & diff |
| 7. Oracle parity at 1e-12 strict | §Validation Architecture: representative sweep |
| 8. `cargo build --workspace` zero new warnings | `#![deny(warnings)]` per crate; §6 |

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Domain types (Family/Spin/Kind/...) | libxc-core | — | Pure data; no compute |
| Static registry tables | libxc-core | — | Generated tables, no compute |
| Error enum (`LibxcRsError`) | libxc-core | — | Per D-01; consumed by both libxc-eval and libxc-compat |
| I/O bundles (`LdaInput`/`LdaOutput`/...) | libxc-core | — | Buffer descriptors with validation; no compute |
| Dimensions / layout / OutputMask | libxc-core | — | Pure data |
| Dispatch routing | libxc-eval | — | Compute orchestration |
| `Functional` lifecycle | libxc-eval | — | Stores typed runtime state |
| `EvaluationWorkspace` | libxc-eval | — | Scratch buffer ownership |
| Kernel-launch glue | libxc-eval | — | CubeCL-touching code |
| `extern "C"` C-ABI shim | libxc-compat | — | All `unsafe` confined per BUILD-04 |
| Thread-local errno | libxc-compat | — | Phase 6 D-A4-1 location |
| `BatchEvaluator` / `FunctionalBuilder` / `EvaluateInput` | root `libxc_rs` (api/) | — | Layer-3 ergonomic; thin facade target |
| Curated re-exports | root `libxc_rs` (lib.rs) | — | Preserves downstream surface (D-A4-1 unchanged) |

## Implementation Knowledge

### 1. `git mv` mechanics for large module moves

**Failure modes the planner should anticipate:**

- **`git mv src/eval crates/libxc-eval/src/eval` succeeds idempotently** [CITED: git docs]; submodule paths inside the moved tree (`mod.rs`'s `pub mod gga_dispatch;` etc.) stay correct because they're crate-local relative paths. No `#[path = "..."]` attributes anywhere in `src/` [VERIFIED: grep returned empty].
- **The seam fails predictably:** every `use crate::error::LibxcRsError` (or `crate::model::*`, `crate::dims::*`, `crate::input::*`, `crate::output::*`, `crate::meta::*`) inside the moved files is wrong after the move. There are **133 such cross-module imports** in eval/functional/kernel/workspace/api/compat [VERIFIED: grep count]. Each must become `use libxc_core::error::LibxcRsError` (or the curated alias if libxc-eval's lib.rs re-exports them).
- **`include_str!` / `include_bytes!` / `include!` macros do not exist in `src/`** [VERIFIED: grep returned empty]. No relative-file-path machinery breaks.
- **No `build.rs` files in root or `verify/`** [VERIFIED: find returned empty matching the relevant top dirs]. xtask is the only codegen.
- **Tests modules survive in-place** because they're `#[cfg(test)] mod tests { use crate::... }` at the bottom of each file. Same fix-up rule: `crate::error::*` → `libxc_core::error::*` (or via the local crate's re-export).

**Recommended tactic per CONTEXT discretion:** `git mv` per directory at a time, rather than copy+delete, to preserve `git blame`. Bisectability is the constraint — see §10.

### 2. Cargo workspace member additions

**Convention used by this codebase** [VERIFIED: Cargo.toml + every `crates/kernel-*/Cargo.toml`]:
- **No `[workspace.dependencies]` table is in use.** Each leaf crate declares its dep with a literal version string (`cubecl = { version = "0.9.0", default-features = false, features = ["cpu"] }`). Don't introduce `workspace = true` patterns in this phase — that's scope creep and inconsistent with the kernel-* convention.
- Root `Cargo.toml` `[workspace] members` currently lists `xtask`, `verify`, `libxc-sys`. The 170 kernel-* crates are in `default-members`. **Add `crates/libxc-core`, `crates/libxc-eval`, `crates/libxc-compat` to BOTH lists** so they participate in `cargo build` (no `-p`) and tooling.
- Root `Cargo.toml` `[dependencies]` shrinks: keep only what `api/` needs at root level; everything else moves to leaf crate Cargo.toml files. Recommended root deps post-split: `libxc-core = { path = "crates/libxc-core" }`, `libxc-eval = { path = "crates/libxc-eval" }`, `libxc-compat = { path = "crates/libxc-compat" }`. The `cubecl`, `bytemuck`, `bitflags`, `thiserror` deps move to whichever leaf crate uses them (libxc-core takes thiserror+bitflags+bytemuck; libxc-eval takes cubecl+bytemuck via re-import; libxc-compat takes thiserror).
- **Dependency-block partition:**

| Dep | libxc-core | libxc-eval | libxc-compat | root |
|-----|------------|-----------|--------------|------|
| `bitflags 2.10` | YES (FunctionalFlags, OutputMask) | — | — | — |
| `bytemuck 1.25 (derive)` | YES (Pod/Zeroable on input/output structs) | YES (cast_slice in kernel/launch.rs) | — | — |
| `cubecl 0.9 cpu` | NO (D-criterion 2) | YES (kernel/launch.rs uses ComputeClient) | — | — |
| `thiserror 2.0` | YES (LibxcRsError enum) | — | YES (errno conversion in compat) | — |
| `libxc-kernel-{math,lda,gga,mgga}` | NO | YES (dispatch table) | — | — |
| `libxc-core` | — | YES (path-dep) | YES (path-dep) | YES (path-dep) |
| `libxc-eval` | — | — | YES (path-dep) | YES (path-dep) |
| `libxc-compat` | — | — | — | NO (success criterion 4) |

  Note "root depends on libxc-compat: NO" — because the cdylib comes from libxc-compat directly. Anything root needs that lives in libxc-compat is a sign the layering is wrong. (The ergonomic `api/` does not need any compat layer types.)

### 3. Cross-crate visibility (`pub(crate)` and `pub(super)`)

**Verified via grep across `src/error src/eval src/compat src/functional src/kernel src/workspace`:**

| Item | File | Current Vis | After Split | Action |
|------|------|-------------|-------------|--------|
| `Functional` fields (`meta`, `spin`, `dims`, `thresholds`, `ext_params`, `params`, `auxiliaries`, `mix_coefficients`) | `src/functional/mod.rs:31-48` | `pub(crate)` | Stays libxc-eval-internal (libxc-eval owns Functional) | **No change** — siblings inside libxc-eval still resolve `pub(crate)` |
| `dispatch_*` per-functional fns in batch files (`gga_dispatch/batch*.rs`, `mgga_dispatch/batch*.rs`) | many | `pub(crate)` | Stays libxc-eval-internal | **No change** |
| `xc_func_type::as_initialized_const/_mut` in raw_handle | `src/compat/raw_handle.rs:32,49` | `pub(crate)` | Stays libxc-compat-internal | **No change** |
| `PROPAGATION_RULES` const | `src/meta/generated_propagation.rs:8` | `pub(crate)` | Crosses libxc-core → libxc-eval boundary (consumed by `src/functional/lifecycle.rs:13`) | **WIDEN to `pub`** + widen the module declaration (see below) |
| `mod generated_propagation` in `src/meta/mod.rs:3` | source | `pub(crate)` mod | Same boundary cross | **WIDEN to `pub` mod** (or add a stable re-export `pub use generated_propagation::PROPAGATION_RULES;` in libxc-core's `meta/mod.rs`) |
| `HYBRID_TYPES`, registry tables (`REGISTRY_BY_ID`, `REGISTRY_BY_NAME`, `REMOVED_IDS`, `NAME_ALIASES`), per-functional `XC_LDA_X` etc. | `src/meta/{generated*,registry/by_*,registry/removed}` | `pub(crate)` | Accessed only from registry/mod.rs (same crate post-split) [VERIFIED via grep] | **No change** |
| `FunctionalId(pub(crate) u16)` field | `src/model/mod.rs:81` | `pub(crate) u16` | Inner field never read across crate boundary [VERIFIED: only constructors used externally] | **No change** |

**The single critical visibility widening:**

```rust
// libxc-core/src/meta/mod.rs — was: pub(crate) mod generated_propagation;
pub mod generated_propagation;  // OR keep pub(crate) and add re-export below

// Optional cleaner alternative — keep module pub(crate), add re-export:
pub(crate) mod generated_propagation;
pub use generated_propagation::{PropagationRule, PROPAGATION_RULES};
```

The recommended fix is **the re-export** form, so the public API of libxc-core is `libxc_core::meta::PROPAGATION_RULES` and `libxc_core::meta::PropagationRule` (both already pub at the type level). libxc-eval's lifecycle.rs becomes `use libxc_core::meta::PROPAGATION_RULES;`. This is the only cross-crate-boundary `pub(crate)` widening required. [VERIFIED: full grep across all modules].

### 4. `#[derive(...)]` re-exports and trait coherence

- `LibxcRsError` derives `Debug + thiserror::Error`. Both come with the type when moved. No coherence issue: `thiserror::Error` is implemented on the type defined in the same crate (libxc-core post-move) [CITED: thiserror docs — `#[derive(Error)]` expands inline].
- Bytemuck `Pod`/`Zeroable` derives don't appear on any production type in `src/` [VERIFIED: grep returned only `bytemuck::cast_slice` invocations in `kernel/launch.rs`, not derives]. No orphan-rule concern for input/output structs.
- **`impl ... for crate::model::*` outside `src/model/`:** zero such impls [VERIFIED: grep `impl.*for crate::model::|impl.*for crate::error::|impl.*for crate::meta::` in `src/` returned empty]. All inherent and trait impls live alongside their type definitions. No orphan-rule trap.

### 5. `extern "C"` symbol visibility from the cdylib

**The mechanism that works** [CITED: rust-lang/rust issue #98449, RFC 1510]:
- `#[no_mangle] pub extern "C" fn xc_func_alloc() -> *mut xc_func_type { ... }` defined **directly inside the cdylib's source tree** is exported in the cdylib's symbol table on Linux/macOS/Windows. **This is the exact pattern the codebase uses today** [VERIFIED: every extern fn in `src/compat/{raw_handle.rs,errno.rs}` has `#[unsafe(no_mangle)]` from edition-2024 syntax].
- The current count is 7 extern "C" functions [VERIFIED]; Phase 6 in-flight will add ~80 more across `legacy_eval.rs`, `info.rs`, `hybrid.rs`, `library.rs`, `ids.rs`. All are in `src/compat/` directly.
- **The known issue does NOT apply here:** rust-lang/rust issue #63125 / #96192 / #128949 describe failures when re-exporting `#[no_mangle]` symbols from a **dependency rlib** through a `cdylib` (i.e., `pub use other_crate::foo;` where `foo` is `#[no_mangle]` in `other_crate`'s rlib). On Linux/GCC this fails to export. **Phase 10's design avoids this entirely:** `crates/libxc-compat` IS the cdylib AND defines all extern "C" functions in its own source. There is no re-export hop.
- **Sanity check after the move:** run `nm -D --defined-only target/debug/libxc_rs.so | grep ' T '` and assert all 85+ extern symbols are present. Add this to plan 10-03's verification step. Symbol-presence regression is the only credible failure mode for this concern.
- **`#[unsafe(no_mangle)]` (edition 2024 syntax) is preserved literally on `git mv`** — no rewrite needed.

### 6. `#![deny(warnings)]` propagation

- Root `src/lib.rs:1` sets `#![deny(warnings)]` [VERIFIED]. Each new leaf crate's `lib.rs` should mirror this — **same line at top, plus the same three `#![allow(clippy::...)]` exemptions** copied from current root (excessive_precision, needless_late_init, too_many_arguments). Rationale: kernels and metadata generation need those allowances; they propagate when those modules move.
- **Workspace-only warnings to watch for:**
  - `dead_code`: an item that was used cross-module pre-split may become unused inside its new crate if the consumer moved to a different crate. After each `git mv` step, `cargo check --workspace 2>&1 | tee log/10-NN-step.log` will emit `dead_code` warnings before any `LibxcRsError`-style errors. Treat them as load-bearing: they signal a `pub use` re-export or visibility widening was missed.
  - `unused_imports`: similar dynamic — a `use crate::eval::workspace::EvaluationWorkspace` that ends up inside the libxc-eval crate referencing its own internal item via the wrong path won't error, just warn. The deny gate catches this.
  - `unused_extern_crates`: rare but possible if a leaf crate's Cargo.toml lists a dep that nothing in its src/ actually imports (e.g., libxc-core listing `cubecl` by accident).
- **Verification recipe:** `cargo check --workspace --message-format=short 2>&1 | tee log/10-NN-warning-audit.log; ! grep -E "^warning:" log/10-NN-warning-audit.log` (assert empty grep). Add as the last step of every plan in this phase.

### 7. Curated re-export shape (root facade) — exact paths used

**External consumer survey** — `grep -rhE "use libxc_rs::[a-zA-Z_:{}, ]+;" verify/ tests/ examples/ benches/ | sort -u` returned [VERIFIED]:

| Path used | Site |
|-----------|------|
| `libxc_rs::LibxcRsError` | verify/tests |
| `libxc_rs::eval::EvaluationWorkspace` | verify/tests |
| `libxc_rs::eval::dispatch_gga` | verify/tests |
| `libxc_rs::eval::dispatch_mgga` | verify/tests |
| `libxc_rs::eval::{dispatch_lda, LdaFunctionalParams}` | verify/tests |
| `libxc_rs::functional::Functional` | verify/tests |
| `libxc_rs::functional::classify_hybrid` | verify/tests |
| `libxc_rs::input::{LdaInput, GgaInput, MggaInput}` | verify/tests |
| `libxc_rs::math::constants::{KF_CONST, RS_CONST}` | tests/math_integration.rs |
| `libxc_rs::math::dft_quantities::{reduced_gradient_s, tf_kinetic, wigner_seitz_rs}` | tests/math_integration.rs |
| `libxc_rs::math::erf::erf_approx` | tests/math_integration.rs |
| `libxc_rs::math::powers::pow_1_3` | tests/math_integration.rs |
| `libxc_rs::math::spin::spin_scaling` | tests/math_integration.rs |
| `libxc_rs::meta::{ExtParamSpec, FunctionalMeta, HybridTerm, Reference}` | verify/tests |
| `libxc_rs::model::{HybridType, FunctionalId, GgaFunctional, LdaFunctional, MggaFunctional, Spin, Thresholds, DerivativeOrder, FunctionalFlags, HybridTermKind, Family, Kind, Dimensionality}` | verify/tests |
| `libxc_rs::output::{LdaOutput, GgaOutput, MggaOutput, OutputMask}` | verify/tests |
| `libxc_rs::registry::{lookup_by_id, lookup_by_name, all_functional_ids}` | verify/tests |

**Implication for the root facade (recommended re-export shape — option (c) "split-by-module"):**

```rust
// Root src/lib.rs post-split
#![deny(warnings)]
#![allow(clippy::excessive_precision, clippy::needless_late_init, clippy::too_many_arguments)]

pub mod api;

// Preserve namespace-shaped paths today's consumers use
pub mod model { pub use libxc_core::model::*; }
pub mod meta  { pub use libxc_core::meta::*; }
pub mod error { pub use libxc_core::error::*; }
pub mod dims  { pub use libxc_core::dims::*; }
pub mod registry { pub use libxc_core::registry::*; }
pub mod input { pub use libxc_core::input::*; }
pub mod output { pub use libxc_core::output::*; }

pub mod eval {
    pub use libxc_eval::eval::{dispatch_lda, dispatch_gga, dispatch_mgga, EvaluationWorkspace, LdaFunctionalParams};
}
pub mod functional { pub use libxc_eval::functional::{Functional, classify_hybrid, CamCoefficients, FunctionalParams, NlcCoefficients, NoParams}; }

// Math re-export — D-02 deletes src/math/, but tests/math_integration.rs uses libxc_rs::math::*
pub mod math { pub use libxc_kernel_math::*; }

// Top-level re-exports kept verbatim (today's lines 23-39)
pub use libxc_core::error::LibxcRsError;
pub use libxc_core::model::{Family, Kind, Spin, DerivativeOrder, FunctionalId, FunctionalFlags,
                             HybridType, HybridTermKind, Dimensionality, Thresholds,
                             LdaFunctional, GgaFunctional, MggaFunctional};
pub use libxc_core::meta::{FunctionalMeta, Reference, ExtParamSpec, HybridTerm};
pub use libxc_core::dims::Dimensions;
pub use libxc_core::registry::{lookup_by_id, lookup_by_name, functional_count, version, version_string};
pub use libxc_core::input::{LdaInput, GgaInput, MggaInput};
pub use libxc_core::output::{LdaOutput, GgaOutput, MggaOutput, OutputMask};
pub use libxc_eval::eval::{dispatch_lda, dispatch_gga, dispatch_mgga};
pub use libxc_eval::functional::{classify_hybrid, CamCoefficients, Functional, FunctionalParams, NlcCoefficients, NoParams};
pub use api::{BatchEvaluator, EvaluateInput, FunctionalBuilder};
```

**Note CONTEXT.md recommends shape (a) "explicit only" but the survey shows tests use submodule paths (`libxc_rs::math::constants::*`, `libxc_rs::eval::dispatch_lda`, etc.). Shape (a) — flat top-level re-exports only — would break `tests/math_integration.rs:19-23`. The plan must add `pub mod math` re-exporting `libxc_kernel_math::*` to preserve those paths even after `src/math/` is deleted.** Shape (c) is therefore the lowest-churn choice for downstream consumers.

The `pub mod math { pub use libxc_kernel_math::*; }` re-export is a **D-02a follow-on** that CONTEXT.md mentions ("If found, replace with `pub use libxc_kernel_math::...` in the root facade"). Confirmed needed by the test file survey.

### 8. Compile-time impact estimation

**Hypothesis:** moderate net win on incremental rebuilds, ~zero impact on clean builds.

**Reasoning** (LOW confidence on absolute numbers, MEDIUM on direction):
- Today's root `libxc_rs` is one large compile unit. Any change to anything in `src/` rebuilds the whole crate. Even with `codegen-units=16` it's a single crate-graph node for incremental purposes.
- Post-split: 3 leaf crates + root facade. A change to `src/error/mod.rs` (now `crates/libxc-core/src/error/mod.rs`) requires libxc-core + libxc-eval + libxc-compat + root rebuild, but each is independently smaller (libxc-core has zero CubeCL — its rebuild is ~seconds; libxc-eval is the big one with cubecl proc-macro expansion).
- A change to `src/eval/dispatch.rs` rebuilds libxc-eval + libxc-compat + root — but **NOT libxc-core**. That's the win: editing dispatch logic stops triggering metadata-table recompilation downstream of libxc-core. Today both are in the same crate so any edit invalidates the whole graph.
- Clean-build wall-time should be neutral-to-slightly-positive. CubeCL's proc-macro expansion (the dominant cost) is unchanged — those still happen in libxc-eval and the kernel-* crates. Three additional small crate boundaries add minor cargo bookkeeping (~seconds, not minutes).
- **No reason this phase HURTS build time.** The Phase 8/9 sub-crate explosion already segmented kernels.

**Recommendation for must_haves:** include "no measurable workspace clean-build wall-time regression vs. pre-Phase-10 baseline (within ±10%)" as a soft target. Capture pre-refactor `time cargo build --workspace` once and compare post-merge.

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | rustc built-in test harness + criterion (benches only); no separate framework |
| Config file | none — Cargo's `[lib]/[[test]]/[[bench]]` blocks |
| Quick run command | `cargo check --workspace 2>&1 \| tee log/10-NN-quick.log` |
| Full suite command | `cargo test --workspace --release 2>&1 \| tee log/10-NN-full.log` |
| Phase gate | All success criteria 1-8 satisfied; logs reviewed before `/gsd-verify-work` |

### Phase Requirements → Test Map

(Phase 10 has no REQ-IDs; mapping is to ROADMAP success criteria.)

| Criterion | Behavior | Type | Automated Command | File Exists? |
|-----------|----------|------|-------------------|--------------|
| 1 | Four crates exist | smoke | `test -d crates/libxc-core/src && test -d crates/libxc-eval/src && test -d crates/libxc-compat/src` | ❌ Wave 0 (creates them) |
| 2 | core has no cubecl/kernel-* deps | tree-grep | `cargo tree -p libxc-core --depth 1 2>&1 \| tee log/10-final-cargo-tree-core.log; ! grep -E "cubecl\|libxc-kernel" log/10-final-cargo-tree-core.log` | ✅ post-Wave 3 |
| 3 | eval has core but no compat | tree-grep | `cargo tree -p libxc-eval --depth 2 2>&1 \| tee log/10-final-cargo-tree-eval.log; grep -q "libxc-core" log/10-final-cargo-tree-eval.log && ! grep -q "libxc-compat" log/10-final-cargo-tree-eval.log` | ✅ post-Wave 3 |
| 4 | compat has both; nothing depends on compat | tree-grep | `cargo tree -p libxc-compat --depth 2 2>&1 \| tee log/10-final-cargo-tree-compat.log; grep -q "libxc-core" log/10-final-cargo-tree-compat.log && grep -q "libxc-eval" log/10-final-cargo-tree-compat.log; cargo tree -i -p libxc-compat 2>&1 \| tee log/10-final-cargo-tree-compat-inverse.log` (inverse tree should show only libxc-compat itself) | ✅ post-Wave 3 |
| 5 | Public surface unchanged | unit | `cargo check -p libxc_rs-verify 2>&1 \| tee log/10-final-verify-check.log` (zero source changes in verify/tests/) | ✅ existing verify/ tests |
| 6 | All tests pass-set matches | regression | Pre-capture: `cargo test --workspace 2>&1 \| tee log/10-pre-test-snapshot.log` (stash before Phase 10 begins). Post: `cargo test --workspace 2>&1 \| tee log/10-final-test.log; diff <(grep "test result:" log/10-pre-test-snapshot.log \| sort) <(grep "test result:" log/10-final-test.log \| sort)` | ✅ existing tests |
| 7 | Oracle parity at 1e-12 | integration | `cargo test -p libxc_rs-verify --release --test lda_x_oracle --test gga_oracle --test mgga_oracle 2>&1 \| tee log/10-final-oracle-parity.log` | ✅ verify/tests/{lda_x_oracle,gga_oracle,mgga_oracle}.rs |
| 8 | Zero new warnings | check | `cargo check --workspace --message-format=short 2>&1 \| tee log/10-final-warnings.log; ! grep -E "^warning:" log/10-final-warnings.log` | ✅ deny(warnings) gates |

### Sampling Rate

- **Per task commit:** `cargo check --workspace 2>&1 | tee log/10-NN-NN-task.log` — every commit must leave this green (bisectability invariant from CONTEXT specifics §"Bisectability").
- **Per plan merge:** `cargo test --workspace 2>&1 | tee log/10-NN-merge-test.log` (full suite) + the cargo-tree assertions for the crates created/modified in that plan.
- **Phase gate:** Full suite + all 8 success-criterion commands above green before `/gsd-verify-work 10`.

### Oracle Parity Sample (criterion 7)

Per CONTEXT specifics §"Plan sequencing rationale", run a representative micro-sweep at the end of each plan to catch type-routing regressions early:

| Sample | Target | Order | Spin | Test invocation |
|--------|--------|-------|------|-----------------|
| LDA_X (id 1) | `verify/tests/lda_x_oracle.rs` | 0 + 2 | both | `cargo test -p libxc_rs-verify --release --test lda_x_oracle 2>&1 \| tee log/10-NN-oracle-lda.log` |
| GGA_X_PBE (id 101) | `verify/tests/gga_oracle.rs` | 0 + 2 | both | `cargo test -p libxc_rs-verify --release --test gga_oracle -- gga_x_pbe 2>&1 \| tee log/10-NN-oracle-gga.log` |
| MGGA_X_TPSS (id 202) | `verify/tests/mgga_oracle.rs` | 0 + 2 | both | `cargo test -p libxc_rs-verify --release --test mgga_oracle -- mgga_x_tpss 2>&1 \| tee log/10-NN-oracle-mgga.log` |

**Threshold:** relative error <= 1e-12 on energy (exc), <= 1e-10 on vrho. Matches Phase 4 oracle invariant. Pure refactor — values must not move at all.

### Wave 0 Gaps

- [ ] `crates/libxc-core/Cargo.toml` + `crates/libxc-core/src/lib.rs` — created in plan 10-01 task 1
- [ ] `crates/libxc-eval/Cargo.toml` + `crates/libxc-eval/src/lib.rs` — created in plan 10-02 task 1
- [ ] `crates/libxc-compat/Cargo.toml` + `crates/libxc-compat/src/lib.rs` — created in plan 10-03 task 1
- [ ] `crates/libxc-compat/include/xc_rs.h` — D-09; created in plan 10-03 (only if Phase 6 hasn't shipped one already; verified at plan time per D-09a)
- [ ] **Pre-capture baseline:** `cargo test --workspace 2>&1 | tee log/10-pre-test-snapshot.log` AND `time cargo build --workspace 2>&1 | tee log/10-pre-build-time.log` BEFORE the first `git mv`. Required for criteria 6 (test-set diff) and the §8 build-time soft target. **This must be the very first wave-0 task.**

## Plan Sequencing & Risk

CONTEXT.md sets sequence: 10-01 core → 10-02 eval → 10-03 compat+root. **Confirmed correct.** Reasoning:

1. **libxc-core has zero outbound deps on the moving code.** Verified: `crates/kernel-*/Cargo.toml` contain no `libxc_rs` references in production code (only `kernel-lda` has a dev-dep [VERIFIED]); kernel sources do not `use libxc_rs::*` for production.
2. **Root crate stays compilable between 10-01 and 10-02.** Mechanism: when libxc-core is extracted, root's `src/lib.rs` switches `pub mod model;` → `pub use libxc_core::model;` (and similarly for meta/registry/input/output/layout/dims/error). Files inside `src/eval/`, `src/functional/`, `src/api/`, `src/compat/`, `src/kernel/` continue to write `crate::error::LibxcRsError` because the root crate now re-exports `error` as a submodule. **No path-resolution surprise.** Tested mentally against Rust 2024 module rules.
3. **Same applies between 10-02 and 10-03.** When libxc-eval extracts, root re-exports `eval`, `functional`, `kernel`, `workspace`. The remaining `src/api/` and `src/compat/` files referring to `crate::eval::*` resolve via the re-export.
4. **No alternative ordering is safer.** Inverse-leaf order (compat first, then eval, then core) breaks bisectability — pulling compat out first leaves it referencing types still in root, requiring two-step path rewrites. Bottom-up (core first) lets each leaf finalize its `use libxc_core::...` paths in one pass.

**Risks the planner should call out as must_haves:**
- **R1 — `PROPAGATION_RULES` visibility regression:** §3 above. After plan 10-01, lifecycle.rs cannot compile until either the module is widened to `pub` OR a `pub use` re-export is added in libxc-core's meta/mod.rs. Plan 10-01 task that creates libxc-core MUST include this re-export. Otherwise plan 10-02 won't compile from its first commit.
- **R2 — Curated re-export drift:** §7's path table. Plan 10-03 task that reduces root must add `pub mod math { pub use libxc_kernel_math::*; }` and the namespace-preserving re-exports (`pub mod model { pub use libxc_core::model::*; }` etc.) — easy to forget the math re-export specifically because src/math/ was deleted in plan 10-01 (D-02). Verification: `cargo check -p libxc_rs --tests` (which builds tests/math_integration.rs) must pass.
- **R3 — `extern_c_wrapper!` macro export:** the macro is `#[macro_export]` [VERIFIED in `src/compat/macros.rs`]. After move into libxc-compat, it's exported at `libxc_compat::extern_c_wrapper!`. Internal callers in `src/compat/raw_handle.rs` use `crate::extern_c_wrapper;` [VERIFIED]. After move that resolves to `libxc_compat::extern_c_wrapper` — works because `#[macro_export]` exports at the crate root. **No fix needed,** but verify in plan 10-03 cargo check.
- **R4 — `cargo test --workspace` matching pass-set:** must_have is **STRICT** — no test added/removed/skipped. The pre-capture log/10-pre-test-snapshot.log is the source of truth.
- **R5 — kernel-lda dev-dep on libxc_rs:** `crates/kernel-lda/Cargo.toml:12` has `libxc_rs = { path = "../.." }` as a dev-dep [VERIFIED]. Post-split, `path = "../.."` still points at root, which still exists as a thin facade. **No change needed.** But planner should grep for any other `libxc_rs` references in `crates/*/Cargo.toml` (only one was found) and confirm at plan time.

## xtask Verification Recipe

After D-03's 7 path-string updates, the planner needs evidence that xtask still produces output equivalent to today's. Recommended recipe per plan 10-01:

```bash
# 1. Snapshot current generated files BEFORE the path edit
mkdir -p log/10-xtask-baseline
cp src/meta/generated.rs           log/10-xtask-baseline/generated.rs
cp src/meta/generated_hybrid.rs    log/10-xtask-baseline/generated_hybrid.rs
cp src/meta/generated_propagation.rs log/10-xtask-baseline/generated_propagation.rs
cp src/registry/by_id.rs           log/10-xtask-baseline/by_id.rs
cp src/registry/by_name.rs         log/10-xtask-baseline/by_name.rs
cp src/registry/removed.rs         log/10-xtask-baseline/removed.rs

# 2. Apply D-03 path-string updates to xtask/src/main.rs and xtask/src/generate_metadata.rs
#    (also git mv the 6 files into crates/libxc-core/src/...)

# 3. Re-run xtask
cargo run -p xtask -- generate-registry  2>&1 | tee log/10-xtask-regenerate-registry.log
cargo run -p xtask -- generate-metadata  2>&1 | tee log/10-xtask-regenerate-metadata.log

# 4. Diff against baseline — should be byte-equivalent
diff log/10-xtask-baseline/generated.rs           crates/libxc-core/src/meta/generated.rs           > log/10-xtask-diff-generated.log
diff log/10-xtask-baseline/generated_hybrid.rs    crates/libxc-core/src/meta/generated_hybrid.rs    > log/10-xtask-diff-generated-hybrid.log
diff log/10-xtask-baseline/generated_propagation.rs crates/libxc-core/src/meta/generated_propagation.rs > log/10-xtask-diff-generated-propagation.log
diff log/10-xtask-baseline/by_id.rs               crates/libxc-core/src/registry/by_id.rs            > log/10-xtask-diff-by-id.log
diff log/10-xtask-baseline/by_name.rs             crates/libxc-core/src/registry/by_name.rs          > log/10-xtask-diff-by-name.log
diff log/10-xtask-baseline/removed.rs             crates/libxc-core/src/registry/removed.rs          > log/10-xtask-diff-removed.log

# 5. Assert all diffs empty
for f in log/10-xtask-diff-*.log; do
  if [ -s "$f" ]; then echo "REGRESSION: $f is non-empty"; cat "$f"; exit 1; fi
done
echo "✓ xtask outputs byte-equivalent across path migration"
```

**Sub-cases to anticipate:**
- xtask emits `use crate::model::...` and `use crate::meta::...` in headers (verified in main.rs:296-297, generate_metadata.rs:454-455). Those `crate::` references are correct relative to wherever the file lands — they will resolve inside libxc-core's source tree as `libxc_core::model::*`, `libxc_core::meta::*`. **No xtask logic change required.** [VERIFIED via reading main.rs:296 + generate_metadata.rs:454 emitted strings.]
- The `pub(crate)` visibility on emitted statics (`pub(crate) const XC_LDA_X: ...`) stays correct — `registry/mod.rs` (also in libxc-core post-split) is a sibling and resolves `pub(crate)` fine.

## Pre-planning Blocker Audit

**Blocker 1: `audit-error-math-placement` todo** [`.planning/todos/pending/audit-error-math-placement.md`]
- **Resolved by D-01** (error → libxc-core; counter-position rejected because zero kernel-* sources construct LibxcRsError, verified). The grep evidence is concrete and reproducible: `grep -rln "LibxcRsError" crates/` returns empty [VERIFIED again 2026-05-07].
- **Resolved by D-02** (delete `src/math/mod.rs`; counter-position rejected because grep shows zero `use crate::math::*` callsites in `src/`, verified).
- **Outstanding flag:** D-02a anticipated needing root re-exports for downstream `libxc_rs::math::*` consumers; the survey in §7 confirms this is REQUIRED — `tests/math_integration.rs` uses 5 paths under `libxc_rs::math::*`. The plan's must_have list MUST include the `pub mod math { pub use libxc_kernel_math::*; }` shim. **NOT blocked, but watch this.**
- **Action:** mark the todo resolved when CONTEXT.md is committed.

**Blocker 2: Generated-files research question** [`.planning/research/questions.md` §"How to handle generated.rs files across the modular workspace split"]
- Q1 (xtask entry points and inputs): RESOLVED. Two entry points — `generate-registry` (in xtask/src/main.rs) reads `libxc-master/src/xc_funcs.h` etc.; `generate-metadata` (in xtask/src/generate_metadata.rs) calls libxc via libxc-sys to extract per-functional metadata.
- Q2 (write-into-destination vs. shared+re-export): RESOLVED by D-03 — write directly into the destination crate's src/. Cleaner, accepts the small coupling cost.
- Q3 (does any generated file mix concerns spanning crate boundaries): RESOLVED by D-04. **All 7 files land in libxc-core's domain.** The only cross-crate consumer of generated content is `PROPAGATION_RULES` from libxc-eval's lifecycle.rs, which can be solved with a normal re-export (see §3). The generators do not need splitting.
- Q4 (rebuild behavior): RESOLVED by D-04. Acceptable — Cargo's crate-level granularity means a metadata change rebuilds libxc-core (small) + libxc-eval (large because of cubecl) + libxc-compat + root. Not on critical path.
- Q5 (libxc-codegen library crate): RESOLVED by D-06 — defer. xtask stays a string emitter; if codegen complexity grows, carve later.

Both blockers fully resolved. **No outstanding pre-planning items.** [VERIFIED by reading both source documents end-to-end.]

## Open Questions / Remaining Risks

1. **`src/main.rs` "Hello, world!" disposition** — CONTEXT discretion item; not researched here. Recommend deletion in plan 10-03 task 0 cleanup; no compile/test impact either way. (Background: it's a 3-line vestigial binary [VERIFIED].)

2. **Phase 6 in-flight collision** — STATE.md says Phase 6 is currently EXECUTING (`Phase 06 — public-api-and-c-compatibility`). Phase 10 must NOT start until Phase 6 settles. Currently 7 extern "C" functions are committed; Phase 6's 06-02b/06-03 plans add ~80 more, all of which must move atomically with the rest of `src/compat/` in plan 10-03. Planner should verify Phase 6 status at plan time and if 06-* is still in flight, gate Phase 10 start on its completion. ROADMAP §"Depends on" already says this.

3. **`crates/libxc-compat/include/xc_rs.h` ownership** — D-09a says Phase 10 ships the header **only if Phase 6 hasn't already**. As of the research date, Phase 6's compat layer is partial (only lifecycle + wrapper macro committed); no header file is present in the repo [VERIFIED via `find . -name "xc_rs.h" -o -name "xc.h"` returning only libxc-master sources]. Phase 6's plan 06-03 owns the header. **Recommend planner check at plan time whether the header is committed at any path under `target/include/`, `include/`, `src/compat/include/`, or similar before deciding plan 10-03's scope on it.**

4. **`cargo tree -i -p libxc-compat` reverse query for criterion 4** — the inverse cargo tree (`-i`) requires a reverse-dependency walk. If for any reason the root facade or another crate path-deps libxc-compat (it shouldn't, per CONTEXT), this command shows it. **Worth asserting** in the validation log.

5. **Edition 2024 compatibility of `#[unsafe(no_mangle)]`** — confirmed today's source uses this syntax [VERIFIED]. New crates inherit edition 2024 from CLAUDE.md/MSRV. No risk.

6. **Workspace `resolver = "3"`** — root Cargo.toml declares `resolver = "3"`. New leaf crates inherit; verify no Cargo.toml in the new crates accidentally sets `resolver = "2"`.

7. **`[profile.dev]` / `[profile.release]` overrides** — root Cargo.toml has nontrivial profile config (`debug=0`, `codegen-units=16/256`, `incremental=false`, `[profile.dev.build-override] opt-level=3`). Workspace inheritance: `[profile.*]` blocks at workspace root apply to ALL members. Confirmed by Cargo docs [CITED: doc.rust-lang.org/cargo/reference/profiles.html#profile-selection]. New leaf crates need NO profile blocks. Phase 10 doesn't touch profiles.

8. **`cargo test` parallelism with cubecl-cpu mutex** — STATE.md notes "CubeCL CPU runtime requires mutex serialization for concurrent kernel launches in tests" (Phase 8). Phase 10 doesn't add new tests; existing serialization carries through. No risk.

## Sources

### Primary (HIGH confidence)
- Codebase grep across `src/` (133 cross-module imports counted, exhaustive `pub(crate)` audit, extern fn count, etc.) — verified 2026-05-07.
- `Cargo.toml` (root, kernel-math, kernel-lda, kernel-gga, kernel-mgga, kernel-gga-1a) — convention validated by direct read.
- `.planning/phases/10-workspace-level-modular-split/10-CONTEXT.md` — locked decisions D-01 through D-09.
- `.planning/phases/06-public-api-and-c-compatibility/06-CONTEXT.md` — Phase 6 compat-layer architecture.
- `.planning/notes/workspace-modular-architecture.md` — architecture lock.
- `.planning/research/questions.md` + `.planning/todos/pending/audit-error-math-placement.md` — blocker source documents (read end-to-end, both fully resolved).

### Secondary (MEDIUM confidence)
- [Rust RFC 1510 — cdylib](https://rust-lang.github.io/rfcs/1510-cdylib.html) — semantics of cdylib symbol export rules.
- [rust-lang/rust issue #98449](https://github.com/rust-lang/rust/issues/98449) — confirms `#[no_mangle]` symbols defined in cdylib's own crate ARE exported.
- [rust-lang/rust issue #63125](https://github.com/rust-lang/rust/issues/63125) — re-exporting `#[no_mangle]` symbols from a dependency rlib through cdylib FAILS on Linux. Confirms this design pitfall, but Phase 10's design avoids it (compat IS the cdylib).
- [rust-lang/rust issue #128949](https://github.com/rust-lang/rust/issues/128949) — same issue, re-confirmed in 2024.
- [rust-lang/rust issue #96192](https://github.com/rust-lang/rust/issues/96192) — static-lib symbols through rlib chains.
- [Rust users forum: Including third party `#[no_mangle]` functions in a cdylib](https://users.rust-lang.org/t/including-third-party-no-mangle-functions-in-a-cdylib/15388) — confirms no clean built-in re-export workaround; suggests defining symbols directly in the cdylib's source.
- [doc.rust-lang.org/cargo/reference/profiles.html](https://doc.rust-lang.org/cargo/reference/profiles.html) — workspace profile inheritance.
- [doc.rust-lang.org/reference/linkage.html](https://doc.rust-lang.org/reference/linkage.html) — cdylib output target documentation.

### Tertiary (LOW confidence)
- Build-time impact estimation in §8 — directional reasoning, no measured baseline. Recommend pre-capture `time cargo build --workspace` as part of Wave 0 (already in §Validation Architecture Wave 0 Gaps).

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `cargo tree -i -p libxc-compat` returns only libxc-compat itself when nothing else depends on it (success criterion 4 verification) | §Validation Architecture | LOW — if surprises emerge (e.g., dev-dep cycle), planner adjusts |
| A2 | Workspace `[profile.*]` blocks apply unchanged to new leaf crates without per-crate `[profile.*]` overrides needed | §Open Questions §7 | LOW — convention-following |
| A3 | The `pub mod math { pub use libxc_kernel_math::*; }` re-export at root preserves the path `libxc_rs::math::constants::KF_CONST` because `libxc_kernel_math::constants::KF_CONST` exists today | §7, §Plan Sequencing R2 | MEDIUM — if `libxc_kernel_math` changed module shape post-Phase 8, the re-export must mirror that shape. Recommend planner verify with `grep -n "pub mod" crates/kernel-math/src/lib.rs` at plan time |
| A4 | Build-time impact will be neutral-to-slightly-positive | §8 | LOW — soft target only, not a gate |

## Metadata

**Confidence breakdown:**
- Standard stack (Cargo workspace mechanics, Rust 2024 visibility rules): HIGH — verified against source convention and language reference.
- Architecture (crate boundaries, dep direction): HIGH — re-derived from CONTEXT and verified against grep.
- Pitfalls (`PROPAGATION_RULES`, math re-export, cdylib symbol path): HIGH — concrete reproductions in source.
- Validation architecture: MEDIUM — commands written but not run; pre-capture baseline still needed at plan execution time.
- Build-time estimation: LOW — directional only, cited as soft.

**Research date:** 2026-05-07
**Valid until:** 2026-06-07 (30 days; cubecl/cargo are stable, but rerun the cdylib link issue check if rust-lang releases a stable fix to issue #128949 in that window)
