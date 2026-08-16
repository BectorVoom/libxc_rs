//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 630/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk630(t3118: f64, t3120: f64, t241: f64, t3030: f64, t2921: f64, t2987: f64, t2990: f64, t2997: f64, t3015: f64, t3023: f64, t3066: f64, t3069: f64, t3073: f64, t3077: f64) -> (f64, f64, f64) {
    let t3121 = t3118 * t3120;
    let t3125 = 0.19751789702565206229e-1_f64 * t241 * t3030;
    let t3126 = -t2987 + t2990 - t2997 + t3015 + t3023 + t3066 + t3125 - t3069 + t2921 - t3073 - t3077;
    (t3121, t3125, t3126)
}
