//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1031/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1031(t2262: f64, t2268: f64, t136: f64, t7253: f64, t56: f64, t7856: f64, t209: f64, t212: f64, t2139: f64) -> (f64, f64, f64, f64, f64) {
    let t23548 = 1.0_f64 / t2262 / t2268;
    let t23571 = t136 * t7253;
    let t23572 = t2262 * t2262;
    let t23573 = 1.0_f64 / t23572;
    let t23628 = t56 * t7856;
    let t23682 = t209 * t2139 * t212;
    (t23548, t23571, t23573, t23628, t23682)
}
