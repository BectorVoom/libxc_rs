//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1052 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3713;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3714;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3715;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3716;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3717;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3718;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1052<F: Float>(t1261: F, t20981: F, t3172: F, t13033: F, t21188: F, t20985: F, t20820: F, t3704: F, t17720: F, t5381: F, t17214: F, t17505: F, t17536: F, t17552: F, t17786: F, t20941: F, t21275: F, t21306: F, t3591: F, t44561: F, t5299: F, t5391: F, t57118: F, t58927: F, t20810: F, t3711: F, t17412: F, t5378: F, t17416: F, t12915: F, t20721: F, t247: F, t5384: F, t1214: F, t21082: F, t3584: F, t6587: F, t21192: F, t3647: F, t1042: F, t12956: F, t1715: F, t17261: F, t20876: F, t20880: F, t21246: F, t3719: F, t57125: F, t57303: F, t68669: F, t21143: F, t3636: F, t17448: F, t17580: F, t17679: F, t17684: F, t17690: F, t17732: F, t21014: F, t21017: F, t21242: F, t21267: F, t3620: F, t57128: F, t57145: F, t57164: F, t57167: F, t57170: F, t57344: F, t57707: F, t70303: F, t3666: F, t6594: F, t17283: F, t5362: F, t1238: F, t12832: F, t17280: F, t17405: F, t17672: F, t1791: F, t20851: F, t21042: F, t21177: F, t3625: F, t3626: F, t3663: F, t5320: F, t5323: F, t5373: F, t57173: F, t57176: F, t57178: F, t59025: F, t6429: F, t1222: F, t140: F, t21209: F, t21213: F, t3685: F, t12865: F, t5436: F, t1012: F, t1225: F, t12866: F, t17634: F, t17661: F, t17693: F, t17696: F, t20771: F, t20937: F, t56981: F, t57191: F, t57209: F, t57212: F, t57214: F, t57222: F, t57227: F, t57378: F, t60754: F) -> (F, F, F, F, F, F, F, F) {
        let t70390 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3713::<F>(t1261, t20981, t3172, t13033, t21188, t20985, t20820, t3704, t17720, t5381, t17214, t17505, t17536, t17552, t17786, t20941, t21275, t21306, t3591, t44561, t5299, t5391, t57118, t58927);
        let (t70394, t70403, t70405, t70411, t70413) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3714::<F>(t20810, t3172, t3711, t17412, t5378, t17416, t5381, t12915, t20721, t247, t5384, t1214, t21082);
        let (t70422, t70429) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3715::<F>(t3584, t6587, t21192, t3647, t1042, t12956, t1715, t17261, t20876, t20880, t21246, t247, t3711, t3719, t5384, t57125, t57303, t68669, t70394, t70403, t70405, t70411, t70413);
        let t70453 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3716::<F>(t21143, t3636, t17448, t17580, t17679, t17684, t17690, t17732, t21014, t21017, t21242, t21267, t3620, t57128, t57145, t57164, t57167, t57170, t57344, t57707, t70303);
        let t70480 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3717::<F>(t3666, t6594, t17283, t5362, t1238, t12832, t17280, t17405, t17672, t1791, t20851, t21042, t21177, t3625, t3626, t3663, t5320, t5323, t5373, t57173, t57176, t57178, t59025, t6429);
        let (t70496, t70508) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3718::<F>(t1222, t140, t21209, t21213, t3685, t12865, t5436, t1012, t1225, t12866, t17634, t17661, t17693, t17696, t20771, t20937, t56981, t57191, t57209, t57212, t57214, t57222, t57227, t57378, t60754);
    (t70390, t70413, t70422, t70429, t70453, t70480, t70496, t70508)
}
