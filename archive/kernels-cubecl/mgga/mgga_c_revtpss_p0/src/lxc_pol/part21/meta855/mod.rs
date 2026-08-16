//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta855 (260520-c91 hierarchical CSE).
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
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3235;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3236;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3237;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3238;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3239;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3240;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3241;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3242;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3243;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3244;
use chunk10::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3245;
use chunk11::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3246;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta855<F: Float>(t1210: F, t1214: F, t12600: F, t12603: F, t12606: F, t12666: F, t12690: F, t12696: F, t1274: F, t1277: F, t13177: F, t1775: F, t17963: F, t17975: F, t17986: F, t17987: F, t17992: F, t17995: F, t18037: F, t18047: F, t18065: F, t1813: F, t1829: F, t225: F, t3556: F, t3561: F, t3569: F, t3576: F, t3791: F, t45433: F, t45545: F, t45575: F, t460: F, t494: F, t5231: F, t5237: F, t5246: F, t5417: F, t5498: F, t56707: F, t59453: F, t59464: F, t59510: F, t59544: F, t59579: F, t59611: F, t59649: F, t59689: F, t59724: F, t59762: F, t59797: F, t59833: F, t59877: F, t59916: F, t59951: F, t59983: F, t60022: F, t60058: F, t1209: F, t17807: F, t3727: F, t5219: F, t1204: F, t1215: F, t12599: F, t12621: F, t12622: F, t12633: F, t12641: F, t12647: F, t12650: F, t12651: F, t12673: F, t13166: F, t17973: F, t17974: F, t17979: F, t18043: F, t18059: F, t18070: F, t18073: F, t18109: F, t18114: F, t1828: F, t3572: F, t3575: F, t3585: F, t3732: F, t3736: F, t5251: F, t5497: F, t56327: F, t1300: F, t198: F, t336: F, t56390: F, t56484: F, t56534: F, t56593: F, t56642: F, t56687: F, t57794: F, t57799: F, t57802: F, t57805: F, t57808: F, t57810: F, t57812: F, t57814: F, t57816: F, t57820: F, t1298: F, t3794: F, t18134: F, t5023: F, t57822: F, t57825: F, t57827: F, t57829: F, t57831: F, t57833: F, t57835: F, t57837: F, t57840: F, t57842: F, t12584: F, t12587: F, t1832: F, t3798: F, t44126: F, t5501: F, t57846: F, t57849: F, t57851: F, t57853: F, t57856: F, t57860: F, t57863: F, t57907: F, t57911: F, t58322: F, t58325: F, t58327: F, t58330: F, t58333: F, t58658: F, t58660: F, t58662: F, t58664: F, t58669: F, t58671: F, t58341: F, t58344: F, t58462: F, t58464: F, t58468: F, t58472: F, t58475: F, t58675: F, t58678: F, t58683: F, t58685: F, t13190: F, t5505: F, t58477: F, t58479: F, t58481: F, t58591: F, t58688: F, t58690: F, t58692: F, t58695: F, t58700: F, t58703: F, t18123: F, t18128: F, t3801: F, t58598: F, t58707: F, t58711: F, t58713: F, t58715: F, t58718: F, t58720: F, t58722: F, t58726: F, t33: F, t265: F, t502: F, t51814: F, t56291: F, t10326: F, t11095: F, t1113: F, t1304: F, t13196: F, t13312: F, t1469: F, t15083: F, t1587: F, t1711: F, t18140: F, t1837: F, t2258: F, t3351: F, t3805: F, t4186: F, t4560: F, t49889: F, t504: F, t51827: F, t51829: F, t51831: F, t51833: F, t51835: F, t5509: F, t57: F, t606: F, t9357: F, dens_threshold: F, rho1: F, zeta_threshold: F, t10416: F, t118: F, t13207: F, t13521: F, t13532: F, t13540: F, t1502: F, t1519: F, t18153: F, t18163: F, t2322: F, t3813: F, t4246: F, t4254: F, t4257: F, t4292: F, t46126: F, t49851: F, t49856: F, t56137: F, t651: F, t670: F, t2327: F, t4245: F, t10194: F, t10260: F, t10263: F, t10415: F, t1310: F, t13435: F, t13514: F, t13544: F, t1843: F, t2320: F, t2328: F, t2371: F, t3821: F, t4248: F, t4293: F, t508: F, t5517: F, t5787: F, t45928: F, t45934: F, t45938: F, t45945: F, t45949: F, t2246: F, t4171: F, t10308: F, t1466: F, t13267: F, t602: F, t10355: F, t10356: F, t10368: F, t10373: F, t13299: F, t13302: F, t13303: F, t13306: F, t1480: F, t2251: F, t2270: F, t2275: F, t4201: F, t4202: F, t44: F, t46065: F, t46074: F, t56: F, t614: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t60068 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3235::<F>(t1210, t1214, t12600, t12603, t12606, t12666, t12690, t12696, t1274, t1277, t13177, t1775, t17963, t17975, t17986, t17987, t17992, t17995, t18037, t18047, t18065, t1813, t1829, t225, t3556, t3561, t3569, t3576, t3791, t45433, t45545, t45575, t460, t494, t5231, t5237, t5246, t5417, t5498, t56707, t59453, t59464, t59510, t59544, t59579, t59611, t59649, t59689, t59724, t59762, t59797, t59833, t59877, t59916, t59951, t59983, t60022, t60058);
        let t60117 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3236::<F>(t1209, t17807, t3727, t5219, t1204, t1210, t1215, t12599, t12621, t12622, t12633, t12641, t12647, t12650, t12651, t12673, t1277, t13166, t17973, t17974, t17979, t17986, t18037, t18043, t18059, t18070, t18073, t18109, t18114, t1828, t3556, t3572, t3575, t3585, t3732, t3736, t5251, t5417, t5497, t5498, t56327);
        let t60124 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3237::<F>(t1300, t198, t336, t56390, t56484, t56534, t56593, t56642, t56687, t57794, t57799, t57802, t57805, t57808, t57810, t57812, t57814, t57816, t57820, t60068, t60117);
        let t60130 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3238::<F>(t1298, t3794, t18134, t5023, t57822, t57825, t57827, t57829, t57831, t57833, t57835, t57837, t57840, t57842);
        let t60139 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3239::<F>(t12584, t12587, t1832, t3798, t44126, t5023, t5501, t57846, t57849, t57851, t57853, t57856, t57860, t57863, t57907, t57911);
        let (t60142, t60143) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3240::<F>(t58322, t58325, t58327, t58330, t58333, t58658, t58660, t58662, t58664, t58669, t58671, t58341, t58344, t58462, t58464, t58468, t58472, t58475, t58675, t58678, t58683, t58685);
        let t60147 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3241::<F>(t13190, t5023, t5505, t58477, t58479, t58481, t58591, t58688, t58690, t58692, t58695, t58700, t58703);
        let t60155 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3242::<F>(t1298, t18123, t18128, t3794, t3801, t5023, t58598, t58707, t58711, t58713, t58715, t58718, t58720, t58722, t58726);
        let t60177 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3243::<F>(t33, t265, t502, t51814, t56291, t60124, t60130, t60139, t60142, t60143, t60147, t60155, t10326, t11095, t1113, t1304, t13196, t13312, t1469, t15083, t1587, t1711, t18140, t1837, t2258, t3351, t3805, t4186, t4560, t49889, t504, t51827, t51829, t51831, t51833, t51835, t5509, t57, t606, t9357, dens_threshold, rho1, zeta_threshold);
        let t60183 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3244::<F>(t10416, t118, t13207, t13521, t13532, t13540, t1502, t1519, t18153, t18163, t2322, t3813, t4246, t4254, t4257, t4292, t46126, t49851, t49856, t56137, t60177, t651, t670);
        let (t60206, t60213) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3245::<F>(t2327, t4245, t10194, t10260, t10263, t10415, t1310, t13435, t13514, t13544, t18163, t1843, t2320, t2322, t2328, t2371, t3821, t4248, t4293, t508, t5517, t5787, t651);
        let (t60214, t60215, t60216, t60217, t60218, t60221, t60224, t60248, t60297) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3246::<F>(t45928, t45934, t45938, t45945, t45949, t2246, t4171, t10308, t1466, t13267, t602, t10326, t10355, t10356, t10368, t10373, t13299, t13302, t13303, t13306, t13312, t1469, t1480, t2251, t2258, t2270, t2275, t4186, t4201, t4202, t44, t46065, t46074, t56, t606, t614);
    (t60183, t60206, t60213, t60214, t60215, t60216, t60217, t60218, t60221, t60224, t60248, t60297)
}
