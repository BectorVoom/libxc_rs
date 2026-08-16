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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta855(t1210: f64, t1214: f64, t12600: f64, t12603: f64, t12606: f64, t12666: f64, t12690: f64, t12696: f64, t1274: f64, t1277: f64, t13177: f64, t1775: f64, t17963: f64, t17975: f64, t17986: f64, t17987: f64, t17992: f64, t17995: f64, t18037: f64, t18047: f64, t18065: f64, t1813: f64, t1829: f64, t225: f64, t3556: f64, t3561: f64, t3569: f64, t3576: f64, t3791: f64, t45433: f64, t45545: f64, t45575: f64, t460: f64, t494: f64, t5231: f64, t5237: f64, t5246: f64, t5417: f64, t5498: f64, t56707: f64, t59453: f64, t59464: f64, t59510: f64, t59544: f64, t59579: f64, t59611: f64, t59649: f64, t59689: f64, t59724: f64, t59762: f64, t59797: f64, t59833: f64, t59877: f64, t59916: f64, t59951: f64, t59983: f64, t60022: f64, t60058: f64, t1209: f64, t17807: f64, t3727: f64, t5219: f64, t1204: f64, t1215: f64, t12599: f64, t12621: f64, t12622: f64, t12633: f64, t12641: f64, t12647: f64, t12650: f64, t12651: f64, t12673: f64, t13166: f64, t17973: f64, t17974: f64, t17979: f64, t18043: f64, t18059: f64, t18070: f64, t18073: f64, t18109: f64, t18114: f64, t1828: f64, t3572: f64, t3575: f64, t3585: f64, t3732: f64, t3736: f64, t5251: f64, t5497: f64, t56327: f64, t1300: f64, t198: f64, t336: f64, t56390: f64, t56484: f64, t56534: f64, t56593: f64, t56642: f64, t56687: f64, t57794: f64, t57799: f64, t57802: f64, t57805: f64, t57808: f64, t57810: f64, t57812: f64, t57814: f64, t57816: f64, t57820: f64, t1298: f64, t3794: f64, t18134: f64, t5023: f64, t57822: f64, t57825: f64, t57827: f64, t57829: f64, t57831: f64, t57833: f64, t57835: f64, t57837: f64, t57840: f64, t57842: f64, t12584: f64, t12587: f64, t1832: f64, t3798: f64, t44126: f64, t5501: f64, t57846: f64, t57849: f64, t57851: f64, t57853: f64, t57856: f64, t57860: f64, t57863: f64, t57907: f64, t57911: f64, t58322: f64, t58325: f64, t58327: f64, t58330: f64, t58333: f64, t58658: f64, t58660: f64, t58662: f64, t58664: f64, t58669: f64, t58671: f64, t58341: f64, t58344: f64, t58462: f64, t58464: f64, t58468: f64, t58472: f64, t58475: f64, t58675: f64, t58678: f64, t58683: f64, t58685: f64, t13190: f64, t5505: f64, t58477: f64, t58479: f64, t58481: f64, t58591: f64, t58688: f64, t58690: f64, t58692: f64, t58695: f64, t58700: f64, t58703: f64, t18123: f64, t18128: f64, t3801: f64, t58598: f64, t58707: f64, t58711: f64, t58713: f64, t58715: f64, t58718: f64, t58720: f64, t58722: f64, t58726: f64, t33: f64, t265: f64, t502: f64, t51814: f64, t56291: f64, t10326: f64, t11095: f64, t1113: f64, t1304: f64, t13196: f64, t13312: f64, t1469: f64, t15083: f64, t1587: f64, t1711: f64, t18140: f64, t1837: f64, t2258: f64, t3351: f64, t3805: f64, t4186: f64, t4560: f64, t49889: f64, t504: f64, t51827: f64, t51829: f64, t51831: f64, t51833: f64, t51835: f64, t5509: f64, t57: f64, t606: f64, t9357: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t10416: f64, t118: f64, t13207: f64, t13521: f64, t13532: f64, t13540: f64, t1502: f64, t1519: f64, t18153: f64, t18163: f64, t2322: f64, t3813: f64, t4246: f64, t4254: f64, t4257: f64, t4292: f64, t46126: f64, t49851: f64, t49856: f64, t56137: f64, t651: f64, t670: f64, t2327: f64, t4245: f64, t10194: f64, t10260: f64, t10263: f64, t10415: f64, t1310: f64, t13435: f64, t13514: f64, t13544: f64, t1843: f64, t2320: f64, t2328: f64, t2371: f64, t3821: f64, t4248: f64, t4293: f64, t508: f64, t5517: f64, t5787: f64, t45928: f64, t45934: f64, t45938: f64, t45945: f64, t45949: f64, t2246: f64, t4171: f64, t10308: f64, t1466: f64, t13267: f64, t602: f64, t10355: f64, t10356: f64, t10368: f64, t10373: f64, t13299: f64, t13302: f64, t13303: f64, t13306: f64, t1480: f64, t2251: f64, t2270: f64, t2275: f64, t4201: f64, t4202: f64, t44: f64, t46065: f64, t46074: f64, t56: f64, t614: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t60068 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3235(t1210, t1214, t12600, t12603, t12606, t12666, t12690, t12696, t1274, t1277, t13177, t1775, t17963, t17975, t17986, t17987, t17992, t17995, t18037, t18047, t18065, t1813, t1829, t225, t3556, t3561, t3569, t3576, t3791, t45433, t45545, t45575, t460, t494, t5231, t5237, t5246, t5417, t5498, t56707, t59453, t59464, t59510, t59544, t59579, t59611, t59649, t59689, t59724, t59762, t59797, t59833, t59877, t59916, t59951, t59983, t60022, t60058);
        let t60117 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3236(t1209, t17807, t3727, t5219, t1204, t1210, t1215, t12599, t12621, t12622, t12633, t12641, t12647, t12650, t12651, t12673, t1277, t13166, t17973, t17974, t17979, t17986, t18037, t18043, t18059, t18070, t18073, t18109, t18114, t1828, t3556, t3572, t3575, t3585, t3732, t3736, t5251, t5417, t5497, t5498, t56327);
        let t60124 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3237(t1300, t198, t336, t56390, t56484, t56534, t56593, t56642, t56687, t57794, t57799, t57802, t57805, t57808, t57810, t57812, t57814, t57816, t57820, t60068, t60117);
        let t60130 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3238(t1298, t3794, t18134, t5023, t57822, t57825, t57827, t57829, t57831, t57833, t57835, t57837, t57840, t57842);
        let t60139 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3239(t12584, t12587, t1832, t3798, t44126, t5023, t5501, t57846, t57849, t57851, t57853, t57856, t57860, t57863, t57907, t57911);
        let (t60142, t60143) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3240(t58322, t58325, t58327, t58330, t58333, t58658, t58660, t58662, t58664, t58669, t58671, t58341, t58344, t58462, t58464, t58468, t58472, t58475, t58675, t58678, t58683, t58685);
        let t60147 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3241(t13190, t5023, t5505, t58477, t58479, t58481, t58591, t58688, t58690, t58692, t58695, t58700, t58703);
        let t60155 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3242(t1298, t18123, t18128, t3794, t3801, t5023, t58598, t58707, t58711, t58713, t58715, t58718, t58720, t58722, t58726);
        let t60177 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3243(t33, t265, t502, t51814, t56291, t60124, t60130, t60139, t60142, t60143, t60147, t60155, t10326, t11095, t1113, t1304, t13196, t13312, t1469, t15083, t1587, t1711, t18140, t1837, t2258, t3351, t3805, t4186, t4560, t49889, t504, t51827, t51829, t51831, t51833, t51835, t5509, t57, t606, t9357, dens_threshold, rho1, zeta_threshold);
        let t60183 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3244(t10416, t118, t13207, t13521, t13532, t13540, t1502, t1519, t18153, t18163, t2322, t3813, t4246, t4254, t4257, t4292, t46126, t49851, t49856, t56137, t60177, t651, t670);
        let (t60206, t60213) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3245(t2327, t4245, t10194, t10260, t10263, t10415, t1310, t13435, t13514, t13544, t18163, t1843, t2320, t2322, t2328, t2371, t3821, t4248, t4293, t508, t5517, t5787, t651);
        let (t60214, t60215, t60216, t60217, t60218, t60221, t60224, t60248, t60297) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3246(t45928, t45934, t45938, t45945, t45949, t2246, t4171, t10308, t1466, t13267, t602, t10326, t10355, t10356, t10368, t10373, t13299, t13302, t13303, t13306, t13312, t1469, t1480, t2251, t2258, t2270, t2275, t4186, t4201, t4202, t44, t46065, t46074, t56, t606, t614);
    (t60183, t60206, t60213, t60214, t60215, t60216, t60217, t60218, t60221, t60224, t60248, t60297)
}
