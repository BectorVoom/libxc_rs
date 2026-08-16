//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta459 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1674;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1675;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1676;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1677;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1678;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1679;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1680;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1681;
use chunk8::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1682;
use chunk9::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1683;
use chunk10::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1684;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta459(t473: f64, t6695: f64, t1214: f64, t3759: f64, t6587: f64, t1280: f64, t21082: f64, t21471: f64, t5284: f64, t5332: f64, t1269: f64, t1287: f64, t6622: f64, t6573: f64, t1234: f64, t12756: f64, t1285: f64, t1291: f64, t12966: f64, t12987: f64, t1770: f64, t1825: f64, t21333: f64, t21518: f64, t21521: f64, t21524: f64, t21527: f64, t21535: f64, t21538: f64, t3670: f64, t460: f64, t490: f64, t5216: f64, t5478: f64, t5494: f64, t6564: f64, t6714: f64, t5477: f64, t1248: f64, t17847: f64, t20956: f64, t17854: f64, t20721: f64, t5464: f64, t20856: f64, t1794: f64, t5412: f64, t5245: f64, t5486: f64, t1204: f64, t12717: f64, t1281: f64, t17192: f64, t17289: f64, t17846: f64, t17853: f64, t1818: f64, t20850: f64, t3666: f64, t3746: f64, t5326: f64, t5436: f64, t5449: f64, t5452: f64, t5459: f64, t5463: f64, t5474: f64, t5481: f64, t6723: f64, t6735: f64, t6741: f64, t21464: f64, t21516: f64, t1277: f64, t20849: f64, t487: f64, t1211: f64, t1210: f64, t1215: f64, t12633: f64, t12641: f64, t1271: f64, t1274: f64, t18059: f64, t1813: f64, t21394: f64, t21408: f64, t3732: f64, t495: f64, t5220: f64, t5231: f64, t5237: f64, t5251: f64, t5417: f64, t5423: f64, t5429: f64, t6574: f64, t6703: f64, t20735: f64, t21357: f64, t21393: f64, t12587: f64, t6752: f64, t1298: f64, t1300: f64, t198: f64, t20571: f64, t20573: f64, t20576: f64, t20579: f64, t20582: f64, t20631: f64, t20633: f64, t20635: f64, t20637: f64, t20639: f64, t20643: f64, t20647: f64, t20650: f64, t20654: f64, t20692: f64, t20889: f64, t20894: f64, t20898: f64, t336: f64, t5023: f64, t33: f64, t265: f64, t502: f64, t18884: f64, t20691: f64, t1113: f64, t1304: f64, t1469: f64, t1711: f64, t18281: f64, t1837: f64, t18892: f64, t20256: f64, t4186: f64, t4560: f64, t504: f64, t5509: f64, t57: f64, t5825: f64, t606: f64, t6084: f64, t6416: f64, t6757: f64, t895: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t20248: f64, t118: f64, t1310: f64, t13426: f64, t1502: f64, t1519: f64, t18220: f64, t18227: f64, t18232: f64, t18235: f64, t18242: f64, t18245: f64, t1843: f64, t2322: f64, t4246: f64, t4248: f64, t4254: f64, t4257: f64, t508: f64, t5517: f64, t5877: f64, t5884: f64, t5921: f64, t651: f64, t671: f64, t10275: f64, t10278: f64, t10284: f64, t10287: f64, t10295: f64, t13261: f64, t13262: f64, t13263: f64, t13264: f64, t13265: f64, t13266: f64, t5812: f64, t602: f64, t5816: f64, t644: f64, t1497: f64, t4241: f64, t5872: f64, t70: f64, t72: f64, t1927: f64, t5819: f64, t627: f64, t19680: f64, t36: f64, t5826: f64, t1486: f64, t4181: f64, t4187: f64, t1470: f64, t4217: f64, t1494: f64, t4182: f64, t5820: f64, t5827: f64, t5830: f64, t641: f64, t85: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21542, t21551, t21554, t21558, t21562) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1674(t473, t6695, t1214, t3759, t6587, t1280, t21082, t21471, t5284, t5332, t1269, t1287, t6622);
        let t21568 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1675(t3759, t6573, t1234, t12756, t1285, t1291, t12966, t12987, t1770, t1825, t21333, t21518, t21521, t21524, t21527, t21535, t21538, t21542, t21551, t21554, t21558, t21562, t3670, t460, t490, t5216, t5478, t5494, t6564, t6714);
        let (t21579, t21583, t21587, t21592, t21596, t21599) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1676(t1770, t5477, t1248, t17847, t20956, t17854, t1280, t20721, t5284, t5464, t5332, t1287, t20856);
        let t21615 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1677(t1287, t1794, t5412, t5245, t5486, t1204, t1234, t12717, t1281, t1285, t17192, t17289, t17846, t17853, t1818, t20850, t21579, t21583, t21587, t21592, t21596, t21599, t3666, t3670, t3746, t5326, t5436, t5449, t5452, t5459, t5463, t5474, t5481, t6723, t6735, t6741);
        let t21633 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1678(t21464, t21516, t21568, t21615, t1277, t20849, t487, t1211, t21082, t1210, t1215, t12633, t12641, t1271, t1274, t18059, t1813, t21333, t21394, t21408, t3732, t495, t5216, t5220, t5231, t5237, t5251, t5417, t5423, t5429, t6564, t6574, t6703);
        let t21643 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1679(t20735, t21357, t21393, t21633, t12587, t6752, t1298, t1300, t198, t20571, t20573, t20576, t20579, t20582, t20631, t20633, t20635, t20637, t20639, t20643, t20647, t20650, t20654, t20692, t20889, t20894, t20898, t336, t5023);
        let t21657 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1680(t33, t265, t502, t18884, t20691, t21643, t1113, t1304, t1469, t1711, t18281, t1837, t18892, t20256, t4186, t4560, t504, t5509, t57, t5825, t606, t6084, t6416, t6757, t895, dens_threshold, rho1, zeta_threshold);
        let (t21658, t21660) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1681(t20248, t21657, t118, t1310, t13426, t1502, t1519, t18220, t18227, t18232, t18235, t18242, t18245, t1843, t2322, t4246, t4248, t4254, t4257, t508, t5517, t5877, t5884, t5921, t651, t671);
        let (t21661, t21663) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1682(t10275, t10278, t10284, t10287, t10295, t13261, t13262, t13263, t13264, t13265, t13266, t5812, t602);
        let (t21674, t21677, t21682, t21686, t21687, t21690) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1683(t5816, t644, t1497, t4241, t5872, t1469, t70, t72, t1927, t4186, t5819, t627);
        let (t21698, t21720) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1684(t19680, t70, t18281, t36, t5826, t627, t1486, t4181, t4187, t1470, t4217, t1494, t21686, t21687, t21690, t4182, t5820, t5827, t5830, t641, t85);
    (t21658, t21660, t21661, t21663, t21674, t21677, t21682, t21698, t21720)
}
