//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta21 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk153;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk154;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk155;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk156;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk157;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk158;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk159;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta21<F: Float>(t379: F, t385: F, t342: F, t198: F, t293: F, t328: F, t330: F, t336: F, t265: F, t30: F, t33: F, t45: F, t57: F, dens_threshold: F, rho0: F, rho1: F, zeta_threshold: F, t268: F, t269: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t386 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk153::<F>(t379, t385);
        let (t389, t395, t393) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk154::<F>(t342, t386, t198, t293, t328, t330, t336, t265);
        let (t398, t403, t404) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk155::<F>(t30, t33, t265, t395, t45, t57, dens_threshold, rho0, rho1, zeta_threshold);
        let t406 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk156::<F>(t268, t269, t404);
        let t408 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk157::<F>(t406);
        let t409 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk158::<F>(t406);
        let (t412, t414) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk159::<F>(t406, t404);
    (t386, t389, t395, t393, t398, t403, t404, t406, t408, t409, t412, t414)
}
