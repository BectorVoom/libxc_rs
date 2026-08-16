//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta21 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk141;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk142;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk143;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk144;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk145;
use chunk5::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk146;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta21<F: Float>(t342: F, t386: F, t198: F, t293: F, t328: F, t330: F, t336: F, t265: F, t30: F, t33: F, t45: F, t57: F, dens_threshold: F, rho0: F, rho1: F, zeta_threshold: F, t268: F, t269: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t389, t395, t393) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk141::<F>(t342, t386, t198, t293, t328, t330, t336, t265);
        let (t398, t403, t404) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk142::<F>(t30, t33, t265, t395, t45, t57, dens_threshold, rho0, rho1, zeta_threshold);
        let t406 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk143::<F>(t268, t269, t404);
        let t408 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk144::<F>(t406);
        let t409 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk145::<F>(t406);
        let (t412, t414) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk146::<F>(t406, t404);
    (t389, t395, t393, t398, t403, t404, t406, t408, t409, t412, t414)
}
