---
phase: 11-splitter-v2-unified-5k-cap
plan: 07
task: 2
status: HALTED
captured: 2026-05-18
severity: blocking
trigger: batched_compile_sweep.py confirmed compile failure on `libxc-kernel-lda_c_pk09` in pass 2 (sequential).
---

# Phase 11-07 Sweep HALT — confirmed compile failure

## Critical Anti-Patterns

| # | name | severity | rule |
|---|---|---|---|
| sweep-halt | Confirmed per-`-p` compile failure on `libxc-kernel-lda_c_pk09` after pass-2 sequential retry. | blocking | The sweep is a pure orchestrator (D-31); a confirmed pass-2 failure is the real translator/codegen signal. Per AP-8 boundary: the sweep does NOT auto-patch — diagnosis routes through /gsd:discuss-phase. |

## Failure Detail

**Failing package:** `libxc-kernel-lda_c_pk09` (family: `lda`)

**Cargo invocation:** `cargo build -p libxc-kernel-lda_c_pk09 --jobs 1`

**Peak-RSS observed:** 17218.7 MB

**Batch index at halt:** 0

**Remaining unrun packages after halt:** 30

## Stderr digest (789 errors total; head + tail only)

```
   Compiling libxc-kernel-lda_c_pk09 v0.1.0 (/home/user/Documents/workspace/libxc_rs/crates/kernels/lda/lda_c_pk09)
error[E0308]: mismatched types
  --> crates/kernels/lda/lda_c_pk09/src/fxc_pol/part2/chunk5.rs:15:14
   |
11 | pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk5<F: Float>(t3: F, t5: F) -> (F, F, F) {
   |                                                 - expected this type parameter
...
15 |     (t6, t7, t8)
   |              ^^ expected type parameter `F`, found `f64`
   |
   = note: expected type parameter `F`
                        found type `f64`

error[E0308]: mismatched types
  --> crates/kernels/lda/lda_c_pk09/src/fxc_pol/part2/chunk7.rs:13:5
   |
11 | pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk7<F: Float>() -> F {
   |                                                 -              -
   |                                                 |              |
   |                                                 |              expected `F` because of return type
   |                                                 |              help: consider using an impl return type: `impl Float`
   |                                                 expected this type parameter
12 |     let t10 = M_CBRT3;
13 |     t10
   |     ^^^ expected type parameter `F`, found `f64`
   |
   = note: expected type parameter `F`
                        found type `f64`
   = note: the caller chooses a type for `F` which can be different from `f64`

error[E0308]: mismatched types
  --> crates/kernels/lda/lda_c_pk09/src/fxc_pol/part2/chunk7.rs:10:1
   |
10 | #[cube]
   | ^^^^^^^
   | |
   | expected `NativeExpand<F>`, found `f64`
   | expected `NativeExpand<F>` because of return type
   |
   = note: expected struct `NativeExpand<F>`
                found type `f64`
   = note: this error originates in the attribute macro `cube` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0308]: mismatched types
  --> crates/kernels/lda/lda_c_pk09/src/fxc_pol/part2/chunk8.rs:12:29
   |
11 | pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk8<F: Float>() -> (F, F) {
   |                                                 - expected this type parameter
12 |     let t11 = F::new(1.0) / M_PI;
   |               -----------   ^^^^ expected type parameter `F`, found `f64`
   |               |
   |               expected because this is `F`
   |
   = note: expected type parameter `F`
                        found type `f64`

error[E0308]: mismatched types
  --> crates/kernels/lda/lda_c_pk09/src/fxc_pol/part2/chunk8.rs:13:30
   |
11 | pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk8<F: Float>() -> (F, F) {
   |                                                 - found this type parameter
12 |     let t11 = F::new(1.0) / M_PI;
13 |     let t12 = pow_1_3::<f64>(t11);
   |               -------------- ^^^ expected `f64`, found type parameter `F`
   |               |
   |               arguments to this function are incorrect
   |
   = note:        expected type `f64`
           found type parameter `F`
help: the return type of this call is `F` due to the type of the argument passed
  --> crates/kernels/lda/lda_c_pk09/src/fxc_pol/part2/chunk8.rs:13:15
   |
13 |     let t12 = pow_1_3::<f64>(t11);
   |               ^^^^^^^^^^^^^^^---^
   |                              |
   |                              this argument influences the return type of `pow_1_3`
note: function defined here
  --> crates/kernels/math/src/powers.rs:23:8
   |
23 | pub fn pow_1_3<F: Float>(x: F) -> F {
   |        ^^^^^^^

error[E0308]: mismatched types
  --> crates/kernels/lda/lda_c_pk09/src/fxc_pol/part2/chunk8.rs:14:11
   |
11 | pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk8<F: Float>() -> (F, F) {
   |                                                 - expected this type parameter
...
14 |     (t11, t12)
   |           ^^^ expected type parameter `F`, found `f64`
   |
   = note: expected type parameter `F`
                        found type `f64`

error[E0277]: the trait bound `NativeExpand<F>: From<f64>` is not satisfied
  --> crates/kernels/lda/lda_c_pk09/src/fxc_pol/part2/chunk8.rs:10:1
   |
10 | #[cube]
   | ^^^^^^^ the trait `From<f64>` is not implemented for `NativeExpand<F>`
   |
   = note: required for `f64` to implement `Into<NativeExpand<F>>`
   = note: this error originates in the attribute macro `cube` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
10 | #[cube] where NativeExpand<F>: From<f64>
   |         ++++++++++++++++++++++++++++++++

error[E0277]: the trait bound `NativeExpand<f64>: From<NativeExpand<F>>` is not satisfied
  --> crates/kernels/lda/lda_c_pk09/src/fxc_pol/part2/chunk8.rs:10:1
   |
10 | #[cube]
   | ^^^^^^^ the trait `From<NativeExpand<F>>` is not implemented for `NativeExpand<f64>`
   |
   = note: required for `NativeExpand<F>` to implement `Into<NativeExpand<f64>>`
   = note: this error originates in the attribute macro `cube` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
10 | #[cube] where NativeExpand<f64>: From<NativeExpand<F>>
   |         ++++++++++++++++++++++++++++++++++++++++++++++

error[E0308]: mismatched types
  --> crates/kernels/lda/lda_c_pk09/src/fxc_pol/part2/chunk8.rs:13:9
   |
11 | pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk8<F: Float>() -> (F, F) {
   |                                                 - expected this type parameter
12 |     let t11 = F::new(1.0) / M_PI;
13 |     let t12 = pow_1_3::<f64>(t11);
   |         ^^^ expected `NativeExpand<F>`, found `NativeExpand<f64>`
   |
   = note: expected struct `NativeExpand<F>`
              found struct `NativeExpand<f64>`

error[E0308]: mismatched types
  --> crates/kernels/lda/lda_c_pk09/src/fxc_pol/part2/chunk10.rs:12:30
   |
11 | pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk10<F: Float>(t1: F) -> F {
   |                                                  - found this type parameter
12 |     let t14 = pow_1_3::<f64>(t1);
   |               -------------- ^^ expected `f64`, found type parameter `F`
   |               |
   |               arguments to this function are incorrect
   |
   = note:        expected type `f64`
           found type parameter `F`
help: the return type of this call is `F` due to the type of the argument passed
  --> crates/kernels/lda/lda_c_pk09/src/fxc_pol/part2/chunk10.rs:12:15
   |
12 |     let t14 = pow_1_3::<f64>(t1);
   |               ^^^^^^^^^^^^^^^--^
   |                              |

... [~700 middle errors truncated; all of form 'expected type parameter F, found f64' or 'expected NativeExpand<F>, found NativeExpand<f64>'] ...

note: function defined here
  --> crates/kernels/math/src/piecewise.rs:14:8
   |
14 | pub fn piecewise3<F: Float>(cond: bool, val_true: F, val_false: F) -> F {
   |        ^^^^^^^^^^

error[E0308]: mismatched types
  --> crates/kernels/lda/lda_c_pk09/src/kxc_pol/part2/chunk1019.rs:14:27
   |
11 | pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1019<F: Float>(t44: F, t11007: F, t11055: F, t1727: F, t2727: F, t427: F, t11033: F, z...
   |                                                    - expected this type parameter
...
14 |     let t11059 = t11033 + t11058;
   |                  ------   ^^^^^^ expected type parameter `F`, found `f64`
   |                  |
   |                  expected because this is `F`
   |
   = note: expected type parameter `F`
                        found type `f64`

error[E0277]: the trait bound `NativeExpand<f64>: From<NativeExpand<F>>` is not satisfied
  --> crates/kernels/lda/lda_c_pk09/src/kxc_pol/part2/chunk1019.rs:10:1
   |
10 | #[cube]
   | ^^^^^^^ the trait `From<NativeExpand<F>>` is not implemented for `NativeExpand<f64>`
   |
   = note: required for `NativeExpand<F>` to implement `Into<NativeExpand<f64>>`
   = note: this error originates in the attribute macro `cube` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
10 | #[cube] where NativeExpand<f64>: From<NativeExpand<F>>
   |         ++++++++++++++++++++++++++++++++++++++++++++++

error[E0277]: the trait bound `NativeExpand<F>: From<NativeExpand<f64>>` is not satisfied
  --> crates/kernels/lda/lda_c_pk09/src/kxc_pol/part2/chunk1019.rs:10:1
   |
10 | #[cube]
   | ^^^^^^^ the trait `From<NativeExpand<f64>>` is not implemented for `NativeExpand<F>`
   |
   = note: required for `NativeExpand<f64>` to implement `Into<NativeExpand<F>>`
   = note: this error originates in the attribute macro `cube` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
10 | #[cube] where NativeExpand<F>: From<NativeExpand<f64>>
   |         ++++++++++++++++++++++++++++++++++++++++++++++

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `libxc-kernel-lda_c_pk09` (lib) due to 788 previous errors

```

## Resume guidance

After diagnosis + fix, re-invoke the sweep with `--start-after libxc-kernel-lda_c_pk09` to
skip already-verified packages, OR re-run from scratch if the fix may have
affected earlier packages too.
