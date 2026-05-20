//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta679 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2660;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2661;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2662;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2663;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2664;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta679<F: Float>(t21464: F, t21516: F, t21568: F, t21615: F, t1277: F, t20849: F, t487: F, t1211: F, t21082: F, t1210: F, t1215: F, t12633: F, t12641: F, t1271: F, t1274: F, t18059: F, t1813: F, t21333: F, t21394: F, t21408: F, t3732: F, t495: F, t5216: F, t5220: F, t5231: F, t5237: F, t5251: F, t5417: F, t5423: F, t5429: F, t6564: F, t6574: F, t6703: F, t20735: F, t21357: F, t21393: F, t12587: F, t6752: F, t1298: F, t1300: F, t198: F, t20571: F, t20573: F, t20576: F, t20579: F, t20582: F, t20631: F, t20633: F, t20635: F, t20637: F, t20639: F, t20643: F, t20647: F, t20650: F, t20654: F, t20692: F, t20889: F, t20894: F, t20898: F, t336: F, t5023: F, t33: F, t265: F, t502: F, t18884: F, t20691: F, t1113: F, t1304: F, t1469: F, t1711: F, t18281: F, t1837: F, t18892: F, t20256: F, t4186: F, t4560: F, t504: F, t5509: F, t57: F, t5825: F, t606: F, t6084: F, t6416: F, t6757: F, t895: F, dens_threshold: F, rho1: F, zeta_threshold: F, t20248: F, t118: F, t1310: F, t13426: F, t1502: F, t1519: F, t18220: F, t18227: F, t18232: F, t18235: F, t18242: F, t18245: F, t1843: F, t2322: F, t4246: F, t4248: F, t4254: F, t4257: F, t508: F, t5517: F, t5877: F, t5884: F, t5921: F, t651: F, t671: F, t10275: F, t10278: F, t10284: F, t10287: F, t10295: F, t13261: F, t13262: F, t13263: F, t13264: F, t13265: F, t13266: F, t5812: F, t602: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t21617, t21618, t21621, t21624, t21633) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2660::<F>(t21464, t21516, t21568, t21615, t1277, t20849, t487, t1211, t21082, t1210, t1215, t12633, t12641, t1271, t1274, t18059, t1813, t21333, t21394, t21408, t3732, t495, t5216, t5220, t5231, t5237, t5251, t5417, t5423, t5429, t6564, t6574, t6703);
        let (t21635, t21639, t21643) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2661::<F>(t20735, t21357, t21393, t21633, t12587, t6752, t1298, t1300, t198, t20571, t20573, t20576, t20579, t20582, t20631, t20633, t20635, t20637, t20639, t20643, t20647, t20650, t20654, t20692, t20889, t20894, t20898, t336, t5023);
        let (t21645, t21657) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2662::<F>(t33, t265, t502, t18884, t20691, t21643, t1113, t1304, t1469, t1711, t18281, t1837, t18892, t20256, t4186, t4560, t504, t5509, t57, t5825, t606, t6084, t6416, t6757, t895, dens_threshold, rho1, zeta_threshold);
        let (t21658, t21660) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2663::<F>(t20248, t21657, t118, t1310, t13426, t1502, t1519, t18220, t18227, t18232, t18235, t18242, t18245, t1843, t2322, t4246, t4248, t4254, t4257, t508, t5517, t5877, t5884, t5921, t651, t671);
        let (t21661, t21663) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2664::<F>(t10275, t10278, t10284, t10287, t10295, t13261, t13262, t13263, t13264, t13265, t13266, t5812, t602);
    (t21617, t21618, t21621, t21624, t21635, t21639, t21645, t21658, t21660, t21661, t21663)
}
