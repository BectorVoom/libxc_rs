//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta701 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2524;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta701(t46089: f64, t10414: f64, t116: f64, t112: f64, t10199: f64, t666: f64, t2289: f64, t2341: f64, t2367: f64, t10210: f64, t625: f64, t10214: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46090, t46126, t46143, t46144, t46146, t46148, t46150, t46152) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2524(t46089, t10414, t116, t112, t10199, t666, t2289, t2341, t2367, t10210, t625, t10214);
    (t46090, t46126, t46143, t46144, t46146, t46148, t46150, t46152)
}
