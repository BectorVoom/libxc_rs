//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 489/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk489(t2374: f64, t2418: f64, t2416: f64, t2257: f64, t2259: f64, t2266: f64, t2272: f64, t2276: f64) -> (f64, f64, f64) {
    let t2419 = t2374 * t2418;
    let t2421 = 0.16081824322151104822e2_f64 * t2416 * t2419;
    let t2422 = 0.12361111111111111111e-1_f64 * t2257;
    let t2427 = t2422 + 0.61805555555555555556e-2_f64 * t2259 - 0.61805555555555555555e-2_f64 * t2266 + 0.18541666666666666667e-1_f64 * t2272 - 0.92708333333333333333e-2_f64 * t2276;
    (t2419, t2421, t2427)
}
