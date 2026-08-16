//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta405 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1401;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1402;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1403;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1404;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1405;
use chunk5::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1406;
use chunk6::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1407;
use chunk7::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1408;
use chunk8::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1409;
use chunk9::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1410;
use chunk10::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1411;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta405<F: Float>(t473: F, t6695: F, t1214: F, t3759: F, t6587: F, t1280: F, t21082: F, t21471: F, t5284: F, t5332: F, t1269: F, t1287: F, t6622: F, t6573: F, t1234: F, t12756: F, t1285: F, t1291: F, t12966: F, t12987: F, t1770: F, t1825: F, t21333: F, t21518: F, t21521: F, t21524: F, t21527: F, t21535: F, t21538: F, t3670: F, t460: F, t490: F, t5216: F, t5478: F, t5494: F, t6564: F, t6714: F, t5477: F, t1248: F, t17847: F, t20956: F, t17854: F, t20721: F, t5464: F, t20856: F, t1794: F, t5412: F, t5245: F, t5486: F, t1204: F, t12717: F, t1281: F, t17192: F, t17289: F, t17846: F, t17853: F, t1818: F, t20850: F, t3666: F, t3746: F, t5326: F, t5436: F, t5449: F, t5452: F, t5459: F, t5463: F, t5474: F, t5481: F, t6723: F, t6735: F, t6741: F, t21464: F, t21516: F, t1277: F, t20849: F, t487: F, t1211: F, t1210: F, t1215: F, t12633: F, t12641: F, t1271: F, t1274: F, t18059: F, t1813: F, t21394: F, t21408: F, t3732: F, t495: F, t5220: F, t5231: F, t5237: F, t5251: F, t5417: F, t5423: F, t5429: F, t6574: F, t6703: F, t20735: F, t21357: F, t21393: F, t12587: F, t6752: F, t1298: F, t1300: F, t198: F, t20571: F, t20573: F, t20576: F, t20579: F, t20582: F, t20631: F, t20633: F, t20635: F, t20637: F, t20639: F, t20643: F, t20647: F, t20650: F, t20654: F, t20692: F, t20889: F, t20894: F, t20898: F, t336: F, t5023: F, t33: F, t265: F, t502: F, t18884: F, t20691: F, t1113: F, t1304: F, t1469: F, t1711: F, t18281: F, t1837: F, t18892: F, t20256: F, t4186: F, t4560: F, t504: F, t5509: F, t57: F, t5825: F, t606: F, t6084: F, t6416: F, t6757: F, t895: F, dens_threshold: F, rho1: F, zeta_threshold: F, t20248: F, t118: F, t1310: F, t13426: F, t1502: F, t1519: F, t18220: F, t18227: F, t18232: F, t18235: F, t18242: F, t18245: F, t1843: F, t2322: F, t4246: F, t4248: F, t4254: F, t4257: F, t508: F, t5517: F, t5877: F, t5884: F, t5921: F, t651: F, t671: F, t10275: F, t10278: F, t10284: F, t10287: F, t10295: F, t13261: F, t13262: F, t13263: F, t13264: F, t13265: F, t13266: F, t5812: F, t602: F, t5816: F, t644: F, t1497: F, t4241: F, t5872: F, t70: F, t72: F, t1927: F, t5819: F, t627: F, t19680: F, t36: F, t5826: F, t1486: F, t4181: F, t4187: F, t1470: F, t4217: F, t1494: F, t4182: F, t5820: F, t5827: F, t5830: F, t641: F, t85: F) -> (F, F, F, F, F, F, F, F) {
        let (t21542, t21551, t21554, t21558, t21562) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1401::<F>(t473, t6695, t1214, t3759, t6587, t1280, t21082, t21471, t5284, t5332, t1269, t1287, t6622);
        let t21568 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1402::<F>(t3759, t6573, t1234, t12756, t1285, t1291, t12966, t12987, t1770, t1825, t21333, t21518, t21521, t21524, t21527, t21535, t21538, t21542, t21551, t21554, t21558, t21562, t3670, t460, t490, t5216, t5478, t5494, t6564, t6714);
        let (t21579, t21583, t21587, t21592, t21596, t21599) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1403::<F>(t1770, t5477, t1248, t17847, t20956, t17854, t1280, t20721, t5284, t5464, t5332, t1287, t20856);
        let t21615 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1404::<F>(t1287, t1794, t5412, t5245, t5486, t1204, t1234, t12717, t1281, t1285, t17192, t17289, t17846, t17853, t1818, t20850, t21579, t21583, t21587, t21592, t21596, t21599, t3666, t3670, t3746, t5326, t5436, t5449, t5452, t5459, t5463, t5474, t5481, t6723, t6735, t6741);
        let t21633 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1405::<F>(t21464, t21516, t21568, t21615, t1277, t20849, t487, t1211, t21082, t1210, t1215, t12633, t12641, t1271, t1274, t18059, t1813, t21333, t21394, t21408, t3732, t495, t5216, t5220, t5231, t5237, t5251, t5417, t5423, t5429, t6564, t6574, t6703);
        let t21643 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1406::<F>(t20735, t21357, t21393, t21633, t12587, t6752, t1298, t1300, t198, t20571, t20573, t20576, t20579, t20582, t20631, t20633, t20635, t20637, t20639, t20643, t20647, t20650, t20654, t20692, t20889, t20894, t20898, t336, t5023);
        let t21657 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1407::<F>(t33, t265, t502, t18884, t20691, t21643, t1113, t1304, t1469, t1711, t18281, t1837, t18892, t20256, t4186, t4560, t504, t5509, t57, t5825, t606, t6084, t6416, t6757, t895, dens_threshold, rho1, zeta_threshold);
        let (t21658, t21660) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1408::<F>(t20248, t21657, t118, t1310, t13426, t1502, t1519, t18220, t18227, t18232, t18235, t18242, t18245, t1843, t2322, t4246, t4248, t4254, t4257, t508, t5517, t5877, t5884, t5921, t651, t671);
        let (t21661, t21663) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1409::<F>(t10275, t10278, t10284, t10287, t10295, t13261, t13262, t13263, t13264, t13265, t13266, t5812, t602);
        let (t21674, t21677, t21682, t21686, t21687, t21690) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1410::<F>(t5816, t644, t1497, t4241, t5872, t1469, t70, t72, t1927, t4186, t5819, t627);
        let t21720 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1411::<F>(t19680, t70, t18281, t36, t5826, t627, t1486, t4181, t4187, t1470, t4217, t1494, t21686, t21687, t21690, t4182, t5820, t5827, t5830, t641, t85);
    (t21658, t21660, t21661, t21663, t21674, t21677, t21682, t21720)
}
