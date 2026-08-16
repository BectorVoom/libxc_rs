//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta432 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1382;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta432(t46072: f64, t59: f64, t2681: f64, t64: f64, t112: f64, t10207: f64, t111: f64, t36227: f64, t36415: f64, t39454: f64, t521: f64, t1333: f64, t9413: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46074, t46090, t46143, t46157, t46196, t46212, t46292, t46297) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1382(t46072, t59, t2681, t64, t112, t10207, t111, t36227, t36415, t39454, t521, t1333, t9413);
    (t46074, t46090, t46143, t46157, t46196, t46212, t46292, t46297)
}
