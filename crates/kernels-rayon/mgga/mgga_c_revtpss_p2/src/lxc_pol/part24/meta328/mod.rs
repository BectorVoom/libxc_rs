//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta328 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1139;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1140;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1141;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1142;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1143;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1144;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1145;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta328(t1390: f64, t22954: f64, t828: f64, t1388: f64, t13959: f64, t14013: f64, t14043: f64, t22156: f64, t22179: f64, t22183: f64, t22260: f64, t22264: f64, t22268: f64, t22285: f64, t22292: f64, t22914: f64, t9953: f64, t22840: f64, t22874: f64, t22903: f64, t225: f64, t1903: f64, t6918: f64, t4076: f64, t6895: f64, t9657: f64, t13727: f64, t1424: f64, t213: f64, t22400: f64, t22405: f64, t22407: f64, t22410: f64, t561: f64, t5715: f64, t6896: f64, t9639: f64, t9650: f64, t9666: f64, t9691: f64, t9694: f64, t10035: f64, t10090: f64, t10102: f64, t14120: f64, t14149: f64, t14161: f64, t14166: f64, t14171: f64, t14203: f64, t14221: f64, t1437: f64, t1883: f64, t22316: f64, t22321: f64, t22858: f64, t22863: f64, t22912: f64, t4114: f64, t5767: f64, t6844: f64, t6862: f64, t6874: f64, t820: f64, t4003: f64, t6843: f64, t10114: f64, t10117: f64, t10126: f64, t10129: f64, t14243: f64, t14252: f64, t22009: f64, t22329: f64, t22333: f64, t22337: f64, t22353: f64, t22362: f64, t22366: f64, t22370: f64, t22374: f64, t22381: f64, t546: f64, t5735: f64, t5745: f64, t5755: f64, t1427: f64, t10157: f64, t14091: f64, t14097: f64, t14105: f64, t14280: f64, t14290: f64, t14294: f64, t14297: f64, t1904: f64, t22390: f64, t22428: f64, t22447: f64, t22450: f64, t22454: f64, t6919: f64, t1343: f64, t1450: f64, t198: f64, t22768: f64, t22791: f64, t22809: f64, t22919: f64, t22920: f64, t22921: f64, t22922: f64, t532: f64, t9394: f64, t9396: f64, t9409: f64, t9412: f64, t9415: f64, t9421: f64, t9427: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22956, t22962) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1139(t1390, t22954, t828, t1388, t13959, t14013, t14043, t22156, t22179, t22183, t22260, t22264, t22268, t22285, t22292, t22914, t9953);
        let t22964 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1140(t22840, t22874, t22903, t22962);
        let (t22965, t22970, t22971, t22974, t22975, t22984) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1141(t225, t22964, t1903, t6918, t4076, t6895, t9657, t13727, t1424, t213, t22400, t22405, t22407, t22410, t561, t5715, t6896, t9639, t9650, t9666, t9691, t9694);
        let t23019 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1142(t10035, t10090, t10102, t14120, t14149, t14161, t14166, t14171, t14203, t14221, t1437, t1883, t22316, t22321, t22858, t22863, t22912, t22954, t4114, t5767, t6844, t6862, t6874, t820);
        let (t23037, t23041) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1143(t4003, t6843, t10114, t10117, t10126, t10129, t14243, t14252, t1883, t213, t22009, t22329, t22333, t22337, t22353, t22362, t22366, t22370, t22374, t22381, t22964, t546, t5735, t5745, t5755);
        let (t23042, t23043, t23058) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1144(t23019, t23041, t1427, t10157, t14091, t14097, t14105, t1424, t14280, t14290, t14294, t14297, t1904, t22390, t22428, t22447, t22450, t22454, t5715, t6919);
        let (t23059, t23063) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1145(t22984, t23058, t1343, t1450, t198, t22768, t22791, t22809, t22919, t22920, t22921, t22922, t532, t9394, t9396, t9409, t9412, t9415, t9421, t9427);
    (t22956, t22964, t22965, t22970, t22971, t22974, t22975, t23037, t23042, t23043, t23059, t23063)
}
