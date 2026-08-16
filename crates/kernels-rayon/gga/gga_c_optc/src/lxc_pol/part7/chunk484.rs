//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 484/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk484(t2391: f64, t780: f64, t2257: f64, t214: f64, t2383: f64, t787: f64, t2280: f64, t2259: f64, t2266: f64, t2272: f64, t2276: f64, t2282: f64, t2286: f64, t2289: f64, t2292: f64, t2384: f64) -> (f64, f64, f64, f64, f64) {
    let t2392 = t780 * t2391;
    let t2394 = 0.39862222222222222223e0_f64 * t2257;
    let t2399 = 1.0_f64/f64::sqrt(t214);
    let t2400 = t2399 * t2383;
    let t2402 = t787 * t2391;
    let t2404 = 0.13692777777777777778e0_f64 * t2280;
    let t2409 = -0.9494625e0_f64 * t2384 + 0.1898925e1_f64 * t2392 + t2394 + 0.19931111111111111111e0_f64 * t2259 - 0.19931111111111111111e0_f64 * t2266 + 0.59793333333333333334e0_f64 * t2272 - 0.29896666666666666667e0_f64 * t2276 + 0.15358125e0_f64 * t2400 + 0.3071625e0_f64 * t2402 + t2404 + 0.10954222222222222222e0_f64 * t2282 - 0.27385555555555555556e-1_f64 * t2286 + 0.16431333333333333333e0_f64 * t2289 - 0.82156666666666666667e-1_f64 * t2292;
    (t2392, t2399, t2400, t2402, t2409)
}
