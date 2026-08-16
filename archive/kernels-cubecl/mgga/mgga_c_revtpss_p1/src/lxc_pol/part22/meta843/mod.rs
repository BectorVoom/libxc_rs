//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta843 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2976;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2977;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta843<F: Float>(t13792: F, t48863: F, t49137: F, t13920: F, t2661: F, t3992: F, t543: F, t550: F, t1398: F, t5658: F, t10073: F, t14124: F, t5760: F, t9292: F, t10069: F, t14207: F, t40921: F, t5737: F, t225: F, t2453: F, t136: F, t137: F, t14140: F, t2438: F, t4003: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t49139, t49144, t49146, t49167) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2976::<F>(t13792, t48863, t49137, t13920, t2661, t3992, t543, t550, t1398, t5658, t10073, t14124);
        let (t49172, t49176, t49178, t49180, t49186) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2977::<F>(t5760, t9292, t10069, t14207, t40921, t5737, t225, t2453, t136, t137, t1398, t14140, t2438, t4003);
    (t49139, t49144, t49146, t49167, t49172, t49176, t49178, t49180, t49186)
}
