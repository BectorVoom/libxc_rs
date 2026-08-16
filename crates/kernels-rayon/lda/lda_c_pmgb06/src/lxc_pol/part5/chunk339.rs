//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 339/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk339(t435: f64, t478: f64, t132: f64, t458: f64, t464: f64, t398: f64, t539: f64, t188: f64, t947: f64, t955: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1392 = t435 * t478;
    let t1393 = t132 * t1392;
    let t1395 = t458 * t464;
    let t1403 = t398 * t539;
    let t1404 = t1403 * t188;
    let t1409 = -0.55_f64 * t947 + 5.0_f64 / 18.0_f64 * t955;
    (t1392, t1393, t1395, t1403, t1404, t1409)
}
