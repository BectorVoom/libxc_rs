//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta791 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2883;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2884;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta791<F: Float>(t10039: F, t2439: F, t2777: F, t1429: F, t39501: F, t4056: F, t9994: F, t10014: F, t10136: F, t215: F, t3923: F, t268: F, t4101: F, t543: F, t10023: F, t4003: F, t1419: F, t5744: F, t786: F, t1398: F, t793: F, t10073: F, t10084: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t46401, t46412, t46416, t46443, t46445, t46448) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2883::<F>(t10039, t2439, t2777, t1429, t39501, t4056, t9994, t10014, t10136, t215, t3923, t268, t4101, t543);
        let (t46452, t46456, t46457, t46463, t46465) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2884::<F>(t10023, t268, t4003, t46445, t1419, t5744, t786, t1398, t4101, t543, t793, t10073, t10084);
    (t46401, t46412, t46416, t46443, t46448, t46452, t46456, t46457, t46463, t46465)
}
