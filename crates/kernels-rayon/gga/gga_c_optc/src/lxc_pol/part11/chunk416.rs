//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 416/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk416(t23: f64, t6: f64, t149: f64, t209: f64, t212: f64, t56: f64, t896: f64) -> (f64, f64, f64, f64) {
    let t2251 = t6 * t23;
    let t2257 = t209 * t149 * t212;
    let t2258 = 0.25851111111111111111e1_f64 * t2257;
    let t2261 = t56 * t896;
    (t2251, t2257, t2258, t2261)
}
