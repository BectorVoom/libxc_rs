//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta789 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2879;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2880;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta789<F: Float>(t36227: F, t36415: F, t3860: F, t4029: F, t3857: F, t4038: F, t9387: F, t2608: F, t3850: F, t512: F, t1333: F, t9413: F, t3853: F, t30: F, t513: F, t9603: F, t33: F, t516: F, t9615: F, t10153: F, t2435: F, t2439: F, t3895: F, t4078: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t46196, t46212, t46279, t46281, t46286, t46289, t46297) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2879::<F>(t36227, t36415, t3860, t4029, t3857, t4038, t9387, t2608, t3850, t512, t1333, t9413);
        let (t46302, t46310, t46328, t46353, t46356) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2880::<F>(t3853, t3860, t30, t513, t9603, t33, t516, t9615, t10153, t2435, t2439, t3895, t4078);
    (t46196, t46212, t46279, t46281, t46286, t46289, t46297, t46302, t46310, t46328, t46353, t46356)
}
