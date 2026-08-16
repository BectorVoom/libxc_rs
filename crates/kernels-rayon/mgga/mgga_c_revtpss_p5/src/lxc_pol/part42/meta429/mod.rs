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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta429(t31027: f64, t31633: f64, t31640: f64, t625: f64, t105872: f64, t116919: f64, t117183: f64, t117184: f64, t117186: f64, t117976: f64, t117978: f64, t118009: f64, t118011: f64, t31035: f64, t31149: f64, t5891: f64, t5911: f64, t661: f64, t8267: f64, t8311: f64, t8315: f64, t114: f64, t118655: f64, t118688: f64, t118728: f64, t1312: f64, t13426: f64, t1453: f64, t18227: f64, t18245: f64, t2322: f64, t27123: f64, t27126: f64, t28219: f64, t30143: f64, t31382: f64, t31407: f64, t31459: f64, t31653: f64, t31654: f64, t31660: f64, t4248: f64, t4254: f64, t5517: f64, t5523: f64, t569: f64, t651: f64, t7732: f64, t7889: f64, t8325: f64, t8327: f64, t8406: f64, t8407: f64, t8411: f64, t8413: f64, t108710: f64, t109150: f64, t109153: f64, t109242: f64, t2198: f64, t2201: f64, t22506: f64, t29508: f64, t30138: f64, t31390: f64, t31401: f64, t31456: f64, t31674: f64, t8307: f64, t8321: f64, t8393: f64, t108714: f64, t1310: f64, t1843: f64, t21658: f64, t2199: f64, t31403: f64, t31451: f64, t31452: f64, t31663: f64, t31677: f64, t508: f64, t6765: f64, t75439: f64, t8320: f64, t85360: f64, t1911: f64, t31657: f64, t5787: f64, t6934: f64, t117: f64, t118630: f64, t1459: f64, t1461: f64, t1916: f64, t1918: f64, t2207: f64, t2209: f64, t22544: f64, t22556: f64, t31475: f64, t31494: f64, t31497: f64, t31500: f64, t31711: f64, t31728: f64, t35858: f64, t4292: f64, t572: f64, t573: f64, t5795: f64, t5805: f64, t5883: f64, t670: f64, t6941: f64, t6948: f64, t8336: f64, t8343: f64, t8421: f64, t8427: f64, t8430: f64, param_d: f64, t2212: f64, t6936: f64, t118089: f64, t118091: f64, t118094: f64, t118099: f64, t118106: f64, t118629: f64, t1456: f64, t1458: f64, t1464: f64, t1914: f64, t2205: f64, t22571: f64, t3: f64, t31512: f64, t31701: f64, t31737: f64, t575: f64, t5808: f64, t8417: f64, t2204: f64, t6951: f64, t31700: f64, t571: f64, t1913: f64, t8433: f64, t1921: f64, t8416: f64, t118108: f64, t118110: f64, t118203: f64, t22533: f64, t31464: f64, t5790: f64, t6937: f64, t8331: f64, t8349: f64) -> f64 {
        let t118746 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1496(t31027, t31633, t31640, t625, t105872, t116919, t117183, t117184, t117186, t117976, t117978, t118009, t118011, t31035, t31149, t5891, t5911, t661, t8267, t8311, t8315);
        let (t118749, t118822) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1497(t114, t118655, t118688, t118728, t118746, t1312, t13426, t1453, t18227, t18245, t2322, t27123, t27126, t28219, t30143, t31382, t31407, t31459, t31653, t31654, t31660, t4248, t4254, t5517, t5523, t569, t651, t7732, t7889, t8325, t8327, t8406, t8407, t8411, t8413);
        let t118864 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1498(t108710, t109150, t109153, t109242, t1312, t13426, t18227, t2198, t2201, t22506, t2322, t27123, t27126, t29508, t30138, t31390, t31401, t31456, t31459, t31674, t4248, t4254, t7732, t7889, t8307, t8321, t8327, t8393, t8411, t8413);
        let t118911 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1499(t108710, t108714, t118749, t1310, t13426, t18245, t1843, t21658, t2198, t2199, t2201, t2322, t30143, t31403, t31451, t31452, t31653, t31663, t31677, t4248, t4254, t508, t5523, t651, t6765, t75439, t7732, t8307, t8320, t8327, t8411, t85360);
        let t118955 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1500(t109150, t109153, t1312, t13426, t18227, t18245, t1911, t2199, t2322, t27123, t30138, t31382, t31390, t31401, t31451, t31452, t31657, t31663, t4248, t5523, t5787, t6934, t7732, t7889, t8307, t8320, t8321, t8325, t8393, t8406, t8413);
        let (t118957, t118962) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1501(t118822, t118864, t118911, t118955, t117, t118630, t118749, t1459, t1461, t1916, t1918, t2207, t2209, t22544, t22556, t31475, t31494, t31497, t31500, t31711, t31728, t35858, t4292, t572, t573, t5795, t5805, t5883, t670, t6941, t6948, t8320, t8336, t8343, t8421, t8427, t8430, param_d);
        let t118975 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1502(t2212, t6936, t118089, t118091, t118094, t118099, t118106, t118629, t118957, t118962, t1456, t1458, t1464, t1914, t2205, t22571, t3, t31512, t31701, t31737, t575, t5808, t8417);
        let t118990 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1503(t2204, t6951, t31700, t575, t31737, t571, t1913, t8433, t1921, t8416, t118108, t118110, t118203, t2212, t22533, t31464, t5790, t6937, t8331, t8349);
        let tv4rho3tau5 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1504(t118975, t118990);
    tv4rho3tau5
}
