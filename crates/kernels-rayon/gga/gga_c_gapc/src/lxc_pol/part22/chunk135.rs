//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 135/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk135(t104: f64, t442: f64, t14: f64, t23: f64, t402: f64, t70: f64, t105: f64, t107: f64, t108: f64, t405: f64, t438: f64, t73: f64) -> (f64, f64, f64, f64, f64) {
    let t443 = t104 * t104;
    let t444 = t443 * t443;
    let t445 = t444 * t104;
    let t446 = t442 * t445;
    let t447 = t23 * t14;
    let t451 = t70 * t402;
    let t457 = 0.13140859333333333333e-2_f64 * t105 * t438 * t108 - 0.98556444999999999995e-3_f64 * t446 * t447 * t108 - 0.19711288999999999999e-2_f64 * t105 * t107 * t451 - 4.0_f64 * t73 * t405;
    (t445, t446, t447, t451, t457)
}
