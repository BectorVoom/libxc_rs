//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta149 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk689;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk690;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk691;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk692;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk693;
use chunk5::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk694;
use chunk6::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk695;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta149(t1079: f64, t3325: f64, t1000: f64, t1073: f64, t1076: f64, t1097: f64, t3043: f64, t3047: f64, t3052: f64, t3058: f64, t3060: f64, t3063: f64, t3067: f64, t3076: f64, t3261: f64, t3264: f64, t3271: f64, t342: f64, t386: f64, t989: f64, t995: f64, t1100: f64, t389: f64, t1102: f64, t198: f64, t2868: f64, t2871: f64, t2878: f64, t2921: f64, t2929: f64, t3019: f64, t3021: f64, t3024: f64, t3028: f64, t3032: f64, t3036: f64, t336: f64, t30: f64, t265: f64, t393: f64, t2838: f64, t1106: f64, t2257: f64, t2258: f64, t395: f64, t45: f64, t605: f64, t606: f64, t895: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t1941: f64, t268: f64, t404: f64, t1123: f64, t689: f64, t1263: f64, t159: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3326, t3329) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk689(t1079, t3325, t1000, t1073, t1076, t1097, t3043, t3047, t3052, t3058, t3060, t3063, t3067, t3076, t3261, t3264, t3271, t342, t386, t989, t995);
        let (t3333, t3335, t3336, t3339) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk690(t1100, t389, t1102, t198, t2868, t2871, t2878, t2921, t2929, t3019, t3021, t3024, t3028, t3032, t3036, t3329, t336);
        let (t3340, t3347) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk691(t30, t265, t393, t2838, t3339, t1106, t2257, t2258, t395, t45, t605, t606, t895, dens_threshold, rho0, zeta_threshold);
        let t3351 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk692(t2257);
        let t3356 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk693(t1941, t268, t404);
        let (t3357, t3358) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk694(t3356, t1123, t689);
        let t3360 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk695(t1263, t159);
    (t3326, t3329, t3333, t3335, t3336, t3340, t3347, t3351, t3356, t3357, t3358, t3360)
}
