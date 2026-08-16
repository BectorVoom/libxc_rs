//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta37 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk276;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk277;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk278;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk279;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk280;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk281;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk282;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta37<F: Float>(t185: F, t744: F, t123: F, t173: F, t186: F, t676: F, t679: F, t704: F, t724: F, t731: F, t739: F, t162: F, t158: F, t716: F, t187: F, t192: F, t72: F, t675: F, t685: F, t177: F, t738: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t745 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk276::<F>(t185);
        let t746 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk277::<F>(t744, t745);
        let t749 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk278::<F>(t123, t173, t186, t676, t679, t704, t724, t731, t739, t746);
        let t750 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk279::<F>(t162, t749);
        let (t751, t752, t754, t755, t757) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk280::<F>(t158, t750, t162, t716, t187, t192, t72, t186, t675, t685);
        let (t759, t760) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk281::<F>(t755, t757, t177, t192);
        let t762 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk282::<F>(t738, t744, t745);
    (t745, t746, t749, t750, t751, t752, t754, t755, t757, t759, t760, t762)
}
