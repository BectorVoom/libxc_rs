//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1095/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1095(t1992: f64, t3745: f64, t1308: f64, t2967: f64, t4818: f64, t571: f64, t10463: f64, t1325: f64, t2026: f64, t2031: f64, t2022: f64, t9313: f64) -> (f64, f64, f64, f64, f64) {
    let t12803 = 8.0_f64 / 9.0_f64 * t3745 * t1992;
    let t12807 = 8.0_f64 / 15.0_f64 * t571 * t1308 * t4818 * t2967;
    let t12809 = t1325 * t10463 * t2026;
    let t12810 = 16.0_f64 / 135.0_f64 * t12809;
    let t12812 = 8.0_f64 / 15.0_f64 * t3745 * t2031;
    let t12814 = t571 * t9313 * t2022;
    (t12803, t12807, t12810, t12812, t12814)
}
