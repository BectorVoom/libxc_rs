//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta149 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk689;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk690;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk691;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk692;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk693;
use chunk5::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk694;
use chunk6::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk695;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta149<F: Float>(t1079: F, t3325: F, t1000: F, t1073: F, t1076: F, t1097: F, t3043: F, t3047: F, t3052: F, t3058: F, t3060: F, t3063: F, t3067: F, t3076: F, t3261: F, t3264: F, t3271: F, t342: F, t386: F, t989: F, t995: F, t1100: F, t389: F, t1102: F, t198: F, t2868: F, t2871: F, t2878: F, t2921: F, t2929: F, t3019: F, t3021: F, t3024: F, t3028: F, t3032: F, t3036: F, t336: F, t30: F, t265: F, t393: F, t2838: F, t1106: F, t2257: F, t2258: F, t395: F, t45: F, t605: F, t606: F, t895: F, dens_threshold: F, rho0: F, zeta_threshold: F, t1941: F, t268: F, t404: F, t1123: F, t689: F, t1263: F, t159: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3326, t3329) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk689::<F>(t1079, t3325, t1000, t1073, t1076, t1097, t3043, t3047, t3052, t3058, t3060, t3063, t3067, t3076, t3261, t3264, t3271, t342, t386, t989, t995);
        let (t3333, t3335, t3336, t3339) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk690::<F>(t1100, t389, t1102, t198, t2868, t2871, t2878, t2921, t2929, t3019, t3021, t3024, t3028, t3032, t3036, t3329, t336);
        let (t3340, t3347) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk691::<F>(t30, t265, t393, t2838, t3339, t1106, t2257, t2258, t395, t45, t605, t606, t895, dens_threshold, rho0, zeta_threshold);
        let t3351 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk692::<F>(t2257);
        let t3356 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk693::<F>(t1941, t268, t404);
        let (t3357, t3358) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk694::<F>(t3356, t1123, t689);
        let t3360 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk695::<F>(t1263, t159);
    (t3326, t3329, t3333, t3335, t3336, t3340, t3347, t3351, t3356, t3357, t3358, t3360)
}
