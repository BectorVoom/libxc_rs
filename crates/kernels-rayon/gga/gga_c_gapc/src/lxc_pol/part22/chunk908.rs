//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 908/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk908(t10336: f64, t3227: f64, t297: f64, t493: f64, t7371: f64, t3217: f64, t3224: f64, t8350: f64, t268: f64, t2920: f64) -> (f64, f64, f64, f64, f64) {
    let t10337 = t10336 * t3227;
    let t10339 = t493 * t297;
    let t10340 = t10339 * t7371;
    let t10341 = t3217 * t10340;
    let t10343 = t8350 * t3224;
    let t10344 = t10343 * t3227;
    let t10346 = t2920 * t268;
    (t10337, t10341, t10343, t10344, t10346)
}
