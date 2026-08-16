//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 791/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk791(t5302: f64, t571: f64, t1333: f64, t833: f64, t951: f64, t1308: f64, t2026: f64, t3859: f64, t1325: f64, t1469: f64, t4763: f64, t2065: f64, t581: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5304 = 16.0_f64 / 135.0_f64 * t571 * t5302;
    let t5305 = t833 * t1333;
    let t5306 = t5305 * t951;
    let t5307 = t1308 * t5306;
    let t5309 = 8.0_f64 / 45.0_f64 * t571 * t5307;
    let t5310 = t3859 * t2026;
    let t5312 = 32.0_f64 / 135.0_f64 * t1325 * t5310;
    let t5314 = 8.0_f64 / 15.0_f64 * t4763 * t1469;
    let t5315 = t581 * t2065;
    (t5304, t5306, t5307, t5309, t5310, t5312, t5314, t5315)
}
