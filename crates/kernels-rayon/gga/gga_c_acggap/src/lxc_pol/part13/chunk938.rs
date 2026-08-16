//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 938/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk938(t1072: f64, t429: f64, t7507: f64, t7512: f64, t310: f64, t7506: f64, t7514: f64, t7518: f64, t22: f64, t30174: f64, t420: f64, t56: f64) -> (f64, f64, f64, f64) {
    let t31386 = t7507 * t7512 * t429 * t1072;
    let t31388 = t310 * t7506;
    let t31389 = t31388 * t7514;
    let t31390 = 0.12862205435420921092e-2_f64 * t31389;
    let t31391 = t31388 * t7518;
    let t31392 = 0.1886885537376249124e-2_f64 * t31391;
    let t31402 = 1.0_f64 / t22 / t30174;
    let t31404 = t31402 * t56 * t420;
    (t31386, t31390, t31392, t31404)
}
