//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta328 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1139;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1140;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1141;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1142;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1143;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1144;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1145;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta328<F: Float>(t1390: F, t22954: F, t828: F, t1388: F, t13959: F, t14013: F, t14043: F, t22156: F, t22179: F, t22183: F, t22260: F, t22264: F, t22268: F, t22285: F, t22292: F, t22914: F, t9953: F, t22840: F, t22874: F, t22903: F, t225: F, t1903: F, t6918: F, t4076: F, t6895: F, t9657: F, t13727: F, t1424: F, t213: F, t22400: F, t22405: F, t22407: F, t22410: F, t561: F, t5715: F, t6896: F, t9639: F, t9650: F, t9666: F, t9691: F, t9694: F, t10035: F, t10090: F, t10102: F, t14120: F, t14149: F, t14161: F, t14166: F, t14171: F, t14203: F, t14221: F, t1437: F, t1883: F, t22316: F, t22321: F, t22858: F, t22863: F, t22912: F, t4114: F, t5767: F, t6844: F, t6862: F, t6874: F, t820: F, t4003: F, t6843: F, t10114: F, t10117: F, t10126: F, t10129: F, t14243: F, t14252: F, t22009: F, t22329: F, t22333: F, t22337: F, t22353: F, t22362: F, t22366: F, t22370: F, t22374: F, t22381: F, t546: F, t5735: F, t5745: F, t5755: F, t1427: F, t10157: F, t14091: F, t14097: F, t14105: F, t14280: F, t14290: F, t14294: F, t14297: F, t1904: F, t22390: F, t22428: F, t22447: F, t22450: F, t22454: F, t6919: F, t1343: F, t1450: F, t198: F, t22768: F, t22791: F, t22809: F, t22919: F, t22920: F, t22921: F, t22922: F, t532: F, t9394: F, t9396: F, t9409: F, t9412: F, t9415: F, t9421: F, t9427: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t22956, t22962) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1139::<F>(t1390, t22954, t828, t1388, t13959, t14013, t14043, t22156, t22179, t22183, t22260, t22264, t22268, t22285, t22292, t22914, t9953);
        let t22964 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1140::<F>(t22840, t22874, t22903, t22962);
        let (t22965, t22970, t22971, t22974, t22975, t22984) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1141::<F>(t225, t22964, t1903, t6918, t4076, t6895, t9657, t13727, t1424, t213, t22400, t22405, t22407, t22410, t561, t5715, t6896, t9639, t9650, t9666, t9691, t9694);
        let t23019 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1142::<F>(t10035, t10090, t10102, t14120, t14149, t14161, t14166, t14171, t14203, t14221, t1437, t1883, t22316, t22321, t22858, t22863, t22912, t22954, t4114, t5767, t6844, t6862, t6874, t820);
        let (t23037, t23041) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1143::<F>(t4003, t6843, t10114, t10117, t10126, t10129, t14243, t14252, t1883, t213, t22009, t22329, t22333, t22337, t22353, t22362, t22366, t22370, t22374, t22381, t22964, t546, t5735, t5745, t5755);
        let (t23042, t23043, t23058) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1144::<F>(t23019, t23041, t1427, t10157, t14091, t14097, t14105, t1424, t14280, t14290, t14294, t14297, t1904, t22390, t22428, t22447, t22450, t22454, t5715, t6919);
        let (t23059, t23063) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1145::<F>(t22984, t23058, t1343, t1450, t198, t22768, t22791, t22809, t22919, t22920, t22921, t22922, t532, t9394, t9396, t9409, t9412, t9415, t9421, t9427);
    (t22956, t22964, t22965, t22970, t22971, t22974, t22975, t23037, t23042, t23043, t23059, t23063)
}
