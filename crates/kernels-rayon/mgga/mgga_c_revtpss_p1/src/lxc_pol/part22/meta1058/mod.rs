//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1058 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3753;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3754;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3755;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3756;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3757;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3758;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3759;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3760;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3761;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3762;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1058(t17445: f64, t5373: f64, t12866: f64, t20933: f64, t56756: f64, t17789: f64, t21017: f64, t3601: f64, t6573: f64, t12916: f64, t17747: f64, t20962: f64, t12787: f64, t13392: f64, t17244: f64, t17351: f64, t17396: f64, t17602: f64, t17672: f64, t17729: f64, t20770: f64, t20921: f64, t3604: f64, t3625: f64, t3720: f64, t44551: f64, t44902: f64, t44906: f64, t44917: f64, t56981: f64, t59094: f64, t6421: f64, t3717: f64, t70994: f64, t1122: f64, t1250: f64, t1715: f64, t17353: f64, t17539: f64, t17651: f64, t17661: f64, t17673: f64, t17736: f64, t17737: f64, t17800: f64, t20721: f64, t20959: f64, t3568: f64, t3626: f64, t3723: f64, t44521: f64, t44925: f64, t44931: f64, t57631: f64, t57663: f64, t59142: f64, t59144: f64, t59146: f64, t70496: f64, t1261: f64, t20867: f64, t3172: f64, t12956: f64, t20783: f64, t3617: f64, t6587: f64, t17609: f64, t5265: f64, t17544: f64, t5274: f64, t1042: f64, t1247: f64, t13069: f64, t17261: f64, t17536: f64, t17569: f64, t17700: f64, t20864: f64, t20903: f64, t21095: f64, t3363: f64, t3647: f64, t3708: f64, t3711: f64, t482: f64, t5381: f64, t59149: f64, t6625: f64, t69609: f64, t1222: f64, t17471: f64, t20298: f64, t20302: f64, t1260: f64, t57465: f64, t21334: f64, t1266: f64, t12832: f64, t17265: f64, t17347: f64, t21143: f64, t21166: f64, t21275: f64, t3600: f64, t3640: f64, t3644: f64, t5302: f64, t5312: f64, t59159: f64, t65433: f64, t68324: f64, t68355: f64, t70343: f64, t17763: f64, t5378: f64, t12800: f64, t17344: f64, t17354: f64, t17401: f64, t17514: f64, t17724: f64, t1808: f64, t21272: f64, t247: f64, t3620: f64, t3719: f64, t58863: f64, t59173: f64, t59176: f64, t59179: f64, t59182: f64, t59185: f64, t6673: f64, t71300: f64, t12855: f64, t20977: f64, t17678: f64, t17683: f64, t17689: f64, t17693: f64, t17730: f64, t17799: f64, t20923: f64, t21046: f64, t3362: f64, t3629: f64, t4181: f64, t44510: f64, t44517: f64, t5245: f64, t56861: f64, t57621: f64, t58960: f64, t59017: f64, t59220: f64, t70910: f64, t70944: f64, t71314: f64, t71452: f64, t20913: f64, t3147: f64, t6593: f64, t3594: f64, t3597: f64, t1244: f64, t17500: f64, t17541: f64, t17584: f64, t20982: f64, t20986: f64, t21102: f64, t3591: f64, t3606: f64, t3613: f64, t5056: f64, t5299: f64, t5308: f64, t5391: f64, t57053: f64, t68299: f64, t68303: f64, t21107: f64, t3704: f64, t17628: f64, t16750: f64, t1794: f64, t12976: f64, t16746: f64, t17237: f64, t17426: f64, t17589: f64, t20952: f64, t21085: f64, t21111: f64, t3667: f64, t3718: f64, t5047: f64, t5277: f64, t5333: f64, t59233: f64, t59239: f64, t6647: f64, t71245: f64, t20851: f64, t3678: f64, t17290: f64, t5362: f64, t17435: f64, t5327: f64, t3655: f64, t6595: f64, t1256: f64, t21313: f64, t21316: f64, t17332: f64, t17638: f64, t17644: f64, t17677: f64, t17682: f64, t1803: f64, t20272: f64, t20795: f64, t484: f64, t5331: f64, t5340: f64, t5405: f64, t6425: f64, t6429: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t71460, t71470, t71476, t71480, t71490) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3753(t17445, t5373, t12866, t20933, t56756, t17789, t21017, t3601, t6573, t12916, t17747, t20962);
        let t71492 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3754(t12787, t13392, t17244, t17351, t17396, t17602, t17672, t17729, t20770, t20921, t3604, t3625, t3720, t44551, t44902, t44906, t44917, t5373, t56981, t59094, t6421, t71460, t71470, t71476, t71480, t71490);
        let t71527 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3755(t3717, t70994, t1122, t1250, t12866, t1715, t17353, t17539, t17651, t17661, t17673, t17736, t17737, t17800, t20721, t20959, t3568, t3626, t3723, t44521, t44925, t44931, t57631, t57663, t59142, t59144, t59146, t70496);
        let t71560 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3756(t1261, t20867, t3172, t12956, t20783, t3617, t6587, t17609, t5265, t17544, t5274, t1042, t1247, t1250, t13069, t17261, t17536, t17569, t17700, t20864, t20903, t21095, t3363, t3647, t3708, t3711, t482, t5381, t59149, t6625, t69609);
        let t71597 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3757(t1222, t17471, t20298, t20302, t1260, t57465, t21334, t1042, t1261, t1266, t12832, t17265, t17347, t21143, t21166, t21275, t3600, t3604, t3640, t3644, t5302, t5312, t59159, t65433, t68324, t68355, t70343);
        let (t71606, t71624) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3758(t17763, t5378, t3568, t6587, t12800, t12866, t17344, t17351, t17354, t17401, t17514, t17724, t1808, t21272, t247, t3620, t3719, t58863, t59173, t59176, t59179, t59182, t59185, t6673, t71300);
        let t71667 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3759(t12855, t12916, t20977, t12787, t12866, t17661, t17678, t17683, t17689, t17693, t17729, t17730, t17736, t17799, t20921, t20923, t21046, t3362, t3625, t3626, t3629, t4181, t44510, t44517, t5245, t56861, t57621, t58960, t59017, t59220, t6421, t70910, t70944, t71314, t71452);
        let t71704 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3760(t20913, t3172, t3711, t3147, t6593, t3594, t3597, t1244, t1042, t1222, t17500, t17541, t17569, t17584, t17700, t20982, t20986, t21102, t3591, t3606, t3613, t3647, t5056, t5299, t5308, t5391, t57053, t68299, t68303);
        let (t71724, t71737) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3761(t21107, t3704, t17628, t5373, t16750, t1794, t1042, t1250, t12976, t16746, t17237, t17351, t17426, t17569, t17589, t20952, t21085, t21111, t3647, t3667, t3711, t3718, t3720, t5047, t5277, t5333, t5391, t59233, t59239, t6647, t71245);
        let t71781 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3762(t20851, t3678, t17290, t5362, t17435, t5327, t3655, t6595, t1256, t21313, t21316, t17332, t17638, t17644, t17672, t17677, t17682, t1803, t20272, t20795, t3625, t3626, t484, t5331, t5340, t5405, t6425, t6429);
    (t71480, t71492, t71527, t71560, t71597, t71606, t71624, t71667, t71704, t71724, t71737, t71781)
}
