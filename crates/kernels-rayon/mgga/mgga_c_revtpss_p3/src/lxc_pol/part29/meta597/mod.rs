//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta597 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2015;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2016;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2017;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2018;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2019;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2020;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2021;
use chunk7::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2022;
use chunk8::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2023;
use chunk9::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2024;
use chunk10::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2025;
use chunk11::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2026;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta597(t99066: f64, t99069: f64, t99073: f64, t99077: f64, t93004: f64, t93010: f64, t93016: f64, t95678: f64, t95680: f64, t95684: f64, t99063: f64, t99071: f64, t99075: f64, t99085: f64, t99091: f64, t93026: f64, t93028: f64, t93031: f64, t93035: f64, t93043: f64, t93045: f64, t93049: f64, t93055: f64, t93058: f64, t99081: f64, t99099: f64, t99102: f64, t99113: f64, t93067: f64, t93069: f64, t93073: f64, t93077: f64, t93080: f64, t93084: f64, t93086: f64, t93088: f64, t93091: f64, t93095: f64, t103259: f64, t103271: f64, t103284: f64, t103298: f64, t103310: f64, t136: f64, t2457: f64, t8015: f64, t25299: f64, t2439: f64, t780: f64, t785: f64, t7997: f64, t15038: f64, t1558: f64, t1580: f64, t213: f64, t225: f64, t25391: f64, t25394: f64, t25407: f64, t257: f64, t26441: f64, t26550: f64, t27199: f64, t7403: f64, t8016: f64, t95832: f64, t95834: f64, t95836: f64, t95847: f64, t95855: f64, t95857: f64, t95894: f64, t7407: f64, t99272: f64, t26482: f64, t99404: f64, t98849: f64, t25305: f64, t14991: f64, t95936: f64, t99373: f64, t2435: f64, t28390: f64, t102993: f64, t25411: f64, t231: f64, t25383: f64, t26547: f64, t28340: f64, t28418: f64, t4534: f64, t7070: f64, t7071: f64, t7076: f64, t836: f64, t886: f64, t95859: f64, t95862: f64, t95866: f64, t2470: f64, t28359: f64, t7064: f64, t822: f64, t28313: f64, t25387: f64, t95822: f64, t98892: f64, t95537: f64, t1957: f64, t25372: f64, t98801: f64, t25386: f64, t2471: f64, t28373: f64, t1956: f64, t233: f64, t26493: f64, t27353: f64, t28411: f64, t51436: f64, t95872: f64, t95876: f64, t10867: f64, t2061: f64, t14481: f64, t2062: f64, t2782: f64, t26519: f64, t99257: f64, t28341: f64, t786: f64, t789: f64, t10073: f64, t1579: f64, t2066: f64, t25390: f64, t2722: f64, t14978: f64, t2067: f64, t25317: f64, t27312: f64, t28309: f64, t51570: f64, t95825: f64, t95888: f64, t95891: f64, t95893: f64, t95899: f64, t95900: f64, t99300: f64, t28448: f64, t28314: f64, t93364: f64, t25416: f64, t2645: f64, t26489: f64, t2723: f64, t4533: f64, t7398: f64, t8007: f64, t93126: f64, t95902: f64, t95905: f64, t95911: f64, t95914: f64, t95925: f64, t95927: f64, t99360: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t103321 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2015(t99066, t99069, t99073, t99077, t93004, t93010, t93016, t95678, t95680, t95684, t99063, t99071, t99075);
        let t103335 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2016(t99085, t99091, t93026, t93028, t93031, t93035, t93043, t93045, t93049, t93055, t93058, t99081);
        let t103349 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2017(t99099, t99102, t99113, t93067, t93069, t93073, t93077, t93080, t93084, t93086, t93088, t93091, t93095);
        let (t103352, t103363, t103364) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2018(t103259, t103271, t103284, t103298, t103310, t103321, t103335, t103349, t136, t2457, t8015, t25299);
        let t103380 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2019(t2439, t780, t785, t7997, t103352, t103364, t15038, t1558, t1580, t213, t225, t25391, t25394, t25407, t257, t26441, t26550, t27199, t7403, t8016, t95832, t95834, t95836, t95847, t95855, t95857, t95894);
        let (t103382, t103391, t103393, t103394, t103396, t103399, t103400) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2020(t7407, t99272, t26482, t99404, t98849, t103363, t25305, t14991, t95936, t99373, t2435, t28390);
        let t103412 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2021(t102993, t25411, t103382, t103391, t103393, t103394, t103396, t103399, t103400, t231, t25383, t26547, t28340, t28418, t4534, t7070, t7071, t7076, t836, t886, t95859, t95862, t95866);
        let (t103421, t103422, t103424, t103431, t103432, t103435, t103437, t103438) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2022(t2470, t28359, t7064, t7997, t822, t28313, t25387, t95822, t98892, t95537, t1957, t26550);
        let t103451 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2023(t103438, t25372, t98801, t25386, t2471, t28373, t103352, t103422, t103424, t103432, t103435, t103437, t1956, t1957, t233, t25383, t25391, t25394, t26493, t26550, t27199, t27353, t28411, t51436, t95872, t95876);
        let (t103452, t103462, t103463, t103467, t103471) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2024(t10867, t2061, t14481, t2062, t2782, t26519, t99257, t28341, t786, t789, t10073, t1579, t2066, t25390);
        let (t103483, t103488) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2025(t2722, t7997, t103452, t103462, t103463, t103467, t103471, t14978, t2061, t2067, t231, t25317, t25391, t27312, t27353, t28309, t51570, t7070, t7071, t7076, t886, t95825, t95888, t95891, t95893, t95899, t95900, t99300);
        let t103519 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2026(t2435, t28448, t28314, t93364, t103483, t231, t25391, t25416, t2645, t26489, t26550, t27199, t2723, t4533, t7070, t7071, t7076, t7398, t7997, t8007, t93126, t95902, t95905, t95911, t95914, t95925, t95927, t99360);
    (t103380, t103412, t103421, t103431, t103451, t103488, t103519)
}
