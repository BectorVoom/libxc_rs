//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 409/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk409(t12: f64, t1489: f64, t176: f64, t166: f64, t161: f64, t1080: f64, t1083: f64, t44: f64, t131: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t13 = t12 <= zeta_threshold;
    let t1490 = t1489 * t176;
    let t1491 = t166 * t1490;
    let t1493 = t161 * t1491 / 30.0_f64;
    let t1497 = piecewise3(t13, 0.0_f64, 2.0_f64 * t12 * t1083 + 2.0_f64 * t1080);
    let t1498 = t1497 * t44;
    let t1499 = t1498 * t131;
    (t1490, t1491, t1493, t1498, t1499)
}
