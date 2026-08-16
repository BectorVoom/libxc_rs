//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta438 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1393;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1394;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta438(t1337: f64, t40101: f64, t1340: f64, t40097: f64, t39816: f64, t1333: f64, t9855: f64, t19: f64, t2237: f64, t521: f64, t9342: f64, t14: f64, t27: f64, t583: f64, t596: f64, t525: f64, t9603: f64, t527: f64, t9615: f64, t40165: f64, t268: f64, t520: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46992, t46996, t46998, t47000, t47003, t47014, t47016) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1393(t1337, t40101, t1340, t40097, t39816, t1333, t9855, t19, t2237, t521, t9342, t14, t27);
        let (t47017, t47020, t47025, t47040, t47059, t47065) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1394(t47016, t521, t583, t596, t525, t9603, t527, t9615, t1340, t40165, t268, t520);
    (t46992, t46996, t46998, t47000, t47003, t47014, t47017, t47020, t47025, t47040, t47059, t47065)
}
