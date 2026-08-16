//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 469/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk469(t2257: f64, t2280: f64, t2259: f64, t2266: f64, t2272: f64, t2276: f64, t2282: f64, t2286: f64, t2289: f64, t2292: f64) -> f64 {
    let t2305 = 0.96922222222222222222e3_f64 * t2257;
    let t2310 = 0.13111111111111111111e3_f64 * t2280;
    let t2315 = t2305 + 0.48461111111111111112e3_f64 * t2259 - 0.48461111111111111111e3_f64 * t2266 + 0.14538333333333333333e4_f64 * t2272 - 0.72691666666666666667e3_f64 * t2276 + t2310 + 0.10488888888888888889e3_f64 * t2282 - 0.26222222222222222222e2_f64 * t2286 + 0.15733333333333333333e3_f64 * t2289 - 0.78666666666666666667e2_f64 * t2292;
    t2315
}
