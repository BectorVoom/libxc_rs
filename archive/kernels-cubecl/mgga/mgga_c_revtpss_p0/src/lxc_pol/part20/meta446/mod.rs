//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta446 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1704;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1705;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta446<F: Float>(t1358: F, t588: F, t9647: F, t4086: F, t9646: F, t1399: F, t22: F, t555: F, t9890: F, t10040: F, t2435: F, t10039: F, t2439: F, t2777: F, t4003: F, t1419: F, t4056: F, t1429: F, t39501: F, t9994: F, t1398: F, t9840: F, t2482: F, t4114: F, t686: F, t72: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t46388, t46392, t46394, t46398, t46401) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1704::<F>(t1358, t588, t9647, t4086, t9646, t1399, t22, t555, t9890, t10040, t2435, t10039, t2439, t2777);
        let (t46403, t46407, t46412, t46416, t46422, t46424) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1705::<F>(t4003, t9890, t1419, t4056, t1429, t39501, t9994, t1398, t9840, t2482, t4114, t686, t72);
    (t46388, t46392, t46394, t46398, t46401, t46403, t46407, t46412, t46416, t46422, t46424)
}
