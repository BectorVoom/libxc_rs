//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 725/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk725(t1518: f64, t834: f64, t211: f64, t785: f64, t493: f64, t1: f64, t1124: f64) -> (f64, f64, f64, f64, f64) {
    let t4561 = t1518 * t834;
    let t4562 = t211 * t4561;
    let t4563 = 4.0_f64 / 135.0_f64 * t4562;
    let t4564 = t1518 * t785;
    let t4565 = t493 * t4564;
    let t4566 = 8.0_f64 / 135.0_f64 * t4565;
    let t4567 = t1 * t1124;
    (t4561, t4563, t4564, t4566, t4567)
}
