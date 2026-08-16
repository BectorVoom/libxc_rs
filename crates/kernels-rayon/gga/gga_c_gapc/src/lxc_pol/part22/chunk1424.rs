//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1424/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1424(t12050: f64, t12045: f64, t12063: f64, t12041: f64, t12296: f64, t12053: f64, t12043: f64, t12047: f64, t12061: f64, t12046: f64, t12059: f64, t12039: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37317 = 4.0_f64 * t12050;
    let t37318 = 2.0_f64 * t12045;
    let t37319 = 4.0_f64 * t12063;
    let t37320 = 2.0_f64 * t12041;
    let t37322 = 2.0_f64 * t12296;
    let t37323 = 12.0_f64 * t12053;
    let t37324 = 4.0_f64 * t12043;
    let t37325 = 4.0_f64 * t12047;
    let t37326 = 2.0_f64 * t12061;
    let t37327 = 2.0_f64 * t12046;
    let t37328 = 4.0_f64 * t12059;
    let t37329 = 2.0_f64 * t12039;
    (t37317, t37318, t37319, t37320, t37322, t37323, t37324, t37325, t37326, t37327, t37328, t37329)
}
