//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 851/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk851(t2724: f64, t8143: f64, t2812: f64, t852: f64, t911: f64, t115: f64, t2718: f64) -> (f64, f64, f64) {
    let t8144 = t8143 * t2724;
    let t8145 = t2812 * t8144;
    let t8147 = t852 * t911;
    let t8148 = t8147 * t115;
    let t8149 = t2718 * t8148;
    (t8144, t8145, t8149)
}
