//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta594 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1989;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1990;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1991;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1992;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1993;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1994;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta594<F: Float>(t102573: F, t13739: F, t1904: F, t2027: F, t2028: F, t25930: F, t26304: F, t27868: F, t28911: F, t28918: F, t48020: F, t49380: F, t545: F, t5774: F, t7295: F, t7296: F, t7506: F, t7511: F, t94705: F, t94823: F, t96512: F, t96567: F, t96570: F, t96577: F, t96584: F, t96588: F, t96591: F, t97871: F, t98062: F, t670: F, t7356: F, t2051: F, t2371: F, t102019: F, t13426: F, t13514: F, t1518: F, t18227: F, t2055: F, t26153: F, t26399: F, t28653: F, t28658: F, t4248: F, t4292: F, t49686: F, t7359: F, t7373: F, t75485: F, t75667: F, t95357: F, t101522: F, t101761: F, t101767: F, t101980: F, t10416: F, t1312: F, t13435: F, t13440: F, t2322: F, t27123: F, t28219: F, t28683: F, t5523: F, t7889: F, t7983: F, t98484: F, t98487: F, t28925: F, t531: F, t101435: F, t102070: F, t102111: F, t102148: F, t102175: F, t102222: F, t102248: F, t102282: F, t102313: F, t102341: F, t102374: F, t102406: F, t102443: F, t102584: F, t102612: F, t102642: F, t102669: F, t13625: F, t13872: F, t1450: F, t1453: F, t2014: F, t2108: F, t25082: F, t25802: F, t26218: F, t26411: F, t28167: F, t28176: F, t28196: F, t28286: F, t28686: F, t28707: F, t28718: F, t28727: F, t28927: F, t28929: F, t28939: F, t34495: F, t4297: F, t532: F, t569: F, t7235: F, t7238: F, t75365: F, t8108: F, t9069: F, t95088: F, t98496: F, t98579: F, t198: F, t7443: F, t2411: F, t28455: F, t1940: F, t2071: F, t580: F, t205: F, t7427: F, t1468: F, t2403: F, t25198: F, t25449: F, t26425: F, t26581: F, t27160: F, t27166: F, t27385: F, t27395: F, t28291: F, t28456: F, t28460: F, t28472: F, t4541: F, t605: F, t7092: F, t7428: F, t8020: F, t95511: F, t98688: F, t98733: F, t98760: F, t98787: F, t98633: F, t206: F, t8019: F, t2257: F, t25208: F, t25211: F, t25452: F, t26585: F, t27169: F, t27402: F, t7432: F, t7787: F, t95527: F, t98694: F, t98699: F, t98702: F, t98713: F, t98716: F, t98764: F, t99558: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t102700 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1989::<F>(t102573, t13739, t1904, t2027, t2028, t25930, t26304, t27868, t28911, t28918, t48020, t49380, t545, t5774, t7295, t7296, t7506, t7511, t94705, t94823, t96512, t96567, t96570, t96577, t96584, t96588, t96591, t97871, t98062);
        let (t102714, t102719, t102738) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1990::<F>(t670, t7356, t2051, t2371, t102019, t13426, t13514, t1518, t18227, t2055, t26153, t26399, t28653, t28658, t4248, t4292, t49686, t7359, t7373, t75485, t75667, t95357);
        let t102764 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1991::<F>(t101522, t101761, t101767, t101980, t10416, t1312, t13435, t13440, t2055, t2322, t26153, t27123, t28219, t28683, t5523, t7373, t7889, t7983, t98484, t98487);
        let t102791 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1992::<F>(t28925, t531, t101435, t102070, t102111, t102148, t102175, t102222, t102248, t102282, t102313, t102341, t102374, t102406, t102443, t102584, t102612, t102642, t102669, t102700, t102738, t102764, t13625, t13872, t1450, t1453, t2014, t2108, t25082, t25802, t26218, t26399, t26411, t28167, t28176, t28196, t28286, t28686, t28707, t28718, t28727, t28927, t28929, t28939, t34495, t4248, t4297, t532, t569, t7235, t7238, t75365, t8108, t9069, t95088, t98496, t98579);
        let (t102851, t102854, t102858, t102864, t102867) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1993::<F>(t198, t7443, t2411, t28455, t1940, t2071, t580, t205, t7427, t1468, t2403, t25198, t25449, t26425, t26581, t27160, t27166, t27385, t27395, t28291, t28456, t28460, t28472, t4541, t605, t7092, t7428, t8020, t95511, t98688, t98733, t98760, t98787);
        let (t102877, t102888, t102905) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1994::<F>(t28472, t98633, t198, t206, t8019, t1940, t2257, t2403, t25208, t25211, t25452, t26425, t26585, t27169, t27402, t28291, t28460, t7428, t7432, t7787, t8020, t95527, t98694, t98699, t98702, t98713, t98716, t98764, t99558);
    (t102714, t102719, t102791, t102851, t102854, t102858, t102864, t102867, t102877, t102888, t102905)
}
