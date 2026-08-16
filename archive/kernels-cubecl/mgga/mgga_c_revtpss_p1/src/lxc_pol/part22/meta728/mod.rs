//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta728 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2784;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2785;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta728<F: Float>(t2394: F, t2475: F, t10069: F, t10929: F, t138: F, t785: F, t9302: F, t2786: F, t10073: F, t10920: F, t10871: F, t2645: F, t234: F, t39545: F, t685: F, t875: F, t2760: F, t2783: F, t786: F, t2778: F, t39515: F, t39501: F, t871: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t40236, t40267, t40270, t40271, t40273, t40284) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2784::<F>(t2394, t2475, t10069, t10929, t138, t785, t9302, t2786, t10073, t10920, t10871, t2645);
        let (t40294, t40297, t40303, t40314, t40316) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2785::<F>(t234, t39545, t685, t875, t2760, t2783, t786, t10069, t10920, t2778, t39515, t39501, t871);
    (t40236, t40267, t40270, t40271, t40273, t40284, t40294, t40297, t40303, t40314, t40316)
}
