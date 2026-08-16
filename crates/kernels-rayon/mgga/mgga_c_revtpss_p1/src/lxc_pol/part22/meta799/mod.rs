//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta799 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2899;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2900;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta799(t3952: f64, t9784: f64, t281: f64, t39644: f64, t40650: f64, t547: f64, t550: f64, t2689: f64, t9715: f64, t40688: f64, t46786: f64, t9704: f64, t1386: f64, t2682: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t46879, t46885, t46886, t46888, t46889, t46895) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2899(t3952, t9784, t281, t39644, t40650, t547, t550, t2689, t9715, t40688, t46786, t9704);
        let t46917 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2900(t1386, t2682, t820);
    (t46879, t46885, t46886, t46888, t46889, t46895, t46917)
}
