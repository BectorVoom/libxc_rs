//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 629/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk629(t3530: f64, t69: f64, t3533: f64, t1227: f64, t342: f64, t3585: f64, t2247: f64, t2248: f64, t3505: f64, t3508: f64, t3517: f64, t3525: f64, t3561: f64, t3578: f64, t3580: f64, t3590: f64, t3602: f64, t3604: f64, t3607: f64, t3613: f64, t3643: f64) -> (f64, f64, f64, f64, f64) {
    let t3644 = t69 * t3530;
    let t3646 = t69 * t3533;
    let t3650 = t342 * t1227;
    let t3654 = t69 * t3585;
    let t3656 = -1.724255_f64 * t69 * t3561 - t3643 - 2.2990066666666666_f64 * t3644 + 1.724255_f64 * t3646 - t3505 - t3613 + t3508 - 20.69106_f64 * t69 * t3590 + 15.518295_f64 * t2247 * t2248 * t3650 - 5.172765_f64 * t3654 - t3517 + t3578 + t3525 + t3580 - t3607 - t3602 - t3604;
    (t3644, t3646, t3650, t3654, t3656)
}
