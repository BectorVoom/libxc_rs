//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1059 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3763;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3764;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3765;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3766;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3767;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3768;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3769;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3770;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1059(t1261: f64, t1264: f64, t12809: f64, t12866: f64, t12910: f64, t16696: f64, t17369: f64, t17412: f64, t17459: f64, t17649: f64, t17661: f64, t17668: f64, t17736: f64, t20956: f64, t21028: f64, t21035: f64, t21164: f64, t21182: f64, t21257: f64, t247: f64, t3626: f64, t3720: f64, t44980: f64, t45371: f64, t5381: f64, t5397: f64, t5405: f64, t5406: f64, t57548: f64, t59062: f64, t59269: f64, t59330: f64, t60927: f64, t68251: f64, t71452: f64, t20272: f64, t3634: f64, t3584: f64, t6573: f64, t12916: f64, t20951: f64, t5340: f64, t17170: f64, t1774: f64, t17396: f64, t17620: f64, t1250: f64, t12800: f64, t16729: f64, t17344: f64, t17351: f64, t17467: f64, t17514: f64, t17693: f64, t20945: f64, t21153: f64, t3647: f64, t3718: f64, t3719: f64, t44521: f64, t5052: f64, t5333: f64, t5373: f64, t5391: f64, t58909: f64, t6679: f64, t68391: f64, t71061: f64, t17472: f64, t1222: f64, t17471: f64, t20266: f64, t20770: f64, t56756: f64, t1214: f64, t12839: f64, t17352: f64, t17475: f64, t17479: f64, t17643: f64, t21213: f64, t3701: f64, t372: f64, t44510: f64, t44769: f64, t5312: f64, t59320: f64, t59336: f64, t6690: f64, t68285: f64, t68290: f64, t68340: f64, t70932: f64, t73: f64, t17729: f64, t20922: f64, t44425: f64, t17617: f64, t6658: f64, t697: f64, t6662: f64, t12268: f64, t12787: f64, t15936: f64, t17580: f64, t17625: f64, t17730: f64, t20317: f64, t5354: f64, t56953: f64, t57147: f64, t59162: f64, t59338: f64, t59349: f64, t59351: f64, t59353: f64, t3588: f64, t6587: f64, t20801: f64, t20805: f64, t5331: f64, t12784: f64, t21090: f64, t13392: f64, t17534: f64, t17694: f64, t17742: f64, t20921: f64, t21040: f64, t5046: f64, t5330: f64, t5343: f64, t59358: f64, t59360: f64, t59492: f64, t69848: f64, t20293: f64, t57484: f64, t17735: f64, t70646: f64, t17423: f64, t21014: f64, t17708: f64, t59498: f64, t1042: f64, t17505: f64, t17584: f64, t17589: f64, t17739: f64, t17750: f64, t17800: f64, t20795: f64, t21093: f64, t3368: f64, t3372: f64, t44551: f64, t5384: f64, t58803: f64, t59379: f64, t59386: f64, t59391: f64, t71440: f64, t21041: f64, t20957: f64, t13396: f64, t17391: f64, t17429: f64, t17737: f64, t20858: f64, t21173: f64, t3367: f64, t4181: f64, t44500: f64, t44624: f64, t5245: f64, t5348: f64, t59404: f64, t59406: f64, t59408: f64, t59415: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t71824 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3763(t1261, t1264, t12809, t12866, t12910, t16696, t17369, t17412, t17459, t17649, t17661, t17668, t17736, t20956, t21028, t21035, t21164, t21182, t21257, t247, t3626, t3720, t44980, t45371, t5381, t5397, t5405, t5406, t57548, t59062, t59269, t59330, t60927, t68251, t71452);
        let (t71827, t71839, t71845, t71854, t71859) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3764(t1261, t20272, t247, t3634, t3584, t6573, t12916, t20951, t5340, t17170, t1774, t17396, t17620);
        let t71867 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3765(t1250, t1261, t1264, t12800, t16729, t17344, t17351, t17369, t17467, t17514, t17693, t20945, t21153, t247, t3647, t3718, t3719, t3720, t44521, t5052, t5333, t5373, t5391, t58909, t6679, t68391, t71061, t71827, t71839, t71845, t71854, t71859);
        let t71905 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3766(t17472, t5373, t1222, t17471, t20266, t17351, t20770, t56756, t1214, t12839, t12866, t17352, t17475, t17479, t17643, t21213, t3701, t372, t44510, t44769, t5312, t58909, t59320, t59336, t6690, t68285, t68290, t68340, t70932, t71452, t73);
        let t71936 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3767(t17729, t20922, t44425, t17396, t17617, t1222, t6658, t697, t6662, t12268, t12787, t15936, t17580, t17625, t17730, t1774, t20317, t3626, t5354, t56953, t57147, t59162, t59338, t59349, t59351, t59353);
        let (t71940, t71945, t71981) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3768(t3588, t6587, t6573, t12916, t20801, t5340, t20805, t5331, t12784, t21090, t1250, t12787, t12866, t12910, t13392, t15936, t17534, t17694, t17729, t17742, t20921, t21035, t21040, t3626, t3718, t3720, t5046, t5330, t5343, t59358, t59360, t59492, t69848);
        let t72014 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3769(t1222, t20293, t57484, t17735, t70646, t17423, t21014, t17708, t59498, t1042, t17505, t17584, t17589, t17739, t17750, t17800, t20795, t21093, t3368, t3372, t3720, t44551, t5384, t58803, t59379, t59386, t59391, t71440);
        let (t72044, t72049) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3770(t12916, t21041, t3718, t1214, t20957, t13396, t17391, t17396, t17429, t17534, t17729, t17736, t17737, t20858, t20956, t21035, t21173, t3367, t3626, t3720, t4181, t44500, t44624, t5245, t5348, t56953, t59404, t59406, t59408, t59415);
    (t71824, t71839, t71854, t71867, t71905, t71936, t71940, t71945, t71981, t72014, t72044, t72049)
}
