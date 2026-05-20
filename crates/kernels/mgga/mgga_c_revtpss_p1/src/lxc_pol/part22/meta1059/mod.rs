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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3763;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3764;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3765;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3766;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3767;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3768;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3769;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3770;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1059<F: Float>(t1261: F, t1264: F, t12809: F, t12866: F, t12910: F, t16696: F, t17369: F, t17412: F, t17459: F, t17649: F, t17661: F, t17668: F, t17736: F, t20956: F, t21028: F, t21035: F, t21164: F, t21182: F, t21257: F, t247: F, t3626: F, t3720: F, t44980: F, t45371: F, t5381: F, t5397: F, t5405: F, t5406: F, t57548: F, t59062: F, t59269: F, t59330: F, t60927: F, t68251: F, t71452: F, t20272: F, t3634: F, t3584: F, t6573: F, t12916: F, t20951: F, t5340: F, t17170: F, t1774: F, t17396: F, t17620: F, t1250: F, t12800: F, t16729: F, t17344: F, t17351: F, t17467: F, t17514: F, t17693: F, t20945: F, t21153: F, t3647: F, t3718: F, t3719: F, t44521: F, t5052: F, t5333: F, t5373: F, t5391: F, t58909: F, t6679: F, t68391: F, t71061: F, t17472: F, t1222: F, t17471: F, t20266: F, t20770: F, t56756: F, t1214: F, t12839: F, t17352: F, t17475: F, t17479: F, t17643: F, t21213: F, t3701: F, t372: F, t44510: F, t44769: F, t5312: F, t59320: F, t59336: F, t6690: F, t68285: F, t68290: F, t68340: F, t70932: F, t73: F, t17729: F, t20922: F, t44425: F, t17617: F, t6658: F, t697: F, t6662: F, t12268: F, t12787: F, t15936: F, t17580: F, t17625: F, t17730: F, t20317: F, t5354: F, t56953: F, t57147: F, t59162: F, t59338: F, t59349: F, t59351: F, t59353: F, t3588: F, t6587: F, t20801: F, t20805: F, t5331: F, t12784: F, t21090: F, t13392: F, t17534: F, t17694: F, t17742: F, t20921: F, t21040: F, t5046: F, t5330: F, t5343: F, t59358: F, t59360: F, t59492: F, t69848: F, t20293: F, t57484: F, t17735: F, t70646: F, t17423: F, t21014: F, t17708: F, t59498: F, t1042: F, t17505: F, t17584: F, t17589: F, t17739: F, t17750: F, t17800: F, t20795: F, t21093: F, t3368: F, t3372: F, t44551: F, t5384: F, t58803: F, t59379: F, t59386: F, t59391: F, t71440: F, t21041: F, t20957: F, t13396: F, t17391: F, t17429: F, t17737: F, t20858: F, t21173: F, t3367: F, t4181: F, t44500: F, t44624: F, t5245: F, t5348: F, t59404: F, t59406: F, t59408: F, t59415: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t71824 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3763::<F>(t1261, t1264, t12809, t12866, t12910, t16696, t17369, t17412, t17459, t17649, t17661, t17668, t17736, t20956, t21028, t21035, t21164, t21182, t21257, t247, t3626, t3720, t44980, t45371, t5381, t5397, t5405, t5406, t57548, t59062, t59269, t59330, t60927, t68251, t71452);
        let (t71827, t71839, t71845, t71854, t71859) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3764::<F>(t1261, t20272, t247, t3634, t3584, t6573, t12916, t20951, t5340, t17170, t1774, t17396, t17620);
        let t71867 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3765::<F>(t1250, t1261, t1264, t12800, t16729, t17344, t17351, t17369, t17467, t17514, t17693, t20945, t21153, t247, t3647, t3718, t3719, t3720, t44521, t5052, t5333, t5373, t5391, t58909, t6679, t68391, t71061, t71827, t71839, t71845, t71854, t71859);
        let t71905 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3766::<F>(t17472, t5373, t1222, t17471, t20266, t17351, t20770, t56756, t1214, t12839, t12866, t17352, t17475, t17479, t17643, t21213, t3701, t372, t44510, t44769, t5312, t58909, t59320, t59336, t6690, t68285, t68290, t68340, t70932, t71452, t73);
        let t71936 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3767::<F>(t17729, t20922, t44425, t17396, t17617, t1222, t6658, t697, t6662, t12268, t12787, t15936, t17580, t17625, t17730, t1774, t20317, t3626, t5354, t56953, t57147, t59162, t59338, t59349, t59351, t59353);
        let (t71940, t71945, t71981) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3768::<F>(t3588, t6587, t6573, t12916, t20801, t5340, t20805, t5331, t12784, t21090, t1250, t12787, t12866, t12910, t13392, t15936, t17534, t17694, t17729, t17742, t20921, t21035, t21040, t3626, t3718, t3720, t5046, t5330, t5343, t59358, t59360, t59492, t69848);
        let t72014 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3769::<F>(t1222, t20293, t57484, t17735, t70646, t17423, t21014, t17708, t59498, t1042, t17505, t17584, t17589, t17739, t17750, t17800, t20795, t21093, t3368, t3372, t3720, t44551, t5384, t58803, t59379, t59386, t59391, t71440);
        let (t72044, t72049) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3770::<F>(t12916, t21041, t3718, t1214, t20957, t13396, t17391, t17396, t17429, t17534, t17729, t17736, t17737, t20858, t20956, t21035, t21173, t3367, t3626, t3720, t4181, t44500, t44624, t5245, t5348, t56953, t59404, t59406, t59408, t59415);
    (t71824, t71839, t71854, t71867, t71905, t71936, t71940, t71945, t71981, t72014, t72044, t72049)
}
