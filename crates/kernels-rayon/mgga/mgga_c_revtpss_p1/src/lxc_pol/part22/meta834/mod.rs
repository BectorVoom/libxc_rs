//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta834 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2958;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2959;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta834(t13769: f64, t808: f64, t9736: f64, t13952: f64, t2689: f64, t13784: f64, t543: f64, t46825: f64, t9793: f64, t1353: f64, t1883: f64, t13848: f64, t9810: f64, t9816: f64, t9818: f64, t1408: f64, t241: f64, t820: f64, t2482: f64, t814: f64, t9991: f64, t13805: f64, t13847: f64, t46917: f64, t5706: f64, t47201: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48690, t48692, t48694, t48696, t48698, t48700, t48709) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2958(t13769, t808, t9736, t13952, t2689, t13784, t543, t46825, t9793, t1353, t1883, t13848, t9810, t9816, t9818);
        let (t48712, t48731, t48734, t48756, t48759) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2959(t1408, t241, t820, t2482, t814, t9991, t13805, t13847, t13848, t46917, t5706, t47201);
    (t48690, t48692, t48694, t48696, t48698, t48700, t48709, t48712, t48731, t48734, t48756, t48759)
}
