//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 502/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk502(t2494: f64, t818: f64, t2257: f64, t2280: f64, t2259: f64, t2266: f64, t2272: f64, t2276: f64, t2282: f64, t2286: f64, t2289: f64, t2292: f64, t2384: f64, t2392: f64, t2400: f64, t2402: f64) -> (f64, f64) {
    let t2495 = t2494 * t818;
    let t2500 = 0.68863333333333333333e0_f64 * t2257;
    let t2507 = 0.17365833333333333333e0_f64 * t2280;
    let t2512 = -0.17648625e1_f64 * t2384 + 0.3529725e1_f64 * t2392 + t2500 + 0.34431666666666666666e0_f64 * t2259 - 0.34431666666666666667e0_f64 * t2266 + 0.103295e1_f64 * t2272 - 0.516475e0_f64 * t2276 + 0.31558125e0_f64 * t2400 + 0.6311625e0_f64 * t2402 + t2507 + 0.13892666666666666667e0_f64 * t2282 - 0.34731666666666666667e-1_f64 * t2286 + 0.20839e0_f64 * t2289 - 0.104195e0_f64 * t2292;
    (t2495, t2512)
}
