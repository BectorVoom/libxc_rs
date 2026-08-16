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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta957(t2258: f64, t5825: f64, t18281: f64, t606: f64, t13312: f64, t13392: f64, t1486: f64, t1927: f64, t19680: f64, t21686: f64, t21695: f64, t21698: f64, t21699: f64, t21702: f64, t21727: f64, t21768: f64, t2291: f64, t2312: f64, t36: f64, t5826: f64, t5827: f64, t607: f64, t60754: f64, t627: f64, t641: f64, t70: f64, t85: f64, t4181: f64, t4186: f64, t13321: f64, t13331: f64, t1480: f64, t21745: f64, t21754: f64, t2270: f64, t2275: f64, t2282: f64, t2283: f64, t2286: f64, t44: f64, t46090: f64, t48: f64, t56: f64, t5835: f64, t5838: f64, t5843: f64, t60: f64, t60308: f64, t60311: f64, t60717: f64, t614: f64, t10355: f64, t10368: f64, t13325: f64, t13328: f64, t21732: f64, t21733: f64, t21736: f64, t21741: f64, t21742: f64, t21761: f64, t2251: f64, t4201: f64, t4210: f64, t46065: f64, t46074: f64, t5819: f64, t13334: f64, t13340: f64, t13393: f64, t13396: f64, t13397: f64, t13400: f64, t13405: f64, t1470: f64, t1494: f64, t21707: f64, t21710: f64, t21713: f64, t38: f64, t4182: f64, t4187: f64, t4217: f64, t4238: f64, t5830: f64, t10298: f64, t10301: f64, t10309: f64, t13283: f64, t13420: f64, t1497: f64, t21674: f64, t21677: f64, t21682: f64, t2247: f64, t2248: f64, t2315: f64, t4178: f64, t4241: f64, t45926: f64, t45931: f64, t45933: f64, t45936: f64, t45941: f64, t45944: f64, t45947: f64, t45952: f64, t45958: f64, t45963: f64, t45972: f64, t5816: f64, t5872: f64, t60224: f64, t603: f64, t60793: f64, t60829: f64, t91: f64, t5: f64, t60692: f64, t117: f64, t10416: f64, t1310: f64, t13425: f64, t13429: f64, t13435: f64, t1502: f64, t1518: f64, t18153: f64, t18220: f64, t18242: f64, t1843: f64, t21658: f64, t21814: f64, t2320: f64, t2322: f64, t3813: f64, t4246: f64, t508: f64, t5517: f64, t5877: f64, t5921: f64, t60650: f64, t60656: f64, t649: f64, t651: f64, t6765: f64) -> (f64, f64, f64, f64, f64) {
        let t60834 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3207(t2258, t5825);
        let t60838 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3208(t18281, t606);
        let t60871 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3209(t13312, t13392, t1486, t1927, t19680, t21686, t21695, t21698, t21699, t21702, t21727, t21768, t2291, t2312, t36, t5826, t5827, t607, t60754, t60834, t60838, t627, t641, t70, t85);
        let t60927 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3210(t4181, t4186);
        let t60937 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3211(t13321, t13331, t1480, t21745, t21754, t2258, t2270, t2275, t2282, t2283, t2286, t44, t46090, t48, t56, t5835, t5838, t5843, t60, t60308, t60311, t60717, t60754, t60927, t614);
        let t60987 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3212(t10355, t10368, t13312, t13325, t13328, t1480, t18281, t21732, t21733, t21736, t21741, t21742, t21761, t2251, t2258, t2275, t2282, t4201, t4210, t44, t46065, t46074, t56, t5819, t5825, t606, t614);
        let t60994 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3213(t13334, t13340, t13393, t13396, t13397, t13400, t13405, t1470, t1486, t1494, t21707, t21710, t21713, t2312, t38, t4181, t4182, t4187, t4217, t4238, t5830, t60937, t60987, t641, t85);
        let t61007 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3214(t10298, t10301, t10309, t13283, t13420, t1497, t21674, t21677, t21682, t2247, t2248, t2315, t4178, t4241, t45926, t45931, t45933, t45936, t45941, t45944, t45947, t45952, t45958, t45963, t45972, t5816, t5872, t60224, t603, t60793, t60829, t60871, t60994, t91);
        let (t61010, t61014) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3215(t5, t60692, t61007, t117, t10416, t1310, t13425, t13429, t13435, t1502, t1518, t18153, t18220, t18242, t1843, t21658, t21814, t2320, t2322, t3813, t4246, t508, t5517, t5877, t5921, t60650, t60656, t649, t651, t6765);
    (t60834, t60838, t60927, t61010, t61014)
}
