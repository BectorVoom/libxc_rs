//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta139 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk927;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk928;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk929;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk930;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk931;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk932;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk933;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk934;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta139(t3316: f64, t342: f64, t3303: f64, t357: f64, t3300: f64, t3259: f64, t380: f64, t1024: f64, t1083: f64, t1087: f64, t1090: f64, t1093: f64, t3043: f64, t3204: f64, t3223: f64, t3278: f64, t3283: f64, t3287: f64, t3288: f64, t3292: f64, t3295: f64, t3299: f64, t3305: f64, t3309: f64, t3313: f64, t381: f64, t989: f64, t1079: f64, t1000: f64, t1073: f64, t1076: f64, t1097: f64, t3047: f64, t3052: f64, t3058: f64, t3060: f64, t3063: f64, t3067: f64, t3076: f64, t3261: f64, t3264: f64, t3271: f64, t386: f64, t995: f64, t1100: f64, t389: f64, t1102: f64, t198: f64, t2868: f64, t2871: f64, t2878: f64, t2921: f64, t2929: f64, t3019: f64, t3021: f64, t3024: f64, t3028: f64, t3032: f64, t3036: f64, t336: f64, t30: f64, t265: f64, t393: f64, t2838: f64, t1106: f64, t2257: f64, t2258: f64, t395: f64, t45: f64, t605: f64, t606: f64, t895: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3317 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk927(t3316, t342);
        let t3318 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk928(t3303, t357);
        let (t3319, t3322, t3325) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk929(t3300, t3318, t3259, t380, t1024, t1083, t1087, t1090, t1093, t3043, t3204, t3223, t3278, t3283, t3287, t3288, t3292, t3295, t3299, t3305, t3309, t3313, t3317, t342, t381, t989);
        let t3326 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk930(t1079, t3325);
        let t3329 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk931(t1000, t1073, t1076, t1097, t3043, t3047, t3052, t3058, t3060, t3063, t3067, t3076, t3261, t3264, t3271, t3326, t342, t386, t989, t995);
        let (t3333, t3335, t3336) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk932(t1100, t389);
        let t3339 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk933(t1102, t198, t2868, t2871, t2878, t2921, t2929, t3019, t3021, t3024, t3028, t3032, t3036, t3329, t3333, t3336, t336);
        let (t3340, t3347) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk934(t30, t265, t393, t2838, t3339, t1106, t2257, t2258, t395, t45, t605, t606, t895, dens_threshold, rho0, zeta_threshold);
    (t3317, t3318, t3319, t3322, t3325, t3326, t3329, t3333, t3335, t3336, t3340, t3347)
}
