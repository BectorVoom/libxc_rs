//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta437 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1391;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1392;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta437(t10111: f64, t1386: f64, t9720: f64, t281: f64, t39644: f64, t40650: f64, t547: f64, t550: f64, t40688: f64, t2682: f64, t820: f64, t2735: f64, t5744: f64, t4086: f64, t9801: f64, t1320: f64, t9545: f64, t40082: f64, t512: f64, t520: f64, t1333: f64, t9410: f64, t3853: f64, t3863: f64, t1340: f64, t40086: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46856, t46885, t46888, t46917, t46929) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1391(t10111, t1386, t9720, t281, t39644, t40650, t547, t550, t40688, t2682, t820, t2735, t5744);
        let (t46946, t46963, t46970, t46972, t46980, t46988) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1392(t4086, t9801, t1320, t9545, t40082, t512, t520, t1333, t9410, t3853, t3863, t1340, t40086);
    (t46856, t46885, t46888, t46917, t46929, t46946, t46963, t46970, t46972, t46980, t46988)
}
