//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 650/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk650(t277: f64, t2835: f64, t2841: f64, t2905: f64, t2908: f64, t2911: f64, t2921: f64, t2987: f64, t3066: f64, t3069: f64, t3073: f64, t3077: f64, t3293: f64, t498: f64, t95: f64) -> f64 {
    let t3294 = t2835 / 3.0_f64 - t2841 + t2905 * t498 / 2.0_f64 - 0.25844881434903430496e-2_f64 * t95 * t277 * t2908 * t2911 + t2921 + t3066 - t2987 - t3069 - t3073 - t3077 + t3293;
    t3294
}
