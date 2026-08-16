//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1361/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1361(t26336: f64, t27082: f64, t22035: f64, t1114: f64, t22046: f64, t1111: f64, t3097: f64, t530: f64, t26143: f64, t27048: f64, t27053: f64, t27056: f64, t27061: f64, t27063: f64, t27067: f64, t27074: f64, t27076: f64, t27079: f64, t3103: f64, t3116: f64, t3132: f64, t322: f64, t4357: f64, t8469: f64) -> (f64, f64, f64) {
    let t27083 = t27082 * t26336;
    let t27084 = t27083 * t22035;
    let t27088 = t1114 * t22046;
    let t27093 = t1111 * t530 * t3097;
    let t27095 = -0.18314556960919660338e2_f64 * t3132 * t27048 * t4357 - t27053 / 162.0_f64 - t27056 / 27.0_f64 - t27061 + 0.36629113921839320676e2_f64 * t3103 * t8469 * t27063 + 0.5680050638253047068e0_f64 * t3116 * t27067 * t26143 - t27074 + 5.0_f64 / 972.0_f64 * t27076 + 7.0_f64 / 486.0_f64 * t27079 + 35.0_f64 / 972.0_f64 * t1111 * t322 * t27084 + t1111 * t322 * t27088 / 288.0_f64 - t27093 / 216.0_f64;
    (t27084, t27088, t27095)
}
