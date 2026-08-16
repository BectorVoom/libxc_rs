//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta696 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2444;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2445;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta696(t1390: f64, t1399: f64, t46856: f64, t685: f64, t3952: f64, t9784: f64, t281: f64, t39644: f64, t40650: f64, t547: f64, t550: f64, t40688: f64, t46786: f64, t1386: f64, t2682: f64, t820: f64, t2735: f64, t5744: f64, t4086: f64, t9801: f64, t9846: f64, t1320: f64, t9545: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46859, t46879, t46885, t46888) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2444(t1390, t1399, t46856, t685, t3952, t9784, t281, t39644, t40650, t547, t550, t40688);
        let (t46889, t46917, t46929, t46946, t46947, t46963) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2445(t46786, t46888, t1386, t2682, t820, t2735, t5744, t4086, t9801, t9846, t1320, t9545);
    (t46859, t46879, t46885, t46888, t46889, t46917, t46929, t46946, t46947, t46963)
}
