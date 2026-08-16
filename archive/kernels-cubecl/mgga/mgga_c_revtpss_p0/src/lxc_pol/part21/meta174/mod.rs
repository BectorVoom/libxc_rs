//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta174 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1087;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1088;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1089;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1090;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1091;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1092;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta174<F: Float>(t225: F, t4028: F, t4043: F, t1412: F, t73: F, t3829: F, t1394: F, t3889: F, t1392: F, t1395: F, t539: F, t541: F, t543: F, t1390: F, t828: F, t1389: F, t1408: F, t2736: F, t1388: F, t1410: F, t3970: F, t3976: F, t3982: F, t3987: F, t3990: F, t3996: F, t4002: F, t4006: F, t4014: F, t4022: F, t1370: F, t3926: F, t3931: F, t3934: F, t3940: F, t3944: F, t3946: F, t3950: F, t3953: F, t3956: F, t3958: F, t3961: F, t3967: F, t1419: F, t213: F, t1425: F, t560: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4045, t4050, t4053, t4056) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1087::<F>(t225, t4028, t4043, t1412, t73, t3829, t1394, t3889, t1392, t1395, t539, t541);
        let t4057 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1088::<F>(t4056, t543);
        let (t4059, t4062, t4064, t4065) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1089::<F>(t1390, t4057, t828, t1389, t1408, t2736, t1388, t1410, t3970, t3976, t3982, t3987, t3990, t3996, t4002, t4006, t4014, t4022);
        let t4066 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1090::<F>(t1370, t1388, t3926, t3931, t3934, t3940, t3944, t3946, t3950, t3953, t3956, t3958, t3961, t3967, t4065);
        let (t4067, t4071) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1091::<F>(t225, t4066, t1419, t213);
        let t4075 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1092::<F>(t1425, t560);
    (t4045, t4050, t4053, t4056, t4057, t4059, t4062, t4064, t4066, t4067, t4071, t4075)
}
