//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 439/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk439(t102: f64, t128: f64, t1664: f64, t120: f64, t1568: f64, t118: f64, t119: f64, t473: f64, t156: f64, t427: f64, t426: f64, t436: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1667 = 5.84605_f64 * t102 * t128 * t1664;
    let t1670 = 2.923025_f64 * t102 * t120 * t1568;
    let t1674 = t118 * t119 * t473 * t120 / 9.0_f64;
    let t1675 = t156 * t427;
    let t1676 = t426 * t1675;
    let t1678 = t436 * t1664;
    (t1667, t1670, t1674, t1675, t1676, t1678)
}
