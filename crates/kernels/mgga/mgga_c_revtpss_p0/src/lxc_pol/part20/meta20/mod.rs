//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta20 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk160;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk161;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk162;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk163;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk164;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk165;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk166;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta20<F: Float>(t378: F, t380: F, t342: F, t379: F, t198: F, t293: F, t328: F, t330: F, t336: F, t265: F, t30: F, t33: F, t45: F, t57: F, dens_threshold: F, rho0: F, rho1: F, zeta_threshold: F, t268: F, t269: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t381 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk160::<F>(t378, t380);
        let (t384, t385) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk161::<F>(t342, t381);
        let t386 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk162::<F>(t379, t385);
        let (t389, t395, t393) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk163::<F>(t342, t386, t198, t293, t328, t330, t336, t265);
        let (t398, t403, t404) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk164::<F>(t30, t33, t265, t395, t45, t57, dens_threshold, rho0, rho1, zeta_threshold);
        let t406 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk165::<F>(t268, t269, t404);
        let t408 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk166::<F>(t406);
    (t381, t384, t385, t386, t389, t395, t393, t398, t403, t404, t406, t408)
}
