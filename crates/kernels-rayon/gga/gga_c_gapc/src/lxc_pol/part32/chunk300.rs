//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 300/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk300(t1153: f64, t131: f64, t1: f64, t348: f64, t399: f64, t350: f64, t95: f64, t1150: f64, t19: f64) -> (f64, f64, f64, f64) {
    let t1154 = t1153 * t131;
    let t1158 = t348 * t399 * t1;
    let t1161 = t350 * t95;
    let t1164 = t1150 * t19;
    let t1165 = t1164 * t1154;
    (t1154, t1158, t1161, t1165)
}
