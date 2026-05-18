# Phase 11 PATTERN.md — Canonical Generic Helper Conversion Pattern

**Authored:** 11-06 Task 3 (Direction A per D-25)
**Source:** All 5 Phase-1 clean files (powers.rs, piecewise.rs, lambert_w.rs, polynomials.rs, spin.rs)
**Mandate:** D-27 — canonical reference each manual conversion (Task 5) mirrors

## Purpose

This document is the canonical pattern reference for converting `crates/kernels/math/src/`
helpers from concrete f64 to generic `<F: Float>`. It is derived from the 5 Phase-1 files
that landed cleanly in commits `466e074d0` (powers/piecewise/lambert_w) and `d8cc4da0c`
(polynomials/spin). Any deviation from this pattern in Task 5 conversions MUST be justified
in the file's commit message and documented as a one-off exception in 11-06-SUMMARY.md.

## Rules

### Rule 1 — Generic signature shape

Concrete-f64 helpers convert to generic-over-F:

BEFORE (concrete f64):
```rust
#[cube]
pub fn safe_cbrt(x: f64) -> f64 { ... }
```

AFTER (generic <F: Float>):
```rust
#[cube]
pub fn safe_cbrt<F: Float>(x: F) -> F { ... }
```

Both `<F: Float>` and `pub` are required for cross-crate / cross-module callers from
`#[cube]` chunks in `crates/kernels/{lda,gga,mgga}/`.

Multi-output: `pub fn name(a: f64, b: f64) -> (f64, f64)` becomes
`pub fn name<F: Float>(a: F, b: F) -> (F, F)`.

### Rule 2 — Literal-wrap rule (F::new for f32-compatible literals)

Short float literals that fit f32 precision use `F::new(literal)`:

```rust
F::new(1.0), F::new(2.0), F::new(3.0), F::new(0.5), F::new(-1.0), F::new(0.0)
```

These ARE safe to wrap in `F::new` because `Float::new(val: f32)` accepts an f32 argument
(per CubeCL 0.10 `cubecl-core/src/frontend/element/float.rs:75`). A literal `1.0` is an
unsuffixed numeric literal that Rust coerces to f32 at the call site.

**Exception:** integer-like positions (e.g. `for _ in 0..N`, `arr[3]`, `0u32`) — leave
concrete integer literals.

### Rule 3 — Named-f64-const rule (F::cast_from for f64-precision constants)

Named constants defined as `f64` (e.g. `SQRT_DBL_EPSILON`, `M_PI`, `M_CBRT3`, `KF_CONST`,
`RS_CONST`, `LOG_DBL_MAX`, `TWO_DBL_MIN`, `ERX`, `PI_TWO_THIRDS`, `POW_32PI_TWO_THIRDS`,
`f64::MAX`, ...) use `F::cast_from(<const>)`:

```rust
let eps = F::cast_from(SQRT_DBL_EPSILON);
let pi = F::cast_from(M_PI);
let max = F::cast_from(f64::MAX);
```

**Do NOT use F::new(NAMED_F64_CONST)** — that is the broken Phase-2 pattern that
triggered E0308 `expected f32, found f64` errors. `F::new` accepts only f32; an f64
named const must go through the `Cast` trait's `cast_from` method (per
`cubecl-core/src/frontend/element/cast.rs:14-37`, blanket `impl<P: CubePrimitive> Cast for P`).

### Rule 4 — Doc-comment / string-literal handling

Doc-comments (`///`) and string literals (`"…"`) are **NEVER** wrapped in `F::new`. The
broken Phase-2 auto-script corrupted prose like `LDA`, `MGGA`, `BR89`, `"17.5K"` into
`F::new(LDA)`, `F::new(MGGA)`, `F::new("17.)5K"`. Direction A reverts those automatically
via Task 2's path-scoped reset to `d8cc4da0c` (state-equivalent to whole-reverting
`7a65f3bc6`/`dcb7d517d`/`233a8890d` for math/src/).

When converting Phase-2 files in Task 5: preserve every doc-comment and string-literal
exactly as it was BEFORE the auto-script ran. The reset from Task 2 already restores
these — no manual change to doc-comments or string literals is required (or permitted).

### Rule 5 — Type-annotation handling

When the right-hand side becomes generic, the type annotation MUST also be generic:

BEFORE:
```rust
let mut acc: f64 = 0.0;
```

AFTER:
```rust
let mut acc: F = F::new(0.0);
```

