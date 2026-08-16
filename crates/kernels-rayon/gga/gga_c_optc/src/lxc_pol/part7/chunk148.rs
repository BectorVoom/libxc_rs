//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 148/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk148(t314: f64, t320: f64, t324: f64, t331: f64) -> (f64, f64) {
    let t334 = 1.0_f64 + 0.86931614897887578546e-1_f64 * t320 * t324 + 0.75571056687546295932e-2_f64 * t331 * t314;
    let t335 = 1.0_f64 / t334;
    (t334, t335)
}
