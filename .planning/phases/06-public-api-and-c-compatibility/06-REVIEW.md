---
phase: 06-public-api-and-c-compatibility
reviewed: 2026-05-25T00:00:00Z
depth: standard
files_reviewed: 11
files_reviewed_list:
  - src/compat/legacy_eval.rs
  - src/compat/ids.rs
  - src/compat/info.rs
  - src/compat/library.rs
  - src/compat/hybrid.rs
  - src/compat/removed.rs
  - src/compat/macros.rs
  - src/compat/mod.rs
  - src/lib.rs
  - include/xc.h
  - verify/tests/compat_smoke.rs
findings:
  critical: 0
  warning: 4
  info: 6
  total: 10
status: issues_found
---

# Phase 6: Code Review Report

**Reviewed:** 2026-05-25T00:00:00Z
**Depth:** standard
**Files Reviewed:** 11
**Status:** issues_found

## Summary

Reviewed the Phase-6 C-ABI compatibility layer: `extern "C"` shims over the typed
Rust API (`legacy_eval.rs`, `ids.rs`, `info.rs`, `library.rs`, `hybrid.rs`), the
opaque-handle lifecycle and `FunctionalSlot` state machine (`raw_handle.rs`, read
for context), the thread-local errno + `cache_cstring` machinery (`errno.rs`, read
for context), the `extern_c_wrapper!` panic-safety macro, the `removed` errno
helper, the hand-written `include/xc.h`, and the FFI smoke test.

Overall the FFI boundary is well-constructed. NULL handling is consistent
(NULL handle → errno + sentinel return; NULL output pointer → skip-derivative per
libxc Pitfall 8), `catch_unwind` wraps every entry point so panics — including the
`np * stride` overflow `expect()` panics — convert to errno rather than unwinding
across the FFI boundary (which would be UB). The opaque-pointer casts
(`Box<FunctionalSlot>` ↔ `*mut xc_func_type`, `&'static FunctionalMeta` ↔
`*const xc_func_info_type`) are internally consistent. `np.checked_mul(stride)`
guards the buffer-sizing multiply against overflow. `cargo check -p libxc_rs
--no-default-features --lib` passes clean.

The findings below are correctness/maintainability concerns and ABI-doc mismatches,
not memory-safety defects. The highest-value items are the unbounded lifetime
elision in the slice/handle helpers (WR-01), a small set of `*-cam-coef` /
`*-nlc-coef` / aux-getter functions that set errno but leave caller output buffers
untouched on the error path — diverging from libxc's always-write behavior (WR-02),
the reused `lda_order_from_int` reporting a misleading order in its error (WR-03),
and a header/impl naming/parameter-doc mismatch (WR-04).

## Warnings

### WR-01: Unbounded lifetime elision in FFI slice/handle helpers

**File:** `src/compat/legacy_eval.rs:321`, `src/compat/legacy_eval.rs:332`, `src/compat/info.rs:14`, `src/compat/info.rs:24`, `src/compat/raw_handle.rs:32`, `src/compat/raw_handle.rs:54`

**Issue:** The helper signatures `unsafe fn ptr_to_opt_slice<'a>(ptr, np, stride) -> Option<&'a mut [f64]>`, `input_slice<'a>(...) -> &'a [f64]`, `info_ref<'a>(...) -> Option<&'a FunctionalMeta>`, `ref_ref<'a>`, and `FunctionalSlot::as_initialized_const/_mut<'a>` all introduce a free lifetime `'a` that is unconstrained by any input — i.e. the caller chooses `'a` to be anything, including `'static`. This is the classic "unbounded lifetime" footgun: the borrow checker can no longer tie the returned reference's validity to the raw pointer's actual lifetime. It happens to be sound *as currently used* because every caller consumes the slice/reference within the same enclosing `extern_c_wrapper!` body before the underlying buffer can be invalidated, but the signature provides no compiler-enforced protection against a future refactor that stores or returns the reference past the pointer's validity. Each is `unsafe fn` so the contract is at least flagged, but the lifetime is doing no work.