The broken Phase-2 pattern `let mut acc: f64 = F::new(0.0)` is a type mismatch — `F::new`
returns `F`, not `f64`. D-23's surgical commit `9df2880b3` fixed 6 such sites in
`bessel.rs`; the Task 2 reset to `d8cc4da0c` predates that corruption entirely.

### Rule 6 — `#[cube]` pub-visibility

Every helper called from a `#[cube]` chunk in `crates/kernels/{lda,gga,mgga}/` MUST be `pub`
(or `pub(crate)` / `pub(super)` for module-local helpers). CubeCL's macro expansion
performs an accessibility check that fails if a `#[cube]` helper is private.

The broken Phase-2 baseline had some non-pub `#[cube]` helpers — these were one of the
4 corruption categories beyond D-23's scope that caused the 4th-iter HALT. When converting
in Task 5, **every `#[cube] fn` that is called externally MUST be `pub`** (or one of the
restricted-pub variants). Check call sites in `crates/kernels/{lda,gga,mgga}/` (read-only
inspection) before deciding the visibility level.

### Rule 7 — Mixed-precision signatures (special case)

If a helper legitimately takes a non-F parameter (e.g. `idx: usize`) or returns a non-F
type (e.g. `pub fn is_deferred(id: u16) -> bool`), document the exception in the FILE'S
HEADER COMMENT, not silently inline. Examples from Phase-1 (`spin.rs`):

```rust
/// Compute spin polarization zeta = (rho_up - rho_down) / (rho_up + rho_down).
/// Note: `threshold: F` parameter is the unpolarized-clamp threshold;
/// the *float* portion of the signature is fully generic.
#[cube]
pub fn compute_zeta<F: Float>(rho_up: F, rho_down: F, threshold: F) -> F { ... }
```

Default expectation: fully-generic over `<F: Float>`. Mixed-precision is a case-by-case
deviation requiring a header-comment justification.

### Rule 8 — Non-generic file exclusion list

The following files in `crates/kernels/math/src/` are **NOT** generic-over-F and are NOT
in the 9-file conversion set for Task 5:

| File | Why excluded |
|---|---|
| powers.rs | Phase-1 clean (already generic; PRESERVE per D-25) |
| piecewise.rs | Phase-1 clean (already generic; PRESERVE) |
| lambert_w.rs | Phase-1 clean (already generic; PRESERVE) |
| polynomials.rs | Phase-1 clean (already generic; PRESERVE) |
| spin.rs | Phase-1 clean (already generic; PRESERVE) |
| deferred.rs | Registry-shaped (`pub fn is_deferred(id: u16) -> bool`); not parameterized over F; concrete by design (Task 2 reset restored pre-Phase-2 state which coincides with D-23 9df2880b3 full revert) |
| constants.rs | Constants-only module; no functions to convert |
| lib.rs | Module declarations only |

The 9 conversion targets are: bspline, dft_quantities, erf, special, mbrxc, br89,
integrate, expint_e1, bessel (in easiest-first order; bessel LAST per D-26).

## Concrete Example: From Phase-1 powers.rs

```rust
use cubecl::prelude::*;

/// Compute the cube root of x, correctly handling negative values.
///
/// Standard `powf(x, 1/3)` returns NaN for negative x. This function
/// extracts the sign, computes `|x|^(1/3)`, and restores the sign.
#[cube]
pub fn safe_cbrt<F: Float>(x: F) -> F {
    let abs_x = F::abs(x);
    let cbrt_abs = F::powf(abs_x, F::new(1.0) / F::new(3.0));
    let sign = select(x < F::new(0.0), F::new(-1.0), F::new(1.0));
    select(x == F::new(0.0), F::new(0.0), sign * cbrt_abs)
}
```

Rules applied:
- Rule 1 (generic signature `<F: Float>`)
- Rule 2 (short literals via `F::new(1.0)`, `F::new(3.0)`, `F::new(-1.0)`, `F::new(0.0)`)
- Rule 6 (pub visibility)
- Rule 4 (doc-comment preserved untouched)

## Concrete Example: F::cast_from for named f64 const (illustrative — Phase-1 files happen not to use any named consts directly, but the pattern applies whenever a Phase-2 file imports an f64 const from `constants.rs` or uses `f64::MAX` etc.)

```rust
use crate::constants::SQRT_DBL_EPSILON;

#[cube]
pub fn series_with_eps<F: Float>(x: F) -> F {
    let eps = F::cast_from(SQRT_DBL_EPSILON);   // Rule 3 — NOT F::new(SQRT_DBL_EPSILON)
    // ... body uses eps for convergence threshold
    select(x < eps, F::new(0.0), x)
}
```

