//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 849/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk849(t45108: f64, t13548: f64, t731: f64, t11832: f64, t22090: f64, t2508: f64, t7291: f64, t11969: f64, t2592: f64, t10301: f64, t8045: f64, t11714: f64, t7324: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45109 = 0.32043859292259267849e-3_f64 * t45108;
    let t45110 = t731 * t13548;
    let t45111 = 0.42725145723012357132e-3_f64 * t45110;
    let t45115 = 0.1845726295234133828e0_f64 * t2508 * t22090 * t11832 * t7291;
    let t45124 = t2592 * t11969;
    let t45134 = 4.0_f64 * t8045 * t10301;
    let t45141 = 4.0_f64 * t7324 * t11714;
    (t45109, t45111, t45115, t45124, t45134, t45141)
}
