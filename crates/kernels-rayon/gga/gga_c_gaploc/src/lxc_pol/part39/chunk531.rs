//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 531/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk531(t3141: f64, t501: f64, t3145: f64, t605: f64, t2497: f64, t921: f64, t3207: f64, t584: f64, t6575: f64) -> (f64, f64, f64, f64, f64) {
    let t9243 = t3141 * t501;
    let t9253 = t3145 * t605;
    let t9256 = t921 * t2497;
    let t9260 = t3207 * t605;
    let t9263 = t584 * t6575;
    (t9243, t9253, t9256, t9260, t9263)
}
