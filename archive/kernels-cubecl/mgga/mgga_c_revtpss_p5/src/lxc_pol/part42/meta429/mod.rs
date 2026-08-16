//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta429 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1496;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1497;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1498;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1499;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1500;
use chunk5::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1501;
use chunk6::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1502;
use chunk7::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1503;
use chunk8::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1504;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta429<F: Float>(t31027: F, t31633: F, t31640: F, t625: F, t105872: F, t116919: F, t117183: F, t117184: F, t117186: F, t117976: F, t117978: F, t118009: F, t118011: F, t31035: F, t31149: F, t5891: F, t5911: F, t661: F, t8267: F, t8311: F, t8315: F, t114: F, t118655: F, t118688: F, t118728: F, t1312: F, t13426: F, t1453: F, t18227: F, t18245: F, t2322: F, t27123: F, t27126: F, t28219: F, t30143: F, t31382: F, t31407: F, t31459: F, t31653: F, t31654: F, t31660: F, t4248: F, t4254: F, t5517: F, t5523: F, t569: F, t651: F, t7732: F, t7889: F, t8325: F, t8327: F, t8406: F, t8407: F, t8411: F, t8413: F, t108710: F, t109150: F, t109153: F, t109242: F, t2198: F, t2201: F, t22506: F, t29508: F, t30138: F, t31390: F, t31401: F, t31456: F, t31674: F, t8307: F, t8321: F, t8393: F, t108714: F, t1310: F, t1843: F, t21658: F, t2199: F, t31403: F, t31451: F, t31452: F, t31663: F, t31677: F, t508: F, t6765: F, t75439: F, t8320: F, t85360: F, t1911: F, t31657: F, t5787: F, t6934: F, t117: F, t118630: F, t1459: F, t1461: F, t1916: F, t1918: F, t2207: F, t2209: F, t22544: F, t22556: F, t31475: F, t31494: F, t31497: F, t31500: F, t31711: F, t31728: F, t35858: F, t4292: F, t572: F, t573: F, t5795: F, t5805: F, t5883: F, t670: F, t6941: F, t6948: F, t8336: F, t8343: F, t8421: F, t8427: F, t8430: F, param_d: F, t2212: F, t6936: F, t118089: F, t118091: F, t118094: F, t118099: F, t118106: F, t118629: F, t1456: F, t1458: F, t1464: F, t1914: F, t2205: F, t22571: F, t3: F, t31512: F, t31701: F, t31737: F, t575: F, t5808: F, t8417: F, t2204: F, t6951: F, t31700: F, t571: F, t1913: F, t8433: F, t1921: F, t8416: F, t118108: F, t118110: F, t118203: F, t22533: F, t31464: F, t5790: F, t6937: F, t8331: F, t8349: F) -> F {
        let t118746 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1496::<F>(t31027, t31633, t31640, t625, t105872, t116919, t117183, t117184, t117186, t117976, t117978, t118009, t118011, t31035, t31149, t5891, t5911, t661, t8267, t8311, t8315);
        let (t118749, t118822) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1497::<F>(t114, t118655, t118688, t118728, t118746, t1312, t13426, t1453, t18227, t18245, t2322, t27123, t27126, t28219, t30143, t31382, t31407, t31459, t31653, t31654, t31660, t4248, t4254, t5517, t5523, t569, t651, t7732, t7889, t8325, t8327, t8406, t8407, t8411, t8413);
        let t118864 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1498::<F>(t108710, t109150, t109153, t109242, t1312, t13426, t18227, t2198, t2201, t22506, t2322, t27123, t27126, t29508, t30138, t31390, t31401, t31456, t31459, t31674, t4248, t4254, t7732, t7889, t8307, t8321, t8327, t8393, t8411, t8413);
        let t118911 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1499::<F>(t108710, t108714, t118749, t1310, t13426, t18245, t1843, t21658, t2198, t2199, t2201, t2322, t30143, t31403, t31451, t31452, t31653, t31663, t31677, t4248, t4254, t508, t5523, t651, t6765, t75439, t7732, t8307, t8320, t8327, t8411, t85360);
        let t118955 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1500::<F>(t109150, t109153, t1312, t13426, t18227, t18245, t1911, t2199, t2322, t27123, t30138, t31382, t31390, t31401, t31451, t31452, t31657, t31663, t4248, t5523, t5787, t6934, t7732, t7889, t8307, t8320, t8321, t8325, t8393, t8406, t8413);
        let (t118957, t118962) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1501::<F>(t118822, t118864, t118911, t118955, t117, t118630, t118749, t1459, t1461, t1916, t1918, t2207, t2209, t22544, t22556, t31475, t31494, t31497, t31500, t31711, t31728, t35858, t4292, t572, t573, t5795, t5805, t5883, t670, t6941, t6948, t8320, t8336, t8343, t8421, t8427, t8430, param_d);
        let t118975 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1502::<F>(t2212, t6936, t118089, t118091, t118094, t118099, t118106, t118629, t118957, t118962, t1456, t1458, t1464, t1914, t2205, t22571, t3, t31512, t31701, t31737, t575, t5808, t8417);
        let t118990 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1503::<F>(t2204, t6951, t31700, t575, t31737, t571, t1913, t8433, t1921, t8416, t118108, t118110, t118203, t2212, t22533, t31464, t5790, t6937, t8331, t8349);
        let tv4rho3tau5 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1504::<F>(t118975, t118990);
    tv4rho3tau5
}
