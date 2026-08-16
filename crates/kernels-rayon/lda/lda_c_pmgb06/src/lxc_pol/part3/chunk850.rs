//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 850/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk850(t1276: f64, t8352: f64, t1243: f64, t1180: f64, t361: f64, t360: f64, t1234: f64, t409: f64, t55: f64, t3600: f64, t1227: f64, t3594: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8353 = t1276 * t8352;
    let t8355 = t1243 * t8352;
    let t8357 = t1180 * t361;
    let t8358 = t360 * t8357;
    let t8369 = t55 * t409 * t1234;
    let t8370 = t3600 * t8369;
    let t8373 = t55 * t409 * t1227;
    let t8374 = t1243 * t8373;
    let t8376 = t3594 * t8369;
    (t8353, t8355, t8357, t8358, t8370, t8373, t8374, t8376)
}
