//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1050/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1050(t12299: f64, t1443: f64, t1287: f64, t1318: f64, t1466: f64, t5315: f64, t3899: f64, t5321: f64, t571: f64, t3663: f64, t822: f64, t1294: f64, t1960: f64) -> (f64, f64, f64, f64, f64) {
    let t12301 = 8.0_f64 / 5.0_f64 * t12299 * t1443;
    let t12305 = 4.0_f64 / 5.0_f64 * t1318 * t1466 * t5315 * t1287;
    let t12307 = t571 * t3899 * t5321;
    let t12308 = 16.0_f64 / 15.0_f64 * t12307;
    let t12309 = t822 * t3663;
    let t12310 = 4.0_f64 / 45.0_f64 * t12309;
    let t12311 = t1960 * t1294;
    (t12301, t12305, t12308, t12310, t12311)
}
