//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta725 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2490;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2491;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta725<F: Float>(t46888: F, t48908: F, t1413: F, t46835: F, t48694: F, t13775: F, t9793: F, t9794: F, t5690: F, t9741: F, t2659: F, t5744: F, t816: F, t10073: F, t14124: F, t5760: F, t9292: F, t10069: F, t14207: F, t40921: F, t5737: F, t225: F, t2453: F, t136: F, t137: F, t1398: F, t14140: F, t2438: F, t4003: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t49105, t49122, t49125, t49127, t49137) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2490::<F>(t46888, t48908, t1413, t46835, t48694, t13775, t9793, t9794, t5690, t9741, t2659, t5744, t816);
        let (t49167, t49172, t49177, t49178, t49180, t49186) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2491::<F>(t10073, t14124, t5760, t9292, t10069, t14207, t40921, t5737, t225, t2453, t136, t137, t1398, t14140, t2438, t4003);
    (t49105, t49122, t49125, t49127, t49137, t49167, t49172, t49177, t49178, t49180, t49186)
}
