//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta36 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk269;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk270;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk271;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk272;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk273;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta36<F: Float>(t45: F, t57: F, t706: F, t707: F, t606: F, t78: F, t81: F, zeta_threshold: F, t150: F, t190: F, t169: F, t164: F, t687: F, t689: F, t693: F, t698: F, t172: F, t182: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t709, t716) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk269::<F>(t45, t57, t706, t707, t606, t78, t81, zeta_threshold);
        let (t717, t718, t722, t723, t724, t729) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk270::<F>(t150, t716, t190, t169, t164, t687, t689, t693, t698);
        let t730 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk271::<F>(t172);
        let t731 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk272::<F>(t729, t730);
        let (t737, t738) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk273::<F>(t182);
    (t709, t716, t717, t718, t722, t723, t724, t729, t730, t731, t737, t738)
}
