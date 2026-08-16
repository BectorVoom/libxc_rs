//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 631/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk631(t1: f64, t3126: f64, t438: f64, t450: f64, t2667: f64, t465: f64) -> (f64, f64, f64, f64) {
    let t3127 = t3126 * t1;
    let t3128 = t3127 * t438;
    let t3129 = t450 * t3128;
    let t3132 = t465 * t2667;
    (t3127, t3128, t3129, t3132)
}
