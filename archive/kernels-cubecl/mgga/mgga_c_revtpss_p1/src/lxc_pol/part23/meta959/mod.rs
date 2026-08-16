//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta959 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3221;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3222;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3223;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3224;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3225;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3226;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3227;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3228;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3229;
use chunk9::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3230;
use chunk10::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3231;
use chunk11::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3232;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta959<F: Float>(t1234: F, t1280: F, t12987: F, t17192: F, t17307: F, t17958: F, t21082: F, t21448: F, t21473: F, t21491: F, t21592: F, t24912: F, t24951: F, t3666: F, t45779: F, t5443: F, t5446: F, t5459: F, t5462: F, t5466: F, t5477: F, t5481: F, t5486: F, t59674: F, t6564: F, t69637: F, t72267: F, t72386: F, t82525: F, t1287: F, t1770: F, t17949: F, t20850: F, t20956: F, t21451: F, t21455: F, t21459: F, t21599: F, t25005: F, t3755: F, t45634: F, t45718: F, t45739: F, t5284: F, t5452: F, t59686: F, t59817: F, t60008: F, t60019: F, t6717: F, t82493: F, t1248: F, t12709: F, t12723: F, t12756: F, t1285: F, t1794: F, t1818: F, t20800: F, t21342: F, t21557: F, t24986: F, t24989: F, t3670: F, t3783: F, t5412: F, t5478: F, t5494: F, t59241: F, t59864: F, t59865: F, t6622: F, t6714: F, t70209: F, t82207: F, t82321: F, t82886: F, t84175: F, t1214: F, t12717: F, t13127: F, t13129: F, t17854: F, t1811: F, t20900: F, t21439: F, t21607: F, t45659: F, t5436: F, t5449: F, t5474: F, t5491: F, t59871: F, t59872: F, t6695: F, t82293: F, t82899: F, t83662: F, t84462: F, t12050: F, t12751: F, t16695: F, t17853: F, t17861: F, t1825: F, t21333: F, t21527: F, t21583: F, t21587: F, t24934: F, t24964: F, t5458: F, t59705: F, t6731: F, t70890: F, t72429: F, t72432: F, t82476: F, t82725: F, t83792: F, t84645: F, t1284: F, t24698: F, t1288: F, t20703: F, t21480: F, t21483: F, t21484: F, t21512: F, t21595: F, t24931: F, t25009: F, t3746: F, t3782: F, t45769: F, t45859: F, t460: F, t487: F, t489: F, t5487: F, t59749: F, t82422: F, t84203: F, t84415: F, t84457: F, t1281: F, t17864: F, t17880: F, t21507: F, t21551: F, t21565: F, t24633: F, t24948: F, t24956: F, t24999: F, t3759: F, t45385: F, t45846: F, t5326: F, t59854: F, t6738: F, t72270: F, t72435: F, t82471: F, t83108: F, t83567: F, t1210: F, t1211: F, t1274: F, t1277: F, t13182: F, t1775: F, t17973: F, t17974: F, t18037: F, t1829: F, t20710: F, t21348: F, t21365: F, t21366: F, t21394: F, t21618: F, t21621: F, t24524: F, t24525: F, t25019: F, t3732: F, t3737: F, t45427: F, t5220: F, t5225: F, t5231: F, t5237: F, t5245: F, t5251: F, t5422: F, t56327: F, t56607: F, t6574: F, t6580: F, t6702: F, t72802: F, t73137: F, t73222: F, t73236: F, t84392: F, t84425: F, t84461: F, t84506: F, t84541: F, t84570: F, t84605: F, t84641: F, t84679: F, t83107: F, t1215: F, t12633: F, t12641: F, t1271: F, t1295: F, t17986: F, t18054: F, t18087: F, t18114: F, t20741: F, t20759: F, t21389: F, t21407: F, t21408: F, t24509: F, t24906: F, t25016: F, t3561: F, t45449: F, t495: F, t5417: F, t6588: F, t6703: F, t6745: F, t72874: F, t72894: F, t72933: F, t73205: F, t83232: F, t1300: F, t198: F, t336: F, t81646: F, t81649: F, t81653: F, t81656: F, t81660: F, t82119: F, t82169: F, t82220: F, t82266: F, t82391: F, t82394: F, t82396: F, t82398: F, t84241: F, t84290: F, t84337: F, t1298: F, t1832: F, t21639: F, t24501: F, t44126: F, t5023: F, t5501: F, t73273: F, t82060: F, t82400: F, t82402: F, t82404: F, t82406: F, t82410: F, t82415: F, t82418: F, t33: F, t265: F, t502: F, t77472: F, t81153: F, t81318: F, t81350: F, t81583: F, t81615: F, t81642: F, t1113: F, t1304: F, t1469: F, t1587: F, t1711: F, t18281: F, t1837: F, t18884: F, t20256: F, t21645: F, t22671: F, t22783: F, t23436: F, t25032: F, t4186: F, t4560: F, t504: F, t5509: F, t57: F, t5825: F, t606: F, t6416: F, t6757: F, t76397: F, t77481: F, t81123: F, t895: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
        let t84710 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3221::<F>(t1234, t1280, t12987, t17192, t17307, t17958, t21082, t21448, t21473, t21491, t21592, t24912, t24951, t3666, t45779, t5443, t5446, t5459, t5462, t5466, t5477, t5481, t5486, t59674, t6564, t69637, t72267, t72386, t82525);
        let t84741 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3222::<F>(t1287, t1770, t17949, t17958, t20850, t20956, t21451, t21455, t21459, t21599, t25005, t3755, t45634, t45718, t45739, t5284, t5452, t5466, t5481, t59686, t59817, t60008, t60019, t6717, t82493);
        let t84778 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3223::<F>(t1234, t1248, t12709, t12723, t12756, t1280, t1285, t1287, t1794, t1818, t20800, t21342, t21557, t24986, t24989, t3670, t3783, t5412, t5478, t5494, t59241, t59864, t59865, t6564, t6622, t6714, t70209, t82207, t82321, t82886, t84175);
        let t84816 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3224::<F>(t1214, t1248, t12717, t12723, t1285, t1287, t13127, t13129, t17854, t1811, t20850, t20900, t21439, t21607, t24989, t3755, t45659, t5284, t5436, t5449, t5474, t5491, t59871, t59872, t6695, t82293, t82886, t82899, t83662, t84462);
        let t84851 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3225::<F>(t12050, t1214, t12751, t1287, t16695, t1770, t17853, t17854, t17861, t17949, t1825, t20956, t21333, t21527, t21583, t21587, t24934, t24964, t3666, t3755, t5284, t5458, t59705, t6717, t6731, t70890, t72429, t72432, t82476, t82725, t83792, t84645);
        let t84887 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3226::<F>(t1284, t24698, t12751, t1285, t1287, t1288, t17192, t17958, t20703, t20850, t21448, t21480, t21483, t21484, t21512, t21595, t24931, t25009, t3670, t3746, t3782, t3783, t45769, t45859, t460, t487, t489, t5486, t5487, t59749, t82422, t84203, t84415, t84457);
        let t84917 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3227::<F>(t1234, t1280, t1281, t1287, t17307, t17864, t17880, t21507, t21551, t21565, t24633, t24948, t24956, t24999, t3670, t3755, t3759, t45385, t45846, t5326, t5459, t59854, t6738, t72270, t72435, t82471, t83108, t83567);
        let t84947 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3228::<F>(t1210, t1211, t1214, t1274, t1277, t13182, t1775, t17973, t17974, t18037, t1829, t20710, t21348, t21365, t21366, t21394, t21618, t21621, t24524, t24525, t25019, t3732, t3737, t45427, t5220, t5225, t5231, t5237, t5245, t5251, t5422, t56327, t56607, t6574, t6580, t6702, t72802, t73137, t73222, t73236, t84175, t84392, t84425, t84461, t84506, t84541, t84570, t84605, t84641, t84679, t84710, t84741, t84778, t84816, t84851, t84887, t84917);
        let t84992 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3229::<F>(t24698, t487, t83107, t1215, t12633, t12641, t1271, t1295, t1775, t17986, t18054, t18087, t18114, t1829, t20741, t20759, t21389, t21407, t21408, t24509, t24906, t25016, t25019, t3561, t3732, t45449, t495, t5231, t5251, t5417, t6588, t6703, t6745, t72874, t72894, t72933, t73205, t83232);
        let t84999 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3230::<F>(t1300, t198, t336, t81646, t81649, t81653, t81656, t81660, t82119, t82169, t82220, t82266, t82391, t82394, t82396, t82398, t84241, t84290, t84337, t84947, t84992);
        let t85010 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3231::<F>(t1298, t1832, t21639, t24501, t44126, t5023, t5501, t73273, t82060, t82400, t82402, t82404, t82406, t82410, t82415, t82418);
        let t85032 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3232::<F>(t33, t265, t502, t77472, t81153, t81318, t81350, t81583, t81615, t81642, t84999, t85010, t1113, t1304, t1469, t1587, t1711, t18281, t1837, t18884, t20256, t21645, t22671, t22783, t23436, t25032, t4186, t4560, t504, t5509, t57, t5825, t606, t6416, t6757, t76397, t77481, t81123, t895, dens_threshold, rho1, zeta_threshold);
    t85032
}
