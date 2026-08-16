//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta36 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk224;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk225;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk226;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta36<F: Float>(t157: F, t705: F, t45: F, t57: F, t190: F, t606: F, t78: F, t81: F, t150: F, t169: F, t164: F, t687: F, t689: F, t693: F, t698: F, zeta_threshold: F, t172: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t706 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk224::<F>(t157, t705);
        let (t707, t709, t716, t717, t718, t722, t723, t724, t729) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk225::<F>(t45, t57, t190, t606, t706, t78, t81, t150, t169, t164, t687, t689, t693, t698, zeta_threshold);
        let t730 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk226::<F>(t172);
    (t706, t707, t709, t716, t717, t718, t722, t723, t724, t729, t730)
}
