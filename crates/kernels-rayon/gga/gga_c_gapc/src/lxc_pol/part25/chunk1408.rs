//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1408/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1408(t35358: f64, t35361: f64, t12191: f64, t883: f64, t972: f64, t12050: f64, t12045: f64, t12053: f64, t12043: f64, t12047: f64, t12046: f64, t12059: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37294 = 0.14759453667534722223e-5_f64 * t35358;
    let t37295 = 0.20220636637604418766e-5_f64 * t35361;
    let t37306 = t12191 * t883;
    let t37308 = 2.0_f64 * t37306 * t972;
    let t37317 = 4.0_f64 * t12050;
    let t37318 = 2.0_f64 * t12045;
    let t37323 = 12.0_f64 * t12053;
    let t37324 = 4.0_f64 * t12043;
    let t37325 = 4.0_f64 * t12047;
    let t37327 = 2.0_f64 * t12046;
    let t37328 = 4.0_f64 * t12059;
    (t37294, t37295, t37308, t37317, t37318, t37323, t37324, t37325, t37327, t37328)
}
