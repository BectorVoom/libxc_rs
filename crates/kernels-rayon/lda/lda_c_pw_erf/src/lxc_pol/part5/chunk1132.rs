//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1132/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1132(t15582: f64, t2035: f64, t2011: f64, t7007: f64, t2014: f64, t15926: f64, t6479: f64, t2018: f64, t2526: f64, t833: f64, t11983: f64, t571: f64, t593: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20963 = 8.0_f64 / 15.0_f64 * t15582 * t2035;
    let t20965 = 8.0_f64 / 15.0_f64 * t7007 * t2011;
    let t20967 = 16.0_f64 / 15.0_f64 * t7007 * t2014;
    let t20969 = 16.0_f64 / 15.0_f64 * t15926 * t6479;
    let t20971 = 8.0_f64 / 9.0_f64 * t7007 * t2018;
    let t20972 = t2526 * t833;
    let t20976 = 12.0_f64 / 5.0_f64 * t571 * t11983 * t20972 * t593;
    (t20963, t20965, t20967, t20969, t20971, t20972, t20976)
}
