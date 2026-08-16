//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 701/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk701(t5269: f64, t6243: f64, t1318: f64, t2035: f64, t4763: f64, t2011: f64, t2146: f64, t2014: f64, t2018: f64, t2419: f64, t549: f64, t1319: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6244 = t5269 * t6243;
    let t6246 = 16.0_f64 / 15.0_f64 * t1318 * t6244;
    let t6248 = 16.0_f64 / 45.0_f64 * t4763 * t2035;
    let t6250 = 8.0_f64 / 45.0_f64 * t2146 * t2011;
    let t6252 = 16.0_f64 / 45.0_f64 * t2146 * t2014;
    let t6254 = 8.0_f64 / 27.0_f64 * t2146 * t2018;
    let t6255 = t2419 * t549;
    let t6256 = t1319 * t6255;
    (t6244, t6246, t6248, t6250, t6252, t6254, t6255, t6256)
}
