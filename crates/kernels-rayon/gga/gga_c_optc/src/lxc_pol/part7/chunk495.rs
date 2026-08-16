//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 495/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk495(t2441: f64, t847: f64, t2257: f64, t2259: f64, t2266: f64, t2272: f64, t2276: f64, t232: f64, t2280: f64, t2282: f64, t2286: f64, t2289: f64, t2292: f64, t2384: f64, t2392: f64, t2400: f64, t2402: f64) -> (f64, f64, f64, f64) {
    let t2443 = 0.11696446794910408142e1_f64 * t2441 * t847;
    let t2444 = 0.23744444444444444444e-1_f64 * t2257;
    let t2449 = t2444 + 0.11872222222222222222e-1_f64 * t2259 - 0.11872222222222222222e-1_f64 * t2266 + 0.35616666666666666666e-1_f64 * t2272 - 0.17808333333333333333e-1_f64 * t2276;
    let t2451 = 0.62182e-1_f64 * t2449 * t232;
    let t2454 = 0.40256666666666666667e0_f64 * t2257;
    let t2461 = 0.137975e0_f64 * t2280;
    let t2466 = -0.1294625e1_f64 * t2384 + 0.258925e1_f64 * t2392 + t2454 + 0.20128333333333333334e0_f64 * t2259 - 0.20128333333333333333e0_f64 * t2266 + 0.60385e0_f64 * t2272 - 0.301925e0_f64 * t2276 + 0.82524375e-1_f64 * t2400 + 0.16504875e0_f64 * t2402 + t2461 + 0.11038e0_f64 * t2282 - 0.27595e-1_f64 * t2286 + 0.16557e0_f64 * t2289 - 0.82785e-1_f64 * t2292;
    (t2443, t2449, t2451, t2466)
}
