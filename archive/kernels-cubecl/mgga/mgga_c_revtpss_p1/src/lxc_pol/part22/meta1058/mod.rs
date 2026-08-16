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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1058<F: Float>(t17445: F, t5373: F, t12866: F, t20933: F, t56756: F, t17789: F, t21017: F, t3601: F, t6573: F, t12916: F, t17747: F, t20962: F, t12787: F, t13392: F, t17244: F, t17351: F, t17396: F, t17602: F, t17672: F, t17729: F, t20770: F, t20921: F, t3604: F, t3625: F, t3720: F, t44551: F, t44902: F, t44906: F, t44917: F, t56981: F, t59094: F, t6421: F, t3717: F, t70994: F, t1122: F, t1250: F, t1715: F, t17353: F, t17539: F, t17651: F, t17661: F, t17673: F, t17736: F, t17737: F, t17800: F, t20721: F, t20959: F, t3568: F, t3626: F, t3723: F, t44521: F, t44925: F, t44931: F, t57631: F, t57663: F, t59142: F, t59144: F, t59146: F, t70496: F, t1261: F, t20867: F, t3172: F, t12956: F, t20783: F, t3617: F, t6587: F, t17609: F, t5265: F, t17544: F, t5274: F, t1042: F, t1247: F, t13069: F, t17261: F, t17536: F, t17569: F, t17700: F, t20864: F, t20903: F, t21095: F, t3363: F, t3647: F, t3708: F, t3711: F, t482: F, t5381: F, t59149: F, t6625: F, t69609: F, t1222: F, t17471: F, t20298: F, t20302: F, t1260: F, t57465: F, t21334: F, t1266: F, t12832: F, t17265: F, t17347: F, t21143: F, t21166: F, t21275: F, t3600: F, t3640: F, t3644: F, t5302: F, t5312: F, t59159: F, t65433: F, t68324: F, t68355: F, t70343: F, t17763: F, t5378: F, t12800: F, t17344: F, t17354: F, t17401: F, t17514: F, t17724: F, t1808: F, t21272: F, t247: F, t3620: F, t3719: F, t58863: F, t59173: F, t59176: F, t59179: F, t59182: F, t59185: F, t6673: F, t71300: F, t12855: F, t20977: F, t17678: F, t17683: F, t17689: F, t17693: F, t17730: F, t17799: F, t20923: F, t21046: F, t3362: F, t3629: F, t4181: F, t44510: F, t44517: F, t5245: F, t56861: F, t57621: F, t58960: F, t59017: F, t59220: F, t70910: F, t70944: F, t71314: F, t71452: F, t20913: F, t3147: F, t6593: F, t3594: F, t3597: F, t1244: F, t17500: F, t17541: F, t17584: F, t20982: F, t20986: F, t21102: F, t3591: F, t3606: F, t3613: F, t5056: F, t5299: F, t5308: F, t5391: F, t57053: F, t68299: F, t68303: F, t21107: F, t3704: F, t17628: F, t16750: F, t1794: F, t12976: F, t16746: F, t17237: F, t17426: F, t17589: F, t20952: F, t21085: F, t21111: F, t3667: F, t3718: F, t5047: F, t5277: F, t5333: F, t59233: F, t59239: F, t6647: F, t71245: F, t20851: F, t3678: F, t17290: F, t5362: F, t17435: F, t5327: F, t3655: F, t6595: F, t1256: F, t21313: F, t21316: F, t17332: F, t17638: F, t17644: F, t17677: F, t17682: F, t1803: F, t20272: F, t20795: F, t484: F, t5331: F, t5340: F, t5405: F, t6425: F, t6429: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t71460, t71470, t71476, t71480, t71490) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3753::<F>(t17445, t5373, t12866, t20933, t56756, t17789, t21017, t3601, t6573, t12916, t17747, t20962);
        let t71492 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3754::<F>(t12787, t13392, t17244, t17351, t17396, t17602, t17672, t17729, t20770, t20921, t3604, t3625, t3720, t44551, t44902, t44906, t44917, t5373, t56981, t59094, t6421, t71460, t71470, t71476, t71480, t71490);
        let t71527 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3755::<F>(t3717, t70994, t1122, t1250, t12866, t1715, t17353, t17539, t17651, t17661, t17673, t17736, t17737, t17800, t20721, t20959, t3568, t3626, t3723, t44521, t44925, t44931, t57631, t57663, t59142, t59144, t59146, t70496);
        let t71560 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3756::<F>(t1261, t20867, t3172, t12956, t20783, t3617, t6587, t17609, t5265, t17544, t5274, t1042, t1247, t1250, t13069, t17261, t17536, t17569, t17700, t20864, t20903, t21095, t3363, t3647, t3708, t3711, t482, t5381, t59149, t6625, t69609);
        let t71597 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3757::<F>(t1222, t17471, t20298, t20302, t1260, t57465, t21334, t1042, t1261, t1266, t12832, t17265, t17347, t21143, t21166, t21275, t3600, t3604, t3640, t3644, t5302, t5312, t59159, t65433, t68324, t68355, t70343);
        let (t71606, t71624) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3758::<F>(t17763, t5378, t3568, t6587, t12800, t12866, t17344, t17351, t17354, t17401, t17514, t17724, t1808, t21272, t247, t3620, t3719, t58863, t59173, t59176, t59179, t59182, t59185, t6673, t71300);
        let t71667 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3759::<F>(t12855, t12916, t20977, t12787, t12866, t17661, t17678, t17683, t17689, t17693, t17729, t17730, t17736, t17799, t20921, t20923, t21046, t3362, t3625, t3626, t3629, t4181, t44510, t44517, t5245, t56861, t57621, t58960, t59017, t59220, t6421, t70910, t70944, t71314, t71452);
        let t71704 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3760::<F>(t20913, t3172, t3711, t3147, t6593, t3594, t3597, t1244, t1042, t1222, t17500, t17541, t17569, t17584, t17700, t20982, t20986, t21102, t3591, t3606, t3613, t3647, t5056, t5299, t5308, t5391, t57053, t68299, t68303);
        let (t71724, t71737) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3761::<F>(t21107, t3704, t17628, t5373, t16750, t1794, t1042, t1250, t12976, t16746, t17237, t17351, t17426, t17569, t17589, t20952, t21085, t21111, t3647, t3667, t3711, t3718, t3720, t5047, t5277, t5333, t5391, t59233, t59239, t6647, t71245);
        let t71781 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3762::<F>(t20851, t3678, t17290, t5362, t17435, t5327, t3655, t6595, t1256, t21313, t21316, t17332, t17638, t17644, t17672, t17677, t17682, t1803, t20272, t20795, t3625, t3626, t484, t5331, t5340, t5405, t6425, t6429);
    (t71480, t71492, t71527, t71560, t71597, t71606, t71624, t71667, t71704, t71724, t71737, t71781)
}
