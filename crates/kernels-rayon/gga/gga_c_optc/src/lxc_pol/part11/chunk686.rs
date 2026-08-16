//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 686/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk686(t601: f64, t6642: f64, t103: f64, t193: f64, t197: f64, t652: f64, t102: f64, t133: f64, t115: f64, t2139: f64, t2048: f64, t592: f64) -> (f64, f64, f64, f64, f64) {
    let t6644 = 0.58482233974552040708e0_f64 * t601 * t6642;
    let t6653 = 15400.0_f64 / 243.0_f64 * t193 * t652 * t103 * t197;
    let t6654 = t133 * t102;
    let t6680 = t2139 * t115;
    let t6695 = t2048 * t592;
    (t6644, t6653, t6654, t6680, t6695)
}
