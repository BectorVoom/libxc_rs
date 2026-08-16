//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta852 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3202;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3203;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3204;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3205;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3206;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3207;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3208;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3209;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3210;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3211;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta852(t45786: f64, t58919: f64, t17708: f64, t45846: f64, t12975: f64, t1803: f64, t225: f64, t56412: f64, t480: f64, t12984: f64, t5323: f64, t12916: f64, t17390: f64, t3718: f64, t1238: f64, t12732: f64, t12866: f64, t12972: f64, t13029: f64, t13043: f64, t17283: f64, t17290: f64, t17514: f64, t17515: f64, t17756: f64, t3603: f64, t3663: f64, t3720: f64, t44561: f64, t44823: f64, t44829: f64, t44838: f64, t44884: f64, t5327: f64, t5332: f64, t5340: f64, t5373: f64, t56981: f64, t58921: f64, t17500: f64, t372: f64, t13142: f64, t56878: f64, t56756: f64, t17723: f64, t1774: f64, t1248: f64, t12776: f64, t12835: f64, t13053: f64, t13063: f64, t17353: f64, t17513: f64, t17654: f64, t17661: f64, t17662: f64, t2251: f64, t3367: f64, t3604: f64, t3630: f64, t44510: f64, t44578: f64, t44769: f64, t44886: f64, t44888: f64, t44892: f64, t44898: f64, t44902: f64, t44906: f64, t44912: f64, t45371: f64, t5341: f64, t5354: f64, t56999: f64, t58909: f64, t12832: f64, t17617: f64, t12851: f64, t1778: f64, t17429: f64, t17789: f64, t12910: f64, t17624: f64, t11231: f64, t12777: f64, t12784: f64, t12816: f64, t12920: f64, t17222: f64, t17412: f64, t17605: f64, t17623: f64, t17633: f64, t17635: f64, t17729: f64, t3620: f64, t3626: f64, t3644: f64, t3708: f64, t44917: f64, t44925: f64, t44928: f64, t44931: f64, t5046: f64, t5352: f64, t5391: f64, t57005: f64, t57275: f64, t17709: f64, t17712: f64, t3766: f64, t5219: f64, t5330: f64, t17601: f64, t12855: f64, t17579: f64, t12809: f64, t17483: f64, t12772: f64, t17731: f64, t44546: f64, t5353: f64, t3588: f64, t5245: f64, t45833: f64, t1042: f64, t1250: f64, t12787: f64, t12858: f64, t16756: f64, t17420: f64, t17426: f64, t17625: f64, t17638: f64, t17646: f64, t17711: f64, t17736: f64, t17760: f64, t17784: f64, t20921: f64, t3363: f64, t3617: f64, t3625: f64, t3711: f64, t44624: f64, t44938: f64, t471: f64, t5331: f64, t56861: f64, t57536: f64, t127: f64, t17693: f64, t17695: f64, t5302: f64, t1261: f64, t12879: f64, t247: f64, t5056: f64, t12963: f64, t56587: f64, t12287: f64, t12705: f64, t12712: f64, t12938: f64, t13022: f64, t13046: f64, t17351: f64, t17354: f64, t17391: f64, t17505: f64, t17669: f64, t3629: f64, t3674: f64, t3719: f64, t44500: f64, t44949: f64, t44965: f64, t44972: f64, t44980: f64, t5384: f64, t56530: f64, t17795: f64, t3172: f64, t1214: f64, t17759: f64, t44425: f64, t29048: f64, t3362: f64, t10326: f64, t10356: f64, t12931: f64, t16719: f64, t16724: f64, t17344: f64, t17482: f64, t17558: f64, t17580: f64, t17687: f64, t17688: f64, t17730: f64, t17781: f64, t3368: f64, t3568: f64, t3628: f64, t3647: f64, t4186: f64, t44484: f64, t44551: f64, t45346: f64, t45352: f64, t51959: f64, t5296: f64, t5351: f64, t56620: f64, t57548: f64, t58969: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t59011, t59017, t59025, t59032, t59033, t59041, t59043) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3202(t45786, t58919, t17708, t45846, t12975, t1803, t225, t56412, t480, t12984, t5323, t12916, t17390, t3718);
        let t59056 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3203(t1238, t12732, t12866, t12972, t13029, t13043, t17283, t17290, t17514, t17515, t17756, t3603, t3663, t3720, t44561, t44823, t44829, t44838, t44884, t5323, t5327, t5332, t5340, t5373, t56981, t58921, t59011, t59017, t59025, t59033, t59041, t59043);
        let (t59062, t59066, t59078, t59094, t59096) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3204(t17500, t372, t13142, t56878, t12866, t17514, t56756, t12916, t17723, t3718, t13043, t1774);
        let t59108 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3205(t1248, t12776, t12835, t12866, t13053, t13063, t17353, t17513, t17654, t17661, t17662, t2251, t3367, t3604, t3630, t3720, t44510, t44561, t44578, t44769, t44886, t44888, t44892, t44898, t44902, t44906, t44912, t45371, t5341, t5354, t56999, t58909, t59062, t59066, t59078, t59094, t59096);
        let t59151 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3206(t12832, t17617, t12851, t1778, t17429, t17789, t12910, t12916, t17624, t11231, t12777, t12784, t12816, t12920, t17222, t17412, t17605, t17623, t17633, t17635, t17729, t3620, t3626, t3644, t3708, t3718, t3720, t44917, t44925, t44928, t44931, t5046, t5352, t5391, t57005, t57275);
        let (t59159, t59162, t59173, t59176, t59179) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3207(t12916, t17709, t17712, t3766, t5219, t5330, t17601, t3718, t12855, t17579, t12809, t17483);
        let (t59187, t59215) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3208(t12772, t17729, t17731, t3718, t44546, t5353, t3588, t5245, t45833, t58919, t1042, t1250, t12784, t12787, t12858, t12920, t13043, t16756, t17420, t17426, t17625, t17633, t17638, t17646, t17709, t17711, t17736, t17760, t17784, t20921, t3363, t3617, t3625, t3626, t3711, t3720, t44624, t44938, t471, t5331, t56861, t57536, t58921, t59159, t59162, t59173, t59176, t59179);
        let (t59220, t59233, t59239, t59241) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3209(t127, t17693, t17695, t5302, t1261, t12879, t247, t5056, t12963, t5323, t225, t56587);
        let t59267 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3210(t480, t59241, t12287, t1250, t12705, t12712, t12784, t12832, t12938, t13022, t13046, t17351, t17353, t17354, t17391, t17505, t17638, t17669, t17693, t247, t3629, t3674, t3719, t3720, t44500, t44949, t44965, t44972, t44980, t5373, t5384, t56530, t56981, t59096, t59220, t59233, t59239);
        let t59334 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3211(t17795, t3172, t3711, t1214, t3604, t17729, t17759, t44425, t29048, t3362, t10326, t10356, t1042, t12787, t12920, t12931, t16719, t16724, t17344, t17429, t17482, t17500, t17558, t17580, t17633, t17687, t17688, t17730, t17781, t247, t3368, t3568, t3625, t3626, t3628, t3647, t3719, t3720, t4186, t44484, t44551, t45346, t45352, t5046, t51959, t5296, t5351, t5384, t56620, t57548, t58969);
    (t59032, t59056, t59096, t59108, t59151, t59187, t59215, t59241, t59267, t59334)
}
