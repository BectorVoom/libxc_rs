//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta720 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2559;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta720(t4038: f64, t9318: f64, t1337: f64, t40101: f64, t9323: f64, t1340: f64, t40097: f64, t39816: f64, t19: f64, t2237: f64, t521: f64, t1331: f64, t9342: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t46989, t46992, t46993, t46996, t46998, t47003, t47005) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2559(t4038, t9318, t1337, t40101, t9323, t1340, t40097, t39816, t19, t2237, t521, t1331, t9342);
    (t46989, t46992, t46993, t46996, t46998, t47003, t47005)
}
