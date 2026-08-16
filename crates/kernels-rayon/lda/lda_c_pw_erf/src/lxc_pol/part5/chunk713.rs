//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 713/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk713(t2017: f64, t6379: f64, t571: f64, t2334: f64, t3589: f64, t352: f64, t4776: f64, t1943: f64, t34: f64, t4868: f64, t2027: f64, t4738: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6380 = t2017 * t6379;
    let t6382 = 8.0_f64 / 9.0_f64 * t571 * t6380;
    let t6383 = t3589 * t2334;
    let t6384 = t6383 * t352;
    let t6385 = t4776 * t6384;
    let t6387 = 32.0_f64 / 81.0_f64 * t571 * t6385;
    let t6388 = t1943 * t34;
    let t6389 = t4868 * t6388;
    let t6391 = 16.0_f64 / 27.0_f64 * t571 * t6389;
    let t6393 = 16.0_f64 / 45.0_f64 * t4738 * t2027;
    (t6380, t6382, t6383, t6384, t6385, t6387, t6388, t6389, t6391, t6393)
}
