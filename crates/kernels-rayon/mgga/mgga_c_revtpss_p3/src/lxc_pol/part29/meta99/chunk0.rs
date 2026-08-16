//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 601/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk601(t2259: f64, t70: f64, t607: f64, t627: f64, t362: f64, t41: f64, t47: f64, t2251: f64, t2258: f64, t48: f64, t59: f64, t60: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2260 = t2259 * t70;
    let t2263 = t607 * t627;
    let t2269 = 1.0_f64 / t41 / t362;
    let t2270 = sigma0 * t2269;
    let t2275 = 1.0_f64 / t47;
    let t2276 = t2275 * t2251;
    let t2279 = t48 * t2258;
    let t2282 = 1.0_f64 / t59;
    let t2283 = t2282 * t2251;
    let t2286 = t60 * t2258;
    (t2260, t2263, t2270, t2275, t2276, t2279, t2282, t2283, t2286)
}
