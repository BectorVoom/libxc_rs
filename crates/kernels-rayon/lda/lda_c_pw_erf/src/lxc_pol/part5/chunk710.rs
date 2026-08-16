//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 710/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk710(t2429: f64, t494: f64, t1991: f64, t1325: f64, t542: f64, t3402: f64, t519: f64, t2325: f64, t3476: f64, t348: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6343 = t2429 * t494;
    let t6344 = t1991 * t6343;
    let t6346 = 8.0_f64 / 27.0_f64 * t1325 * t6344;
    let t6347 = t2429 * t542;
    let t6348 = t3402 * t6347;
    let t6350 = 4.0_f64 / 27.0_f64 * t519 * t6348;
    let t6351 = t3476 * t2325;
    let t6352 = t6351 * t348;
    (t6343, t6344, t6346, t6347, t6348, t6350, t6351, t6352)
}
