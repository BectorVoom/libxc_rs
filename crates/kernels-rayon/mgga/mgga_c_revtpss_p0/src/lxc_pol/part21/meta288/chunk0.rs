//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1526/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1526(t10326: f64, t36: f64, t70: f64, t2259: f64, t627: f64, t2291: f64, t607: f64, t363: f64, t41: f64, t46: f64, t47: f64, t2251: f64, t606: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10327 = t36 * t10326;
    let t10328 = t10327 * t70;
    let t10331 = t2259 * t627;
    let t10336 = t607 * t2291;
    let t10344 = 1.0_f64 / t41 / t363;
    let t10345 = sigma0 * t10344;
    let t10355 = 1.0_f64 / t47 / t46;
    let t10356 = t2251 * t606;
    (t10327, t10328, t10331, t10336, t10345, t10355, t10356)
}
