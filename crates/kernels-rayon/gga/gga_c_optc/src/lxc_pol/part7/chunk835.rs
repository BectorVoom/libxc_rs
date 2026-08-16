//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 835/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk835(t2693: f64, t956: f64, t2818: f64, t2612: f64, t871: f64, t938: f64, t2708: f64, t2789: f64, t2367: f64, t2785: f64, t913: f64, t115: f64, t2770: f64, t852: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7953 = t2693 * t956;
    let t7954 = t7953 * t2818;
    let t7958 = t938 * t2612 * t871;
    let t7961 = t2708 * t2789;
    let t7965 = t2367 * t2785;
    let t7966 = t913 * t7965;
    let t7969 = t852 * t2770 * t115;
    (t7953, t7954, t7958, t7961, t7965, t7966, t7969)
}