Rules applied:
- Rule 1 (generic signature)
- Rule 3 (`F::cast_from` for named f64 const — NOT `F::new`)
- Rule 6 (pub visibility)

## Concrete Example: From Phase-1 spin.rs (multi-output / select)

```rust
/// Compute spin polarization zeta = (rho_up - rho_down) / (rho_up + rho_down).
///
/// If total density is below `threshold`, returns 0.0 (unpolarized).
#[cube]
pub fn compute_zeta<F: Float>(rho_up: F, rho_down: F, threshold: F) -> F {
    let total = rho_up + rho_down;
    let zeta = (rho_up - rho_down) / total;
    select(total < threshold, F::new(0.0), zeta)
}
```

Rules applied:
- Rule 1 (generic signature, multiple F-typed params)
- Rule 2 (`F::new(0.0)` short-literal wrap)
- Rule 4 (doc-comment preserved)
- Rule 6 (pub visibility)

## Anti-Examples (corruption patterns Task 2's reset removed; Task 5 MUST NOT re-introduce)

| Anti-pattern | Why broken | Correct form |
|---|---|---|
| `F::new(SQRT_DBL_EPSILON)` | `Float::new(val: f32)` rejects f64 | `F::cast_from(SQRT_DBL_EPSILON)` |
| `F::new(LDA)` in doc-comment | LDA is prose text, not a value | preserve original doc-comment text untouched |
| `F::new("17.)5K"` (corrupted string) | string literals are not numeric | preserve original string literal |
| `let mut x: f64 = F::new(0.0)` | type mismatch (F vs f64) | `let mut x: F = F::new(0.0)` |
| `for _ in 0.F::new(.500)` | mis-wrapped range operator | `for _ in 0..500` |
| `F::new(3.)0_f64` | mis-wrapped literal suffix | `F::new(3.0)` |
| `F::F::new(MAX)` | double-wrap of `f64::MAX` | `F::cast_from(f64::MAX)` |
| `#[cube] fn helper<F: Float>(...)` (no `pub`) | private fn rejected by CubeCL macro expansion when called externally | `#[cube] pub fn helper<F: Float>(...)` |

## Conversion Checklist (apply per file in Task 5)

For each file:

1. Read the FULL file pre-conversion (use Read tool; do not skip the body).
2. Replace each `fn <name>(args: f64, ...) -> f64 { ... }` with `fn <name><F: Float>(args: F, ...) -> F { ... }`.
3. Inside the body: rewrite `1.0`, `2.0`, `0.5`, `-1.0` etc. as `F::new(1.0)` etc. (Rule 2).
4. Inside the body: rewrite `SQRT_DBL_EPSILON`, `M_PI`, `f64::MAX`, named f64 consts as `F::cast_from(CONST)` (Rule 3).
5. Type-annotations: rewrite `let mut x: f64 = ...` as `let mut x: F = ...` when RHS is F (Rule 5).
6. Doc-comments and string-literals: leave untouched (Rule 4 — already correct after Task 2 reset).
7. Visibility: confirm `pub` on every helper called from outside the file (Rule 6).
8. Mixed-precision signatures: document as a one-off in the file header if used (Rule 7).
9. Compile gate: `cargo build -p libxc-kernel-math` exit 0 at f64.
10. Compile + parity gate: same at f32 under `LIBXC_RS_F32=1`.
11. Atomic commit: `git commit --only -- crates/kernels/math/src/<file>.rs` with "green at both precisions".

If any rule cannot be applied cleanly (e.g. a body construct that doesn't fit the
pattern), HALT for that file, document the obstacle in 11-06-SUMMARY.md, and trigger
discuss-phase. Do NOT improvise a deviation — Direction A's correctness depends on
pattern fidelity.

## Cross-Reference

- CONTEXT.md D-25 (Direction A lock)
- CONTEXT.md D-26 (conversion mechanics: cadence/order/done-criterion)
- CONTEXT.md D-27 (this PATTERN.md's mandate)
- CONTEXT.md D-28 (cast_from classifier preservation policy — relevant to Task 7)
- 11-06-SUMMARY-HALT.md (3rd-iter HALT; pre-cast_from baseline)
- 11-06-SUMMARY-HALT-4TH.md (4th-iter HALT; cast_from policy applied but Gate 2 unreachable — empirical evidence of the corruption patterns Rule 4/5/6/7 prevent)
- cubecl-core 0.10 API: `Float::new` at `frontend/element/float.rs:75`, `Cast::cast_from` at `frontend/element/cast.rs:14-37`
