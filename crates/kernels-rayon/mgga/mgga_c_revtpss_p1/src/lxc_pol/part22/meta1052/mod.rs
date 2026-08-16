//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1052 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3713;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3714;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3715;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3716;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3717;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3718;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1052(t1261: f64, t20981: f64, t3172: f64, t13033: f64, t21188: f64, t20985: f64, t20820: f64, t3704: f64, t17720: f64, t5381: f64, t17214: f64, t17505: f64, t17536: f64, t17552: f64, t17786: f64, t20941: f64, t21275: f64, t21306: f64, t3591: f64, t44561: f64, t5299: f64, t5391: f64, t57118: f64, t58927: f64, t20810: f64, t3711: f64, t17412: f64, t5378: f64, t17416: f64, t12915: f64, t20721: f64, t247: f64, t5384: f64, t1214: f64, t21082: f64, t3584: f64, t6587: f64, t21192: f64, t3647: f64, t1042: f64, t12956: f64, t1715: f64, t17261: f64, t20876: f64, t20880: f64, t21246: f64, t3719: f64, t57125: f64, t57303: f64, t68669: f64, t21143: f64, t3636: f64, t17448: f64, t17580: f64, t17679: f64, t17684: f64, t17690: f64, t17732: f64, t21014: f64, t21017: f64, t21242: f64, t21267: f64, t3620: f64, t57128: f64, t57145: f64, t57164: f64, t57167: f64, t57170: f64, t57344: f64, t57707: f64, t70303: f64, t3666: f64, t6594: f64, t17283: f64, t5362: f64, t1238: f64, t12832: f64, t17280: f64, t17405: f64, t17672: f64, t1791: f64, t20851: f64, t21042: f64, t21177: f64, t3625: f64, t3626: f64, t3663: f64, t5320: f64, t5323: f64, t5373: f64, t57173: f64, t57176: f64, t57178: f64, t59025: f64, t6429: f64, t1222: f64, t140: f64, t21209: f64, t21213: f64, t3685: f64, t12865: f64, t5436: f64, t1012: f64, t1225: f64, t12866: f64, t17634: f64, t17661: f64, t17693: f64, t17696: f64, t20771: f64, t20937: f64, t56981: f64, t57191: f64, t57209: f64, t57212: f64, t57214: f64, t57222: f64, t57227: f64, t57378: f64, t60754: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t70390 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3713(t1261, t20981, t3172, t13033, t21188, t20985, t20820, t3704, t17720, t5381, t17214, t17505, t17536, t17552, t17786, t20941, t21275, t21306, t3591, t44561, t5299, t5391, t57118, t58927);
        let (t70394, t70403, t70405, t70411, t70413) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3714(t20810, t3172, t3711, t17412, t5378, t17416, t5381, t12915, t20721, t247, t5384, t1214, t21082);
        let (t70422, t70429) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3715(t3584, t6587, t21192, t3647, t1042, t12956, t1715, t17261, t20876, t20880, t21246, t247, t3711, t3719, t5384, t57125, t57303, t68669, t70394, t70403, t70405, t70411, t70413);
        let t70453 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3716(t21143, t3636, t17448, t17580, t17679, t17684, t17690, t17732, t21014, t21017, t21242, t21267, t3620, t57128, t57145, t57164, t57167, t57170, t57344, t57707, t70303);
        let t70480 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3717(t3666, t6594, t17283, t5362, t1238, t12832, t17280, t17405, t17672, t1791, t20851, t21042, t21177, t3625, t3626, t3663, t5320, t5323, t5373, t57173, t57176, t57178, t59025, t6429);
        let (t70496, t70508) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3718(t1222, t140, t21209, t21213, t3685, t12865, t5436, t1012, t1225, t12866, t17634, t17661, t17693, t17696, t20771, t20937, t56981, t57191, t57209, t57212, t57214, t57222, t57227, t57378, t60754);
    (t70390, t70413, t70422, t70429, t70453, t70480, t70496, t70508)
}
