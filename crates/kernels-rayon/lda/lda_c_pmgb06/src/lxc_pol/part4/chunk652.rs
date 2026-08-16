//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 652/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk652(t186: f64, t409: f64, t55: f64, t543: f64, t1400: f64, t27: f64, t545: f64, t1403: f64, t534: f64, t97: f64, t1377: f64, t1410: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3309 = t55 * t409 * t186;
    let t3311 = 0.09618703433213194_f64 * t543 * t3309;
    let t3312 = t1400 * t27;
    let t3313 = t3312 * t545;
    let t3315 = t1403 * t27;
    let t3316 = t3315 * t545;
    let t3319 = t534 * t97;
    let t3320 = t3319 * t1377;
    let t3322 = t1410 * t27;
    (t3309, t3311, t3312, t3313, t3315, t3316, t3319, t3320, t3322)
}
