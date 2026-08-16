//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 706/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk706(t6298: f64, t3433: f64, t2425: f64, t568: f64, t2467: f64, t514: f64, t211: f64, t2472: f64, t185: f64, t3551: f64, t3554: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6299 = 4.0_f64 / 45.0_f64 * t6298;
    let t6300 = 4.0_f64 / 135.0_f64 * t3433;
    let t6301 = t2425 * t568;
    let t6302 = 4.0_f64 / 45.0_f64 * t6301;
    let t6303 = t514 * t2467;
    let t6304 = t211 * t6303;
    let t6305 = 8.0_f64 / 45.0_f64 * t6304;
    let t6306 = t514 * t2472;
    let t6307 = t185 * t6306;
    let t6308 = 8.0_f64 / 45.0_f64 * t6307;
    let t6309 = 8.0_f64 / 135.0_f64 * t3551;
    let t6310 = 4.0_f64 / 135.0_f64 * t3554;
    (t6299, t6300, t6301, t6302, t6303, t6304, t6305, t6306, t6307, t6308, t6309, t6310)
}
