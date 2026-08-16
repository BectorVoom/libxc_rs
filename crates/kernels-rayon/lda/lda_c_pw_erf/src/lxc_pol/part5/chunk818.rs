//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 818/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk818(t6227: f64, t6234: f64, t6237: f64, t6240: f64, t6293: f64, t6295: f64, t6298: f64, t2402: f64, t835: f64, t2076: f64, t2480: f64, t6875: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7499 = 8.0_f64 / 15.0_f64 * t6227;
    let t7500 = 16.0_f64 / 45.0_f64 * t6234;
    let t7501 = 16.0_f64 / 45.0_f64 * t6237;
    let t7502 = 32.0_f64 / 45.0_f64 * t6240;
    let t7503 = 32.0_f64 / 45.0_f64 * t6293;
    let t7504 = 8.0_f64 / 15.0_f64 * t6295;
    let t7505 = 4.0_f64 / 15.0_f64 * t6298;
    let t7507 = 4.0_f64 / 5.0_f64 * t2402 * t835;
    let t7509 = 4.0_f64 / 5.0_f64 * t2076 * t2480;
    let t7511 = 4.0_f64 / 5.0_f64 * t6875 * t2480;
    (t7499, t7500, t7501, t7502, t7503, t7504, t7505, t7507, t7509, t7511)
}
