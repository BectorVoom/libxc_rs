//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 920/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk920(t310: f64, t7506: f64, t7514: f64, t7518: f64, t22: f64, t30174: f64, t420: f64, t56: f64, t7507: f64, t7513: f64, t174: f64, t30779: f64, t7322: f64) -> (f64, f64, f64, f64, f64) {
    let t31388 = t310 * t7506;
    let t31389 = t31388 * t7514;
    let t31391 = t31388 * t7518;
    let t31402 = 1.0_f64 / t22 / t30174;
    let t31404 = t31402 * t56 * t420;
    let t31406 = t7507 * t31404 * t7513;
    let t31419 = t7322 * t30779 * t174;
    (t31389, t31391, t31404, t31406, t31419)
}
