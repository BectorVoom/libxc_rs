//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 847/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk847(t109: f64, t3559: f64, t55: f64, t1243: f64, t1267: f64, t410: f64, t360: f64, t110: f64, t3560: f64, t1227: f64, t315: f64, t934: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8309 = t55 * t109 * t3559;
    let t8310 = t1243 * t8309;
    let t8312 = t410 * t1267;
    let t8313 = t360 * t8312;
    let t8315 = t110 * t3560;
    let t8316 = t360 * t8315;
    let t8323 = t934 * t315 * t1227;
    (t8309, t8310, t8312, t8313, t8315, t8316, t8323)
}
