//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta643 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2249;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2250;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2251;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2252;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2253;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2254;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2255;
use chunk7::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2256;
use chunk8::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2257;
use chunk9::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2258;
use chunk10::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2259;
use chunk11::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2260;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta643<F: Float>(t12627: F, t1276: F, t7635: F, t1203: F, t1774: F, t1248: F, t1770: F, t7627: F, t104606: F, t1214: F, t1287: F, t1295: F, t2151: F, t26895: F, t26969: F, t27029: F, t29118: F, t29136: F, t29163: F, t29195: F, t29213: F, t29278: F, t29287: F, t29304: F, t3585: F, t3588: F, t3738: F, t3769: F, t5231: F, t5464: F, t7643: F, t8201: F, t8209: F, t96866: F, t96874: F, t96979: F, t96986: F, t97082: F, t97343: F, t105202: F, t7642: F, t104480: F, t1243: F, t2149: F, t104483: F, t1294: F, t17330: F, t1775: F, t17951: F, t17999: F, t2142: F, t26922: F, t26945: F, t26994: F, t26999: F, t29141: F, t29204: F, t29207: F, t29220: F, t29224: F, t29268: F, t29271: F, t29282: F, t29292: F, t29300: F, t3551: F, t3791: F, t5246: F, t7602: F, t7636: F, t7637: F, t7645: F, t7651: F, t7652: F, t8208: F, t96927: F, t96954: F, t97370: F, t1811: F, t8945: F, t3596: F, t104473: F, t105122: F, t17807: F, t17855: F, t17883: F, t18047: F, t2148: F, t2152: F, t26897: F, t26941: F, t26979: F, t26991: F, t29111: F, t29119: F, t29158: F, t29160: F, t29199: F, t29200: F, t29201: F, t29227: F, t29275: F, t3739: F, t5480: F, t7648: F, t8217: F, t97363: F, t13181: F, t1209: F, t26948: F, t29135: F, t5219: F, t1215: F, t17964: F, t26949: F, t26951: F, t26983: F, t27025: F, t29178: F, t29186: F, t29297: F, t29308: F, t3568: F, t5428: F, t72861: F, t7632: F, t7639: F, t8190: F, t97402: F, t97422: F, t1769: F, t73: F, t29109: F, t18084: F, t1829: F, t26889: F, t26928: F, t26937: F, t26984: F, t29159: F, t29187: F, t29279: F, t29283: F, t5458: F, t96933: F, t97066: F, t3566: F, t460: F, t5251: F, t3601: F, t8197: F, t17944: F, t26891: F, t26918: F, t26996: F, t29132: F, t29174: F, t29247: F, t29251: F, t3569: F, t3783: F, t3790: F, t7666: F, t96938: F, t97041: F, t97318: F, t26921: F, t8205: F, t2143: F, t17306: F, t3556: F, t104490: F, t104504: F, t104510: F, t17170: F, t1794: F, t17975: F, t26924: F, t26931: F, t29216: F, t29217: F, t3584: F, t5284: F, t5352: F, t5457: F, t5479: F, t7659: F, t7660: F, t96928: F, t96953: F, t97067: F, t97095: F, t97304: F, t97308: F, t97397: F, t12640: F, t18043: F, t18109: F, t1828: F, t26884: F, t26901: F, t27015: F, t27020: F, t29129: F, t29148: F, t29175: F, t29237: F, t29272: F, t29293: F, t5429: F, t96966: F, t97313: F, t97348: F, t97358: F, t29313: F, t3801: F, t12587: F, t8220: F, t104509: F, t104560: F, t104601: F, t105057: F, t105107: F, t105155: F, t105206: F, t105258: F, t1298: F, t1300: F, t18123: F, t1832: F, t198: F, t27037: F, t27041: F, t29317: F, t29322: F, t336: F, t3794: F, t3798: F, t5023: F, t5501: F, t73262: F, t7673: F, t97487: F, t97491: F, t97498: F, t33: F, t265: F, t502: F, t100927: F, t101107: F, t13312: F, t1469: F, t2159: F, t2258: F, t27048: F, t29329: F, t4186: F, t57: F, t606: F, t7677: F, t8227: F, dens_threshold: F, rho1: F, zeta_threshold: F, t104450: F, t118: F, t13537: F, t29432: F, t4293: F, t7586: F, t98455: F, t98458: F, t98461: F, t98463: F, t98467: F, t98472: F, t98474: F, t98477: F, t98483: F, t98486: F, t98489: F, t98491: F, t98494: F, t98499: F, t98501: F, t98522: F, t13426: F, t1453: F, t2320: F, t27076: F, t29337: F, t29437: F, t4248: F, t649: F, t7591: F, t8233: F, t98525: F, t98528: F, t98530: F, t98532: F, t98534: F, t98537: F, t98539: F, t98541: F, t98544: F, t98546: F, t98549: F, t98553: F, t98555: F, t98557: F) -> (F, F) {
        let t105310 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2249::<F>(t12627, t1276, t7635, t1203, t1774, t1248, t1770, t7627, t104606, t1214, t1287, t1295, t2151, t26895, t26969, t27029, t29118, t29136, t29163, t29195, t29213, t29278, t29287, t29304, t3585, t3588, t3738, t3769, t5231, t5464, t7643, t8201, t8209, t96866, t96874, t96979, t96986, t97082, t97343);
        let t105358 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2250::<F>(t105202, t7642, t104480, t1243, t2149, t104483, t1214, t1248, t1287, t1294, t17330, t1775, t17951, t17999, t2142, t26922, t26945, t26969, t26994, t26999, t29141, t29204, t29207, t29220, t29224, t29268, t29271, t29282, t29292, t29300, t3551, t3585, t3791, t5246, t7602, t7636, t7637, t7645, t7651, t7652, t8208, t96927, t96954, t97343, t97370);
        let t105402 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2251::<F>(t1811, t7642, t8945, t104480, t2149, t3596, t104473, t104483, t105122, t1214, t17807, t17855, t17883, t18047, t2148, t2152, t26895, t26897, t26941, t26979, t26991, t29111, t29119, t29158, t29160, t29199, t29200, t29201, t29227, t29275, t29282, t3739, t5480, t7602, t7643, t7648, t7652, t8217, t97363);
        let t105457 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2252::<F>(t13181, t7635, t1209, t7642, t26948, t29135, t5219, t7627, t105202, t1203, t1214, t1215, t1287, t1294, t1775, t17964, t1811, t2151, t2152, t26922, t26949, t26951, t26983, t27025, t29118, t29178, t29186, t29297, t29308, t3568, t3588, t5428, t72861, t7632, t7636, t7637, t7639, t7643, t7652, t8190, t8208, t97402, t97422);
        let t105504 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2253::<F>(t1214, t1769, t1248, t73, t8190, t1209, t29109, t1215, t1287, t1294, t18084, t1829, t2151, t26889, t26895, t26928, t26937, t26979, t26984, t26994, t27025, t29159, t29186, t29187, t29224, t29271, t29275, t29279, t29283, t5458, t7602, t7637, t7643, t7652, t8217, t96933, t97066);
        let t105553 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2254::<F>(t29135, t3566, t8190, t29109, t460, t5251, t8945, t3601, t8197, t1248, t1287, t1294, t1295, t17944, t1829, t26889, t26891, t26918, t26969, t26996, t29132, t29158, t29174, t29178, t29187, t29204, t29247, t29251, t3569, t3588, t3769, t3783, t3790, t7643, t7651, t7652, t7666, t8201, t96938, t96979, t97041, t97318);
        let t105613 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2255::<F>(t26921, t8205, t2143, t3566, t17306, t2142, t3556, t8945, t104490, t104504, t104510, t1287, t17170, t1794, t17975, t26889, t26924, t26931, t26969, t26994, t29158, t29159, t29195, t29216, t29217, t3551, t3569, t3584, t3790, t5284, t5352, t5457, t5479, t7636, t7637, t7651, t7659, t7660, t8190, t8197, t8208, t96928, t96953, t97067, t97095, t97304, t97308, t97318, t97397);
        let t105657 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2256::<F>(t12640, t7635, t1248, t1294, t18043, t18109, t1828, t26884, t26901, t26937, t26949, t26979, t27015, t27020, t27025, t29129, t29136, t29148, t29175, t29195, t29237, t29268, t29272, t29293, t3568, t5429, t5464, t7602, t7632, t7637, t7651, t7652, t8201, t8208, t8209, t96954, t96966, t97313, t97348, t97358);
        let t105696 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2257::<F>(t29313, t3801, t12587, t8220, t104509, t104560, t104601, t105057, t105107, t105155, t105206, t105258, t105310, t105358, t105402, t105457, t105504, t105553, t105613, t105657, t1298, t1300, t18123, t1832, t198, t27037, t27041, t29317, t29322, t336, t3794, t3798, t5023, t5501, t73262, t7673, t97487, t97491, t97498);
        let t105709 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2258::<F>(t33, t265, t502, t100927, t105696, t101107, t13312, t1469, t2159, t2258, t27048, t29329, t4186, t57, t606, t7677, t8227, dens_threshold, rho1, zeta_threshold);
        let t105712 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2259::<F>(t104450, t105709, t118, t13537, t29432, t4293, t7586, t98455, t98458, t98461, t98463, t98467, t98472, t98474, t98477, t98483, t98486, t98489, t98491, t98494, t98499, t98501, t98522);
        let t105724 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2260::<F>(t13426, t1453, t2320, t27076, t29337, t29437, t4248, t649, t7591, t8233, t98525, t98528, t98530, t98532, t98534, t98537, t98539, t98541, t98544, t98546, t98549, t98553, t98555, t98557);
    (t105712, t105724)
}
