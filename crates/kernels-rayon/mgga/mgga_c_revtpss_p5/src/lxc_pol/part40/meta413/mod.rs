//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta413 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1494;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1495;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1496;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1497;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1498;
use chunk5::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1499;
use chunk6::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1500;
use chunk7::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1501;
use chunk8::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1502;
use chunk9::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1503;
use chunk10::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1504;
use chunk11::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1505;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta413(t31027: f64, t31143: f64, t116: f64, t31157: f64, t46089: f64, t655: f64, t10199: f64, t2339: f64, t2: f64, t665: f64, t10416: f64, t1310: f64, t1312: f64, t13440: f64, t14310: f64, t18227: f64, t2198: f64, t2322: f64, t31161: f64, t31164: f64, t31169: f64, t31382: f64, t31401: f64, t31403: f64, t31451: f64, t31452: f64, t31456: f64, t31459: f64, t4248: f64, t4254: f64, t5517: f64, t5523: f64, t5787: f64, t651: f64, t7889: f64, t8320: f64, t8327: f64, t8411: f64, t8413: f64, t13426: f64, t13435: f64, t18153: f64, t18163: f64, t1843: f64, t2199: f64, t27123: f64, t27126: f64, t31172: f64, t31390: f64, t31407: f64, t3813: f64, t7732: f64, t8307: f64, t8321: f64, t8393: f64, t8406: f64, t8407: f64, t98535: f64, t31430: f64, t31032: f64, t31434: f64, t31447: f64, t2357: f64, t55: f64, t116929: f64, t8402: f64, t116926: f64, t8395: f64, t2289: f64, t8399: f64, t31424: f64, t101457: f64, t101463: f64, t116919: f64, t13509: f64, t1509: f64, t1513: f64, t2340: f64, t2358: f64, t2362: f64, t2366: f64, t31035: f64, t31149: f64, t31287: f64, t31429: f64, t31433: f64, t4287: f64, t661: f64, t8258: f64, t8267: f64, t8311: f64, t8315: f64, t31440: f64, t31444: f64, t108: f64, t116912: f64, t31417: f64, t31421: f64, t101460: f64, t117183: f64, t117184: f64, t117186: f64, t117188: f64, t117190: f64, t117198: f64, t117218: f64, t117226: f64, t2194: f64, t31142: f64, t36308: f64, t36315: f64, t4279: f64, t114: f64, t101522: f64, t2201: f64, t28219: f64, t31201: f64, t4151: f64, t49686: f64, t508: f64, t75485: f64, t75667: f64, t8325: f64, t98484: f64, t98487: f64, t1453: f64, t1911: f64, t31158: f64, t569: f64, t2204: f64, t5808: f64, t1921: f64, t8330: f64, t1913: f64, t8349: f64, t31512: f64, t571: f64, t117168: f64, t117170: f64, t1464: f64, t18178: f64, t18217: f64, t2205: f64, t2212: f64, t3: f64, t31205: f64, t31464: f64, t4168: f64, t575: f64, t8331: f64, t8417: f64, t31463: f64, t8416: f64, t1455: f64, t8433: f64, t1459: f64, t1461: f64, t1518: f64, t18190: f64, t1916: f64, t1918: f64, t2209: f64, t2327: f64, t2371: f64, t31217: f64, t31234: f64, t31241: f64, t31475: f64, t31497: f64, t31500: f64, t31505: f64, t4158: f64, t4165: f64, t4292: f64, t572: f64, t573: f64, t5795: f64, t5802: f64, t670: f64, t8336: f64, t8343: f64, t8346: f64, t8421: f64, t8430: f64, param_d: f64, t117: f64, t13514: f64, t18204: f64, t18208: f64, t18211: f64, t18214: f64, t2207: f64, t31231: f64, t31235: f64, t31238: f64, t31493: f64, t31494: f64, t31506: f64, t31509: f64, t4162: f64, t5805: f64, t8342: f64, t8427: f64, t5789: f64, t117151: f64, t117153: f64, t117155: f64, t117161: f64, t1456: f64, t1458: f64, t1914: f64, t31244: f64, t4154: f64, t5790: f64) -> (f64, f64) {
        let (t117228, t117338, t117461, t117544, t117545, t117845) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1494(t31027, t31143, t116, t31157, t46089, t655, t10199, t2339, t2, t665, t10416, t1310, t1312, t13440, t14310, t18227, t2198, t2322, t31161, t31164, t31169, t31382, t31401, t31403, t31451, t31452, t31456, t31459, t4248, t4254, t5517, t5523, t5787, t651, t7889, t8320, t8327, t8411, t8413);
        let t117889 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1495(t10416, t13426, t13435, t18153, t18163, t18227, t1843, t2198, t2199, t2322, t27123, t27126, t31157, t31172, t31390, t31407, t3813, t4254, t651, t7732, t8307, t8321, t8327, t8393, t8406, t8407, t8411, t98535);
        let (t117918, t117920, t117927, t117932, t117936, t117938) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1496(t31027, t31430, t31032, t31434, t117461, t31447, t2357, t55, t116929, t8402, t116926, t8395);
        let t117971 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1497(t2289, t8399, t31027, t31424, t101457, t101463, t116919, t117228, t117918, t117920, t117927, t117932, t117936, t117938, t13509, t1509, t1513, t2, t2340, t2358, t2362, t2366, t31035, t31149, t31287, t31429, t31433, t4287, t661, t8258, t8267, t8311, t8315);
        let t118017 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1498(t31027, t31440, t31032, t31444, t108, t1513, t116912, t31417, t31421, t101460, t10199, t117183, t117184, t117186, t117188, t117190, t117198, t117218, t117226, t117544, t117545, t1509, t2194, t2358, t2362, t2366, t31035, t31142, t31149, t31433, t36308, t36315, t4279, t8258, t8267, t8311, t8315);
        let (t118019, t118039) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1499(t114, t117971, t118017, t101522, t1312, t13426, t18227, t2199, t2201, t2322, t27123, t27126, t28219, t31164, t31201, t31401, t31459, t4151, t4248, t49686, t508, t651, t75485, t75667, t7732, t7889, t8307, t8321, t8325, t8327, t8406, t98484, t98487);
        let t118083 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1500(t118019, t1312, t13426, t13435, t1453, t18163, t18227, t1911, t2199, t2201, t2322, t27123, t31157, t31158, t31161, t31169, t31172, t31382, t31390, t31451, t4248, t4254, t49686, t569, t75485, t75667, t7732, t7889, t8307, t8325, t8393, t8413, t98484, t98487);
        let (t118085, t118089, t118091, t118094, t118099) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1501(t117845, t117889, t118039, t118083, t2204, t5808, t1921, t8330, t1913, t8349, t31512, t571);
        let t118100 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1502(t117168, t117170, t118085, t118089, t118091, t118094, t118099, t1464, t18178, t18217, t1921, t2205, t2212, t3, t31205, t31464, t4168, t575, t5808, t8331, t8417);
        let (t118106, t118108, t118110, t118154) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1503(t31463, t575, t1464, t8416, t1455, t8433, t116, t31451, t117338, t118085, t1459, t1461, t1518, t18190, t1916, t1918, t2209, t2327, t2371, t31217, t31234, t31241, t31475, t31497, t31500, t31505, t4158, t4165, t4292, t572, t573, t5795, t5802, t670, t8336, t8343, t8346, t8406, t8421, t8430, param_d);
        let t118198 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1504(t2198, t2371, t670, t8320, t117, t118019, t13514, t1459, t1518, t18204, t18208, t18211, t18214, t1916, t2207, t31231, t31235, t31238, t31493, t31494, t31506, t31509, t4158, t4162, t4292, t572, t5805, t8336, t8342, t8421, t8427);
        let t118204 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1505(t2212, t5789, t117151, t117153, t117155, t117161, t118106, t118108, t118110, t118154, t118198, t1456, t1458, t1914, t31244, t31512, t4154, t5790, t8349, t8433);
    (t118100, t118204)
}
