//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 886/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk886(t13703: f64, t16630: f64, t16634: f64, t16638: f64, t16642: f64, t16646: f64, t16650: f64, t16743: f64, t16747: f64, t16750: f64, t16756: f64, t16759: f64, t16763: f64, t16766: f64) -> f64 {
    let t16769 = 0.51647499999999999999e0_f64 * t13703 + 0.3529725e1_f64 * t16743 - 0.516475e0_f64 * t16650 - 0.62517e0_f64 * t16747 + 0.20839e0_f64 * t16750 + 0.20659e1_f64 * t16634 - 0.309885e1_f64 * t16642 - 0.57386111111111111112e0_f64 * t16630 - 0.46308888888888888889e-1_f64 * t16756 - 0.104195e0_f64 * t16759 + 0.309885e1_f64 * t16646 + 0.62517e0_f64 * t16763 - 0.104195e0_f64 * t16766 - 0.103295e1_f64 * t16638;
    t16769
}
