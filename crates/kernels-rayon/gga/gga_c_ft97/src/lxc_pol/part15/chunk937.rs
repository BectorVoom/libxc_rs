//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 937/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk937(t20032: f64, t458: f64, t20028: f64, t20036: f64, t20024: f64, t20046: f64, t20049: f64, t3020: f64, t77: f64, t534: f64, t73777: f64, t11262: f64, t20031: f64, t419: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t73958 = t458 * t20032;
    let t73975 = t458 * t20028;
    let t73977 = t458 * t20036;
    let t73983 = t458 * t20024;
    let t73985 = t458 * t20046;
    let t74009 = t3020 * t77 * t20049;
    let t74034 = t534 * t73777;
    let t74068 = t419 * t11262 * t20031;
    (t73958, t73975, t73977, t73983, t73985, t74009, t74034, t74068)
}