**Fix:** Tie the output lifetime to the input pointer's borrow by routing through a reference, or use a raw-pointer-derived lifetime explicitly. The cheapest robust option is to keep the helpers `unsafe` but document the invariant as a `# Safety` clause on each (several already have inline `// SAFETY:` notes — promote them to doc comments and state "the returned borrow must not outlive the buffer `ptr` points into"). If a signature change is acceptable, prefer threading the lifetime through an explicit borrow:
```rust
// Caller already holds the pointers for the whole wrapper body; an explicit
// PhantomData/borrow source makes 'a meaningful instead of caller-chosen.
unsafe fn input_slice<'a>(ptr: *const f64, np: usize, stride: usize, _scope: &'a ()) -> &'a [f64] { ... }
```
At minimum, add a unit/`miri` note that these are sound only because callers never escape the borrow. (No code change strictly required for current correctness — this is a hardening recommendation.)

### WR-02: `void`-returning getters leave output buffers untouched on the error path (diverges from libxc)

**File:** `src/compat/hybrid.rs:67` (`xc_hyb_cam_coef`), `src/compat/hybrid.rs:107` (`xc_nlc_coef`), `src/compat/hybrid.rs:139` (`xc_aux_func_ids`), `src/compat/hybrid.rs:158` (`xc_aux_func_weights`)

