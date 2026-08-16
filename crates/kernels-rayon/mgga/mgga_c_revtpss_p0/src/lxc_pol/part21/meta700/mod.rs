//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta700 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2522;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2523;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta700(t10285: f64, t596: f64, t10293: f64, t592: f64, t25: f64, t40649: f64, t10296: f64, t602: f64, t2240: f64, t2246: f64, t10308: f64, t599: f64, t90: f64, t29: f64, t11149: f64, t78: f64, t12267: f64, t81: f64, t46: f64, t47: f64, t58: f64, t59: f64, t2681: f64, t64: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t45945, t45949, t45952, t45955, t45958, t45963) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2522(t10285, t596, t10293, t592, t25, t40649, t10296, t602, t2240, t2246, t10308, t599);
        let (t45972, t46001, t46014, t46065, t46074, t46089) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2523(t90, t29, t11149, t78, t12267, t81, t46, t47, t58, t59, t2681, t64);
    (t45945, t45949, t45952, t45955, t45958, t45963, t45972, t46001, t46014, t46065, t46074, t46089)
}
