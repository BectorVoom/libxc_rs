//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1423/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1423(t224: f64, t36263: f64, t36300: f64, t36458: f64, t37309: f64, t12050: f64, t12045: f64, t12063: f64, t12041: f64, t12296: f64, t12053: f64, t12043: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37312 = t224 * (t36263 + t36300 + t36458 + t37309);
    let t37317 = 4.0_f64 * t12050;
    let t37318 = 2.0_f64 * t12045;
    let t37319 = 4.0_f64 * t12063;
    let t37320 = 2.0_f64 * t12041;
    let t37322 = 2.0_f64 * t12296;
    let t37323 = 12.0_f64 * t12053;
    let t37324 = 4.0_f64 * t12043;
    (t37312, t37317, t37318, t37319, t37320, t37322, t37323, t37324)
}