**Issue:** These functions return `void`, so a C caller has no return code to inspect — its only signal is the written output buffer. When the functional is not the expected shape (e.g. `cam_coefficients()` returns `None`, `nlc_coefficients()` returns `None`, or the handle is uninitialized so `as_initialized_const` returns `Err` and the `if let Ok(...)` arm is skipped), the function sets a thread-local errno (or, for `xc_nlc_coef`/`xc_aux_func_ids`/`xc_aux_func_weights`, silently does nothing) and returns without writing `*omega`/`*alpha`/`*beta`/`*nlc_b`/`*nlc_c`/`ids`/`weights`. The C caller then reads whatever uninitialized/stale value was in its buffer, with no way to know the call failed unless it independently polls `xc_rs_last_error_code()` — which is not the documented contract for these `void` functions and is not how libxc behaves (libxc's `xc_hyb_cam_coef` always populates the three coefficients). This is a silent-stale-read hazard at the boundary.

**Fix:** On every early-return / skipped path, zero-fill the output pointers before returning, matching libxc's always-write semantics. For example in `xc_hyb_cam_coef`:
```rust
// On the `else` (non-CAM) branch and on the as_initialized_const Err path:
if !omega.is_null() { unsafe { *omega = 0.0; } }
if !alpha.is_null() { unsafe { *alpha = 0.0; } }
if !beta.is_null()  { unsafe { *beta  = 0.0; } }
```
Apply the analogous zero-fill to `xc_nlc_coef` (b, c), `xc_aux_func_ids` (fill the declared-length slice with a sentinel such as `-1` or `0`), and `xc_aux_func_weights`. Document the post-condition in `include/xc.h`.

### WR-03: `lda_order_from_int` reused by `xc_gga_new` reports a misleading offending order

**File:** `src/compat/legacy_eval.rs:519` (helper), `src/compat/legacy_eval.rs:862` (GGA reuse)

**Issue:** `lda_order_from_int` is called from both `xc_lda_new` and `xc_gga_new` to map the integer `order` (0..=4) to `DerivativeOrder`. On an out-of-range `order` (e.g. `order = 7`), it returns `UnsupportedDerivativeOrder { id, order: DerivativeOrder::Lxc, max: DerivativeOrder::Lxc }`. The `order` field is hard-coded to `Lxc` regardless of what the caller actually passed, so the error message will claim the functional "does not support derivative order Lxc" even when the caller passed `7`. The integer the caller actually supplied is lost, making the diagnostic misleading for the C user who passed a bad order.

**Fix:** Carry the raw integer into the error, or add a dedicated variant. Minimal change: include the bad value in the message via a more descriptive error, e.g.
```rust
_ => {
    let id = unsafe { FunctionalSlot::as_initialized_const(p)? }.meta().id;
    return Err(crate::LibxcRsError::UnsupportedDerivativeOrder {
        id,
        order: DerivativeOrder::Lxc, // <- best-effort clamp
        max: DerivativeOrder::Lxc,
    });
    // Better: introduce an `InvalidDerivativeOrder { id, requested: i32 }` variant
    // so the actual integer (e.g. 7) appears in xc_rs_last_error_message().
}
```
Also rename the helper (it is no longer LDA-specific) to e.g. `order_from_int` since `xc_gga_new` shares it.

### WR-04: `include/xc.h` declarations diverge from implementations in two places (constness + parameter name)

**File:** `include/xc.h:199`, `include/xc.h:211`–`include/xc.h:339`; cross-ref `src/compat/hybrid.rs:107`, `src/compat/legacy_eval.rs` (LDA/GGA/MGGA evaluators)

**Issue:** Two ABI-surface mismatches between the hand-written header and the Rust impls:
1. **Parameter-name casing inconsistency:** `xc.h:199` declares `void xc_nlc_coef(const xc_func_type *p, double *nlc_b, double *nlc_C);` — note `nlc_C` (capital C) for the last parameter, while every other declaration and the Rust impl uses lowercase `nlc_c`. C ignores parameter names in prototypes so this compiles, but it is an inconsistency that will confuse downstream binding generators (e.g. bindgen consumers) and documentation tooling.
2. **`const`-qualifier divergence on the handle:** The Rust evaluators take `p: *const xc_func_type` and the header matches (`const xc_func_type *p`). Good. However the Rust threshold/ext-param *setters* take `p: *mut xc_func_type` (`xc_func_set_dens_threshold`, `xc_func_set_ext_params`, `xc_func_set_ext_params_name`) and the header correctly declares them non-const — verify these stay in sync, since the setters internally call `as_initialized_mut`. (Current state is consistent; this is a "keep in sync" guard.) The actionable defect is item 1.

**Fix:** Change `include/xc.h:199` `double *nlc_C` → `double *nlc_c` to match the impl and the rest of the header. Consider adding a build-time check (or a doc test) that diffs the header signatures against the `#[no_mangle] extern "C"` Rust signatures so future drift is caught mechanically.

## Info

### IN-01: `compat::removed` module is dead code

**File:** `src/compat/removed.rs:16` (`replacement_for`), `src/compat/removed.rs:29` (`format_removed_message`)

**Issue:** Both public helpers are never called anywhere in `src/` outside this module's own `#[cfg(test)]` block (verified by grep). The errno path already surfaces the full replacement guidance: `LibxcRsError::RemovedFunctionalId`'s `#[error(...)]` Display (`src/error/mod.rs:8`) already emits "removed functional ID {removed_id}; use {replacement_id} ({replacement_name}) instead", which the `extern_c_wrapper!` macro propagates verbatim via `e.to_string()`. So `format_removed_message` duplicates existing behavior and `replacement_for` re-derives data already inside the error.

**Fix:** Either delete `src/compat/removed.rs` (and its `pub mod removed;` line in `src/compat/mod.rs:12`) since the errno path already covers the use case, or wire it into a public accessor if a structured (non-string) replacement-id API is intended for C callers. Leaving unused `pub` helpers invites confusion about which path is canonical.

### IN-02: `FunctionalSlot::as_initialized_mut` is dead-but-retained (already annotated)

**File:** `src/compat/raw_handle.rs:54`

**Issue:** `as_initialized_mut` is `#[allow(dead_code)]` with a comment that no mutable C op is wired yet. It is actually reachable via `legacy_eval.rs` setters (`xc_func_set_dens_threshold` etc. call `FunctionalSlot::as_initialized_mut`), so the "not yet wired to a C entry point" comment is stale — the mutable path IS exercised. Confirm whether the `#[allow(dead_code)]` is still needed (it may only be needed under `--no-default-features` where the setters' callers are present but the lint still fires for some cfg). Low impact; flagged for comment accuracy.

**Fix:** Update the comment to reflect that the threshold/ext-param setters use this method, and drop `#[allow(dead_code)]` if the lint no longer fires under the default build. (Verify before removing — crate is `#![deny(warnings)]`.)

### IN-03: `xc_func_init` accepts `functional == 0` and defers rejection to the registry

**File:** `src/compat/raw_handle.rs:90`

**Issue:** The guard `if functional < 0 || functional > u16::MAX as i32` lets `functional == 0` through to `FunctionalId::from_raw(0)`, which then fails in `registry::lookup_by_id(0)`. This is correct behavior (0 is not a valid libxc id and the lookup rejects it with `UnknownFunctionalId`), but the `UnknownFunctionalId(0)` error constructed in the `< 0 || > u16::MAX` branch hard-codes the id as `0` regardless of the actual out-of-range value passed (e.g. `functional = 70000` reports id `0`, losing the real value). Mirrors WR-03's "lost original value" pattern.

**Fix:** Preserve the offending value where it fits, or note in the message that the value was out of `u16` range:
```rust
if functional < 0 || functional > u16::MAX as i32 {
    return Err(LibxcRsError::UnknownFunctionalId(
        u16::try_from(functional).unwrap_or(u16::MAX) // or carry the i32 in a dedicated variant
    ));
}
```

### IN-04: Legacy evaluators read inputs with the wrong-family stride before the family check rejects

**File:** `src/compat/legacy_eval.rs:339` (`lda_evaluate`), `src/compat/legacy_eval.rs:610` (`gga_evaluate`), `src/compat/legacy_eval.rs:925` (`mgga_run`)

**Issue:** When a caller invokes a family-mismatched entry point (e.g. `xc_lda_exc` on a GGA functional handle), `lda_evaluate` builds `rho_slice` using LDA strides (`Dimensions::lda(spin)`) and constructs `LdaInput` *before* `evaluate_lda` → `LdaFunctional::from_id` rejects the GGA id with `UnsupportedFunctional`. The input slice is read with LDA strides regardless of the real family. For a polarized GGA called via `xc_lda_exc`, the LDA `rho` stride (2) is a subset of the GGA `rho` stride (2), so no over-read occurs for `rho`; but this is incidental. libxc's legacy entry points likewise assume the caller pairs the right `xc_FAMILY_*` function with the handle, so this matches libxc semantics and is not a memory-safety bug *given the caller contract*. Flagged because the family rejection happens after the slice is materialized, so the diagnostic is "unsupported functional" rather than "wrong family for this entry point", which is slightly less actionable.

**Fix:** Optionally add an early `f.meta().family` check at the top of each `*_evaluate` helper that returns `FamilyMismatch { expected, actual }` before any input slicing, giving a clearer error. No correctness change required.

### IN-05: `xc_func_set_ext_params` allocates a `Vec` per call for the default-marker substitution

**File:** `src/compat/legacy_eval.rs:87`

**Issue:** Each `xc_func_set_ext_params` call heap-allocates `substituted: Vec<f64>` even when no value equals `LIBXC_EXT_PARAMS_DEFAULT` (the common case). Minor; ext-param setting is not on the hot evaluation path. (Performance is explicitly out of v1 review scope; noted only as a maintainability/style observation since the allocation is unconditional.)

**Fix:** No action needed for correctness. If desired, short-circuit when `raw_slice.iter().all(|&v| v != LIBXC_EXT_PARAMS_DEFAULT)` and forward `raw_slice` directly.

### IN-06: Smoke-test module-doc claims 7 scenarios; comment/list maintenance

**File:** `verify/tests/compat_smoke.rs:4`

**Issue:** The module doc enumerates 7 scenarios (1–7) and the file defines exactly 7 `#[test]` functions, which match. No defect — flagged only as a reminder that the numbered list is hand-maintained and will silently drift if tests are added/removed. The `ffi_vs_typed_api_bit_equivalence` test (line 185) is the strongest guard (bit-exact FFI-vs-typed parity) and is correctly scoped behind the default (kernel-pulling) features per the file's RAM-constraint note.

**Fix:** None required. Optionally convert the numbered doc list to reference test names so drift is obvious.

---

_Reviewed: 2026-05-25T00:00:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
