//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta691 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2434;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2435;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta691<F: Float>(t1358: F, t588: F, t9647: F, t4086: F, t9646: F, t1399: F, t22: F, t555: F, t1429: F, t39501: F, t1419: F, t5744: F, t786: F, t1398: F, t268: F, t4101: F, t543: F, t793: F, t544: F, t9989: F, t4003: F, t10013: F, t2453: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t46388, t46389, t46392, t46412, t46456) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2434::<F>(t1358, t588, t9647, t4086, t9646, t1399, t22, t555, t1429, t39501, t1419, t5744);
        let (t46457, t46463, t46475, t46478, t46495) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2435::<F>(t46456, t786, t1398, t268, t4101, t543, t793, t544, t9989, t4003, t10013, t2453);
    (t46388, t46389, t46392, t46412, t46457, t46463, t46475, t46478, t46495)
}
