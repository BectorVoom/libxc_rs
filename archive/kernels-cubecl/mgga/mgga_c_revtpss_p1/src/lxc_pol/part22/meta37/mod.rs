//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta37 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk274;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk275;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk276;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk277;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk278;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk279;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta37<F: Float>(t177: F, t738: F, t687: F, t689: F, t693: F, t698: F, t185: F, t123: F, t173: F, t186: F, t676: F, t679: F, t704: F, t724: F, t731: F, t162: F, t158: F, t716: F, t187: F, t192: F, t72: F, t675: F, t685: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t739, t744) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk274::<F>(t177, t738, t687, t689, t693, t698);
        let t745 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk275::<F>(t185);
        let t746 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk276::<F>(t744, t745);
        let t749 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk277::<F>(t123, t173, t186, t676, t679, t704, t724, t731, t739, t746);
        let t750 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk278::<F>(t162, t749);
        let (t751, t752, t754, t755, t757) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk279::<F>(t158, t750, t162, t716, t187, t192, t72, t186, t675, t685);
    (t739, t744, t745, t746, t749, t750, t751, t752, t754, t755, t757)
}
