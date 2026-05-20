//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta834 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2958;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2959;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta834<F: Float>(t13769: F, t808: F, t9736: F, t13952: F, t2689: F, t13784: F, t543: F, t46825: F, t9793: F, t1353: F, t1883: F, t13848: F, t9810: F, t9816: F, t9818: F, t1408: F, t241: F, t820: F, t2482: F, t814: F, t9991: F, t13805: F, t13847: F, t46917: F, t5706: F, t47201: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t48690, t48692, t48694, t48696, t48698, t48700, t48709) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2958::<F>(t13769, t808, t9736, t13952, t2689, t13784, t543, t46825, t9793, t1353, t1883, t13848, t9810, t9816, t9818);
        let (t48712, t48731, t48734, t48756, t48759) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2959::<F>(t1408, t241, t820, t2482, t814, t9991, t13805, t13847, t13848, t46917, t5706, t47201);
    (t48690, t48692, t48694, t48696, t48698, t48700, t48709, t48712, t48731, t48734, t48756, t48759)
}
