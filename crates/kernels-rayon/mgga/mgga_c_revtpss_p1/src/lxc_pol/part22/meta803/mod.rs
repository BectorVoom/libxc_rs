//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta803 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2904;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2905;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta803(t4038: f64, t9323: f64, t1340: f64, t40097: f64, t39816: f64, t1333: f64, t9855: f64, t19: f64, t2237: f64, t521: f64, t1331: f64, t9342: f64, t2619: f64, t9563: f64, t3825: f64, t9586: f64, t14: f64, t27: f64, t525: f64, t9603: f64, t527: f64, t9615: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46993, t46996, t46998, t46999, t47003, t47005) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2904(t4038, t9323, t1340, t40097, t39816, t1333, t9855, t19, t2237, t521, t1331, t9342);
        let (t47007, t47009, t47011, t47016, t47025, t47040) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2905(t1331, t9855, t2619, t9563, t3825, t9586, t14, t27, t521, t525, t9603, t527, t9615);
    (t46993, t46996, t46998, t46999, t47003, t47005, t47007, t47009, t47011, t47016, t47025, t47040)
}
