//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 341/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk341(t1067: f64, t1086: f64, t1448: f64, t1462: f64, t1464: f64, t1472: f64, t1477: f64, t1484: f64, t241: f64, t402: f64, t1085: f64, t1094: f64, t1483: f64) -> (f64, f64, f64) {
    let t1488 = t241 * (-0.3109e-1_f64 * t1464 * t402 + 1.0_f64 * t1067 * t1472 + t1448 - t1462 - 0.19751789702565206229e-1_f64 * t1477 + 0.58482233974552040708e0_f64 * t1086 * t1484);
    let t1490 = 0.19751789702565206229e-1_f64 * t241 * t1477;
    let t1492 = t1085 * t1483 * t1094;
    (t1488, t1490, t1492)
}
