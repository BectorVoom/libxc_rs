//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta957 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3207;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3208;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3209;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3210;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3211;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3212;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3213;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3214;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3215;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta957<F: Float>(t2258: F, t5825: F, t18281: F, t606: F, t13312: F, t13392: F, t1486: F, t1927: F, t19680: F, t21686: F, t21695: F, t21698: F, t21699: F, t21702: F, t21727: F, t21768: F, t2291: F, t2312: F, t36: F, t5826: F, t5827: F, t607: F, t60754: F, t627: F, t641: F, t70: F, t85: F, t4181: F, t4186: F, t13321: F, t13331: F, t1480: F, t21745: F, t21754: F, t2270: F, t2275: F, t2282: F, t2283: F, t2286: F, t44: F, t46090: F, t48: F, t56: F, t5835: F, t5838: F, t5843: F, t60: F, t60308: F, t60311: F, t60717: F, t614: F, t10355: F, t10368: F, t13325: F, t13328: F, t21732: F, t21733: F, t21736: F, t21741: F, t21742: F, t21761: F, t2251: F, t4201: F, t4210: F, t46065: F, t46074: F, t5819: F, t13334: F, t13340: F, t13393: F, t13396: F, t13397: F, t13400: F, t13405: F, t1470: F, t1494: F, t21707: F, t21710: F, t21713: F, t38: F, t4182: F, t4187: F, t4217: F, t4238: F, t5830: F, t10298: F, t10301: F, t10309: F, t13283: F, t13420: F, t1497: F, t21674: F, t21677: F, t21682: F, t2247: F, t2248: F, t2315: F, t4178: F, t4241: F, t45926: F, t45931: F, t45933: F, t45936: F, t45941: F, t45944: F, t45947: F, t45952: F, t45958: F, t45963: F, t45972: F, t5816: F, t5872: F, t60224: F, t603: F, t60793: F, t60829: F, t91: F, t5: F, t60692: F, t117: F, t10416: F, t1310: F, t13425: F, t13429: F, t13435: F, t1502: F, t1518: F, t18153: F, t18220: F, t18242: F, t1843: F, t21658: F, t21814: F, t2320: F, t2322: F, t3813: F, t4246: F, t508: F, t5517: F, t5877: F, t5921: F, t60650: F, t60656: F, t649: F, t651: F, t6765: F) -> (F, F, F, F, F) {
        let t60834 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3207::<F>(t2258, t5825);
        let t60838 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3208::<F>(t18281, t606);
        let t60871 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3209::<F>(t13312, t13392, t1486, t1927, t19680, t21686, t21695, t21698, t21699, t21702, t21727, t21768, t2291, t2312, t36, t5826, t5827, t607, t60754, t60834, t60838, t627, t641, t70, t85);
        let t60927 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3210::<F>(t4181, t4186);
        let t60937 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3211::<F>(t13321, t13331, t1480, t21745, t21754, t2258, t2270, t2275, t2282, t2283, t2286, t44, t46090, t48, t56, t5835, t5838, t5843, t60, t60308, t60311, t60717, t60754, t60927, t614);
        let t60987 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3212::<F>(t10355, t10368, t13312, t13325, t13328, t1480, t18281, t21732, t21733, t21736, t21741, t21742, t21761, t2251, t2258, t2275, t2282, t4201, t4210, t44, t46065, t46074, t56, t5819, t5825, t606, t614);
        let t60994 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3213::<F>(t13334, t13340, t13393, t13396, t13397, t13400, t13405, t1470, t1486, t1494, t21707, t21710, t21713, t2312, t38, t4181, t4182, t4187, t4217, t4238, t5830, t60937, t60987, t641, t85);
        let t61007 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3214::<F>(t10298, t10301, t10309, t13283, t13420, t1497, t21674, t21677, t21682, t2247, t2248, t2315, t4178, t4241, t45926, t45931, t45933, t45936, t45941, t45944, t45947, t45952, t45958, t45963, t45972, t5816, t5872, t60224, t603, t60793, t60829, t60871, t60994, t91);
        let (t61010, t61014) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3215::<F>(t5, t60692, t61007, t117, t10416, t1310, t13425, t13429, t13435, t1502, t1518, t18153, t18220, t18242, t1843, t21658, t21814, t2320, t2322, t3813, t4246, t508, t5517, t5877, t5921, t60650, t60656, t649, t651, t6765);
    (t60834, t60838, t60927, t61010, t61014)
}
