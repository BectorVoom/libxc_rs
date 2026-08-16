//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta163 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1089;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1090;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1091;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1092;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1093;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1094;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1095;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1096;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1097;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1098;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta163(t3860: f64, t521: f64, t583: f64, t588: f64, t1320: f64, t1333: f64, t198: f64, t2522: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t3827: f64, t3828: f64, t3829: f64, t3852: f64, t3854: f64, t3856: f64, t3859: f64, t123: f64, t520: f64, t2630: f64, t1337: f64, t2619: f64, t514: f64, t30: f64, t1344: f64, t2257: f64, t3834: f64, t517: f64, zeta_threshold: f64, t33: f64, t1348: f64, t3351: f64, t3842: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3862 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1089(t3860, t521);
        let t3863 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1090(t583, t588);
        let (t3865, t3867) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1091(t3863, t521, t1320, t1333);
        let t3868 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1092(t198, t2522, t2562, t2569, t2579, t2587, t3827, t3828, t3829, t3852, t3854, t3856, t3859, t3862, t3865, t3867);
        let t3869 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1093(t123, t520);
        let t3871 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1094(t2630, t3869);
        let t3873 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1095(t1337, t2619);
        let t3874 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1096(t514);
        let (t3880, t3881) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1097(t30, t1344, t2257, t3834, t3874, t517, zeta_threshold);
        let t3889 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1098(t33, t1348, t3351, t3842, t3881, t3880, zeta_threshold);
    (t3862, t3863, t3865, t3867, t3868, t3869, t3871, t3873, t3874, t3881, t3889)
}
