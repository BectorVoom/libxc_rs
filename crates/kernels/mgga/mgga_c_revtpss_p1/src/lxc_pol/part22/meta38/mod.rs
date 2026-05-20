//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta38 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk280;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk281;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk282;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk283;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk284;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk285;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk286;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk287;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta38<F: Float>(t755: F, t757: F, t177: F, t192: F, t738: F, t744: F, t745: F, t206: F, t262: F, t78: F, t45: F, t606: F, t81: F, zeta_threshold: F, t57: F, t212: F, t251: F, t225: F, t257: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t759, t760) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk280::<F>(t755, t757, t177, t192);
        let t762 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk281::<F>(t738, t744, t745);
        let (t764, t765) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk282::<F>(t760, t762, t206, t262);
        let t766 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk283::<F>(t78);
        let (t769, t770) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk284::<F>(t45, t606, t766, t81, zeta_threshold);
        let t775 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk285::<F>(t57, t606, t770, t769, zeta_threshold);
        let t779 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk286::<F>(t212, t251);
        let t780 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk287::<F>(t225, t257);
    (t759, t760, t762, t764, t765, t766, t770, t775, t779, t780)
}
