//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta748 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2623;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2624;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta748<F: Float>(t47060: F, t13581: F, t72: F, t757: F, t47073: F, t5635: F, t9586: F, t5571: F, t9425: F, t47078: F, t9318: F, t1857: F, t9342: F, t39807: F, t39813: F, t47059: F, t47063: F, t47067: F, t47070: F, t47072: F, t47076: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t48275, t48278, t48279, t48281, t48283, t48284, t48286, t48287) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2623::<F>(t47060, t13581, t72, t757, t47073, t5635, t9586, t5571, t9425, t47078, t9318, t1857, t9342);
        let (t48288, t48289) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2624::<F>(t48287, t39807, t39813, t47059, t47063, t47067, t47070, t47072, t47076, t48275, t48278, t48279, t48281, t48283, t48284, t48286);
    (t48275, t48278, t48279, t48281, t48283, t48284, t48286, t48288, t48289)
}
