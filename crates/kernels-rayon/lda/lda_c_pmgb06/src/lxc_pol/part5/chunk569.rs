//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 569/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk569(t315: f64, t342: f64, t934: f64, t3566: f64, t1238: f64, t56: f64, t97: f64, t409: f64, t55: f64, t1276: f64, t1243: f64, t19: f64, t369: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3568 = t934 * t315 * t342;
    let t3569 = t3566 * t3568;
    let t3576 = t1238 * t56 * t97;
    let t3577 = t3576 * t3568;
    let t3582 = t55 * t409 * t342;
    let t3583 = t1276 * t3582;
    let t3603 = t1243 * t3582;
    let t3615 = 1.0_f64 / t369 / t19;
    (t3569, t3576, t3577, t3583, t3603, t3615)
}
