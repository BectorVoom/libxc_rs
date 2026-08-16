//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 989/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk989(t4327: f64, t5313: f64, t17978: f64, t3109: f64, t155: f64, t17648: f64, t464: f64, t17855: f64, t438: f64, t449: f64, t894: f64, t18023: f64, t3151: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18085 = t4327 * t5313;
    let t18088 = t17978 * t3109;
    let t18092 = t155 * t464 * t17648;
    let t18098 = t449 * t17855 * t438;
    let t18099 = t894 * t18098;
    let t18102 = t3151 * t18023;
    (t18085, t18088, t18092, t18098, t18099, t18102)
}
