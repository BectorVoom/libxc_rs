//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta802 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2903;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta802(t1320: f64, t9428: f64, t1331: f64, t9410: f64, t9413: f64, t9554: f64, t1340: f64, t40086: f64, t4038: f64, t9318: f64, t1337: f64, t40101: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t46973, t46975, t46977, t46983, t46988, t46989, t46992) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2903(t1320, t9428, t1331, t9410, t9413, t9554, t1340, t40086, t4038, t9318, t1337, t40101);
    (t46973, t46975, t46977, t46983, t46988, t46989, t46992)
}
