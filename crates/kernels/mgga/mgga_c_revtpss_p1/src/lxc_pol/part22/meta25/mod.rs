//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta25 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk194;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk195;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk196;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk197;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk198;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk199;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk200;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta25<F: Float>(t488: F, t494: F, t460: F, t198: F, t336: F, t424: F, t452: F, t454: F, t265: F, t33: F, t57: F, t398: F, dens_threshold: F, rho1: F, zeta_threshold: F, t117: F, t93: F, t19: F, t22: F, t30: F, t153: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t495 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk194::<F>(t488, t494);
        let (t498, t504, t502) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk195::<F>(t460, t495, t198, t336, t424, t452, t454, t265);
        let t508 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk196::<F>(t33, t265, t504, t57, t398, dens_threshold, rho1, zeta_threshold);
        let t511 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk197::<F>(t117, t93);
        let t512 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk198::<F>(t19, t22);
        let t513 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk199::<F>(t30);
        let (t514, t515, t516) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk200::<F>(t30, t513, t153, t33, zeta_threshold);
    (t495, t498, t504, t502, t508, t511, t512, t513, t514, t515, t516)
}
