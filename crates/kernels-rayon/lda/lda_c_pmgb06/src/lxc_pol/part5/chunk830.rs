//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 830/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk830(t5992: f64, t769: f64, t783: f64, t4232: f64, t113: f64, t301: f64, t7364: f64, t23: f64, t7277: f64, t2854: f64, t4718: f64, t4740: f64, t6327: f64, t6358: f64, t7445: f64, t7447: f64, t7448: f64, t7449: f64, t7450: f64, t7451: f64, t7452: f64, t7453: f64, t7454: f64, t7455: f64, t7456: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7917 = t5992 * t769;
    let t7920 = t783 * t769;
    let t7921 = t4232 * t7920;
    let t7934 = t7364 * t113 * t301;
    let t7937 = t7277 * t23;
    let t7945 = -t7445 + t2854 + 2.0_f64 / 45.0_f64 * t4718 + 0.09973633333333333_f64 * t4740 - t7447 - t7448 + t7449 + t7450 + t7451 + t7452 + t7453 + t7454 + t7455 + t7456 + 2.0_f64 / 3.0_f64 * t6327 - 2.0_f64 / 15.0_f64 * t6358;
    (t7917, t7920, t7921, t7934, t7937, t7945)
}
