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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta413<F: Float>(t31027: F, t31143: F, t116: F, t31157: F, t46089: F, t655: F, t10199: F, t2339: F, t2: F, t665: F, t10416: F, t1310: F, t1312: F, t13440: F, t14310: F, t18227: F, t2198: F, t2322: F, t31161: F, t31164: F, t31169: F, t31382: F, t31401: F, t31403: F, t31451: F, t31452: F, t31456: F, t31459: F, t4248: F, t4254: F, t5517: F, t5523: F, t5787: F, t651: F, t7889: F, t8320: F, t8327: F, t8411: F, t8413: F, t13426: F, t13435: F, t18153: F, t18163: F, t1843: F, t2199: F, t27123: F, t27126: F, t31172: F, t31390: F, t31407: F, t3813: F, t7732: F, t8307: F, t8321: F, t8393: F, t8406: F, t8407: F, t98535: F, t31430: F, t31032: F, t31434: F, t31447: F, t2357: F, t55: F, t116929: F, t8402: F, t116926: F, t8395: F, t2289: F, t8399: F, t31424: F, t101457: F, t101463: F, t116919: F, t13509: F, t1509: F, t1513: F, t2340: F, t2358: F, t2362: F, t2366: F, t31035: F, t31149: F, t31287: F, t31429: F, t31433: F, t4287: F, t661: F, t8258: F, t8267: F, t8311: F, t8315: F, t31440: F, t31444: F, t108: F, t116912: F, t31417: F, t31421: F, t101460: F, t117183: F, t117184: F, t117186: F, t117188: F, t117190: F, t117198: F, t117218: F, t117226: F, t2194: F, t31142: F, t36308: F, t36315: F, t4279: F, t114: F, t101522: F, t2201: F, t28219: F, t31201: F, t4151: F, t49686: F, t508: F, t75485: F, t75667: F, t8325: F, t98484: F, t98487: F, t1453: F, t1911: F, t31158: F, t569: F, t2204: F, t5808: F, t1921: F, t8330: F, t1913: F, t8349: F, t31512: F, t571: F, t117168: F, t117170: F, t1464: F, t18178: F, t18217: F, t2205: F, t2212: F, t3: F, t31205: F, t31464: F, t4168: F, t575: F, t8331: F, t8417: F, t31463: F, t8416: F, t1455: F, t8433: F, t1459: F, t1461: F, t1518: F, t18190: F, t1916: F, t1918: F, t2209: F, t2327: F, t2371: F, t31217: F, t31234: F, t31241: F, t31475: F, t31497: F, t31500: F, t31505: F, t4158: F, t4165: F, t4292: F, t572: F, t573: F, t5795: F, t5802: F, t670: F, t8336: F, t8343: F, t8346: F, t8421: F, t8430: F, param_d: F, t117: F, t13514: F, t18204: F, t18208: F, t18211: F, t18214: F, t2207: F, t31231: F, t31235: F, t31238: F, t31493: F, t31494: F, t31506: F, t31509: F, t4162: F, t5805: F, t8342: F, t8427: F, t5789: F, t117151: F, t117153: F, t117155: F, t117161: F, t1456: F, t1458: F, t1914: F, t31244: F, t4154: F, t5790: F) -> (F, F) {
        let (t117228, t117338, t117461, t117544, t117545, t117845) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1494::<F>(t31027, t31143, t116, t31157, t46089, t655, t10199, t2339, t2, t665, t10416, t1310, t1312, t13440, t14310, t18227, t2198, t2322, t31161, t31164, t31169, t31382, t31401, t31403, t31451, t31452, t31456, t31459, t4248, t4254, t5517, t5523, t5787, t651, t7889, t8320, t8327, t8411, t8413);
        let t117889 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1495::<F>(t10416, t13426, t13435, t18153, t18163, t18227, t1843, t2198, t2199, t2322, t27123, t27126, t31157, t31172, t31390, t31407, t3813, t4254, t651, t7732, t8307, t8321, t8327, t8393, t8406, t8407, t8411, t98535);
        let (t117918, t117920, t117927, t117932, t117936, t117938) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1496::<F>(t31027, t31430, t31032, t31434, t117461, t31447, t2357, t55, t116929, t8402, t116926, t8395);
        let t117971 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1497::<F>(t2289, t8399, t31027, t31424, t101457, t101463, t116919, t117228, t117918, t117920, t117927, t117932, t117936, t117938, t13509, t1509, t1513, t2, t2340, t2358, t2362, t2366, t31035, t31149, t31287, t31429, t31433, t4287, t661, t8258, t8267, t8311, t8315);
        let t118017 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1498::<F>(t31027, t31440, t31032, t31444, t108, t1513, t116912, t31417, t31421, t101460, t10199, t117183, t117184, t117186, t117188, t117190, t117198, t117218, t117226, t117544, t117545, t1509, t2194, t2358, t2362, t2366, t31035, t31142, t31149, t31433, t36308, t36315, t4279, t8258, t8267, t8311, t8315);
        let (t118019, t118039) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1499::<F>(t114, t117971, t118017, t101522, t1312, t13426, t18227, t2199, t2201, t2322, t27123, t27126, t28219, t31164, t31201, t31401, t31459, t4151, t4248, t49686, t508, t651, t75485, t75667, t7732, t7889, t8307, t8321, t8325, t8327, t8406, t98484, t98487);
        let t118083 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1500::<F>(t118019, t1312, t13426, t13435, t1453, t18163, t18227, t1911, t2199, t2201, t2322, t27123, t31157, t31158, t31161, t31169, t31172, t31382, t31390, t31451, t4248, t4254, t49686, t569, t75485, t75667, t7732, t7889, t8307, t8325, t8393, t8413, t98484, t98487);
        let (t118085, t118089, t118091, t118094, t118099) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1501::<F>(t117845, t117889, t118039, t118083, t2204, t5808, t1921, t8330, t1913, t8349, t31512, t571);
        let t118100 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1502::<F>(t117168, t117170, t118085, t118089, t118091, t118094, t118099, t1464, t18178, t18217, t1921, t2205, t2212, t3, t31205, t31464, t4168, t575, t5808, t8331, t8417);
        let (t118106, t118108, t118110, t118154) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1503::<F>(t31463, t575, t1464, t8416, t1455, t8433, t116, t31451, t117338, t118085, t1459, t1461, t1518, t18190, t1916, t1918, t2209, t2327, t2371, t31217, t31234, t31241, t31475, t31497, t31500, t31505, t4158, t4165, t4292, t572, t573, t5795, t5802, t670, t8336, t8343, t8346, t8406, t8421, t8430, param_d);
        let t118198 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1504::<F>(t2198, t2371, t670, t8320, t117, t118019, t13514, t1459, t1518, t18204, t18208, t18211, t18214, t1916, t2207, t31231, t31235, t31238, t31493, t31494, t31506, t31509, t4158, t4162, t4292, t572, t5805, t8336, t8342, t8421, t8427);
        let t118204 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1505::<F>(t2212, t5789, t117151, t117153, t117155, t117161, t118106, t118108, t118110, t118154, t118198, t1456, t1458, t1914, t31244, t31512, t4154, t5790, t8349, t8433);
    (t118100, t118204)
}
