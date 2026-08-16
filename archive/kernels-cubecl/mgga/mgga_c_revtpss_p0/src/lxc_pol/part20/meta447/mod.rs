//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta447 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1706;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1707;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1708;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1709;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta447<F: Float>(t1398: F, t4056: F, t543: F, t1437: F, t2482: F, t686: F, t72: F, t10014: F, t10136: F, t215: F, t3923: F, t268: F, t4101: F, t10023: F, t4003: F, t10119: F, t1419: F, t5744: F, t786: F, t10026: F, t793: F, t10073: F, t10084: F, t10059: F, t10130: F, t3924: F, t4057: F, t5745: F, t5755: F, t820: F, t9840: F, t555: F, t9898: F, t14192: F, t2782: F, t9994: F, t544: F, t9989: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t46432, t46433, t46435, t46443, t46445, t46448) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1706::<F>(t1398, t4056, t543, t1437, t2482, t686, t72, t10014, t10136, t215, t3923, t268, t4101);
        let (t46452, t46454, t46458, t46463) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1707::<F>(t10023, t268, t4003, t46445, t10014, t10119, t1419, t5744, t786, t10026, t1398, t4101, t543, t793);
        let t46467 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1708::<F>(t10073, t10084, t10059, t10130, t3924, t4057, t46435, t46443, t46448, t46452, t46454, t46458, t46463, t5745, t5755, t820, t9840);
        let (t46469, t46472, t46475, t46476, t46477, t46479, t46483, t46490) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1709::<F>(t555, t9898, t14192, t2782, t9994, t544, t9989, t3923, t4003, t215, t268, t4056, t4101, t543);
    (t46432, t46433, t46467, t46469, t46472, t46475, t46476, t46477, t46479, t46483, t46490)
}
