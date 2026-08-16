//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta594 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1989;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1990;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1991;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1992;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1993;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1994;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta594(t102573: f64, t13739: f64, t1904: f64, t2027: f64, t2028: f64, t25930: f64, t26304: f64, t27868: f64, t28911: f64, t28918: f64, t48020: f64, t49380: f64, t545: f64, t5774: f64, t7295: f64, t7296: f64, t7506: f64, t7511: f64, t94705: f64, t94823: f64, t96512: f64, t96567: f64, t96570: f64, t96577: f64, t96584: f64, t96588: f64, t96591: f64, t97871: f64, t98062: f64, t670: f64, t7356: f64, t2051: f64, t2371: f64, t102019: f64, t13426: f64, t13514: f64, t1518: f64, t18227: f64, t2055: f64, t26153: f64, t26399: f64, t28653: f64, t28658: f64, t4248: f64, t4292: f64, t49686: f64, t7359: f64, t7373: f64, t75485: f64, t75667: f64, t95357: f64, t101522: f64, t101761: f64, t101767: f64, t101980: f64, t10416: f64, t1312: f64, t13435: f64, t13440: f64, t2322: f64, t27123: f64, t28219: f64, t28683: f64, t5523: f64, t7889: f64, t7983: f64, t98484: f64, t98487: f64, t28925: f64, t531: f64, t101435: f64, t102070: f64, t102111: f64, t102148: f64, t102175: f64, t102222: f64, t102248: f64, t102282: f64, t102313: f64, t102341: f64, t102374: f64, t102406: f64, t102443: f64, t102584: f64, t102612: f64, t102642: f64, t102669: f64, t13625: f64, t13872: f64, t1450: f64, t1453: f64, t2014: f64, t2108: f64, t25082: f64, t25802: f64, t26218: f64, t26411: f64, t28167: f64, t28176: f64, t28196: f64, t28286: f64, t28686: f64, t28707: f64, t28718: f64, t28727: f64, t28927: f64, t28929: f64, t28939: f64, t34495: f64, t4297: f64, t532: f64, t569: f64, t7235: f64, t7238: f64, t75365: f64, t8108: f64, t9069: f64, t95088: f64, t98496: f64, t98579: f64, t198: f64, t7443: f64, t2411: f64, t28455: f64, t1940: f64, t2071: f64, t580: f64, t205: f64, t7427: f64, t1468: f64, t2403: f64, t25198: f64, t25449: f64, t26425: f64, t26581: f64, t27160: f64, t27166: f64, t27385: f64, t27395: f64, t28291: f64, t28456: f64, t28460: f64, t28472: f64, t4541: f64, t605: f64, t7092: f64, t7428: f64, t8020: f64, t95511: f64, t98688: f64, t98733: f64, t98760: f64, t98787: f64, t98633: f64, t206: f64, t8019: f64, t2257: f64, t25208: f64, t25211: f64, t25452: f64, t26585: f64, t27169: f64, t27402: f64, t7432: f64, t7787: f64, t95527: f64, t98694: f64, t98699: f64, t98702: f64, t98713: f64, t98716: f64, t98764: f64, t99558: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t102700 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1989(t102573, t13739, t1904, t2027, t2028, t25930, t26304, t27868, t28911, t28918, t48020, t49380, t545, t5774, t7295, t7296, t7506, t7511, t94705, t94823, t96512, t96567, t96570, t96577, t96584, t96588, t96591, t97871, t98062);
        let (t102714, t102719, t102738) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1990(t670, t7356, t2051, t2371, t102019, t13426, t13514, t1518, t18227, t2055, t26153, t26399, t28653, t28658, t4248, t4292, t49686, t7359, t7373, t75485, t75667, t95357);
        let t102764 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1991(t101522, t101761, t101767, t101980, t10416, t1312, t13435, t13440, t2055, t2322, t26153, t27123, t28219, t28683, t5523, t7373, t7889, t7983, t98484, t98487);
        let t102791 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1992(t28925, t531, t101435, t102070, t102111, t102148, t102175, t102222, t102248, t102282, t102313, t102341, t102374, t102406, t102443, t102584, t102612, t102642, t102669, t102700, t102738, t102764, t13625, t13872, t1450, t1453, t2014, t2108, t25082, t25802, t26218, t26399, t26411, t28167, t28176, t28196, t28286, t28686, t28707, t28718, t28727, t28927, t28929, t28939, t34495, t4248, t4297, t532, t569, t7235, t7238, t75365, t8108, t9069, t95088, t98496, t98579);
        let (t102851, t102854, t102858, t102864, t102867) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1993(t198, t7443, t2411, t28455, t1940, t2071, t580, t205, t7427, t1468, t2403, t25198, t25449, t26425, t26581, t27160, t27166, t27385, t27395, t28291, t28456, t28460, t28472, t4541, t605, t7092, t7428, t8020, t95511, t98688, t98733, t98760, t98787);
        let (t102877, t102888, t102905) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1994(t28472, t98633, t198, t206, t8019, t1940, t2257, t2403, t25208, t25211, t25452, t26425, t26585, t27169, t27402, t28291, t28460, t7428, t7432, t7787, t8020, t95527, t98694, t98699, t98702, t98713, t98716, t98764, t99558);
    (t102714, t102719, t102791, t102851, t102854, t102858, t102864, t102867, t102877, t102888, t102905)
}
