//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 249/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk249(t23: f64, t287: f64, t116: f64, t286: f64, t115: f64, t285: f64) -> (f64, f64, f64) {
    let t857 = t23 * t287;
    let t858 = t116 * t857;
    let t860 = t286 * t858 / 288.0_f64;
    let t861 = t285 * t115;
    (t857, t860, t861)
}
