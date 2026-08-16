//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 467/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk467(t2258: f64, t2259: f64, t2266: f64, t2272: f64, t2276: f64, t2281: f64, t2282: f64, t2286: f64, t2289: f64, t2292: f64, t970: f64, t973: f64) -> (f64, f64) {
    let t2294 = t2258 + 0.12925555555555555555e1_f64 * t2259 - 0.12925555555555555555e1_f64 * t2266 + 0.38776666666666666666e1_f64 * t2272 - 0.19388333333333333333e1_f64 * t2276 + t2281 + 0.1642e-2_f64 * t2282 - 0.4105e-3_f64 * t2286 + 0.2463e-2_f64 * t2289 - 0.12315e-2_f64 * t2292;
    let t2296 = t970 * t973;
    (t2294, t2296)
}
