//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1082/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1082(t2171: f64, t3880: f64, t3884: f64, t9602: f64, t1383: f64, t1960: f64, t3657: f64, t822: f64, t9619: f64, t1289: f64, t6851: f64, t9621: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12665 = t2171 * t3880;
    let t12666 = 8.0_f64 / 45.0_f64 * t12665;
    let t12667 = t2171 * t3884;
    let t12668 = 8.0_f64 / 27.0_f64 * t12667;
    let t12669 = 8.0_f64 / 15.0_f64 * t9602;
    let t12671 = 2.0_f64 / 5.0_f64 * t1960 * t1383;
    let t12673 = 2.0_f64 / 15.0_f64 * t822 * t3657;
    let t12674 = 8.0_f64 / 45.0_f64 * t9619;
    let t12676 = 4.0_f64 / 5.0_f64 * t6851 * t1289;
    let t12677 = 8.0_f64 / 15.0_f64 * t9621;
    (t12666, t12668, t12669, t12671, t12673, t12674, t12676, t12677)
}
