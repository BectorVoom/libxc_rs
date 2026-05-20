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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta597<F: Float>(t99066: F, t99069: F, t99073: F, t99077: F, t93004: F, t93010: F, t93016: F, t95678: F, t95680: F, t95684: F, t99063: F, t99071: F, t99075: F, t99085: F, t99091: F, t93026: F, t93028: F, t93031: F, t93035: F, t93043: F, t93045: F, t93049: F, t93055: F, t93058: F, t99081: F, t99099: F, t99102: F, t99113: F, t93067: F, t93069: F, t93073: F, t93077: F, t93080: F, t93084: F, t93086: F, t93088: F, t93091: F, t93095: F, t103259: F, t103271: F, t103284: F, t103298: F, t103310: F, t136: F, t2457: F, t8015: F, t25299: F, t2439: F, t780: F, t785: F, t7997: F, t15038: F, t1558: F, t1580: F, t213: F, t225: F, t25391: F, t25394: F, t25407: F, t257: F, t26441: F, t26550: F, t27199: F, t7403: F, t8016: F, t95832: F, t95834: F, t95836: F, t95847: F, t95855: F, t95857: F, t95894: F, t7407: F, t99272: F, t26482: F, t99404: F, t98849: F, t25305: F, t14991: F, t95936: F, t99373: F, t2435: F, t28390: F, t102993: F, t25411: F, t231: F, t25383: F, t26547: F, t28340: F, t28418: F, t4534: F, t7070: F, t7071: F, t7076: F, t836: F, t886: F, t95859: F, t95862: F, t95866: F, t2470: F, t28359: F, t7064: F, t822: F, t28313: F, t25387: F, t95822: F, t98892: F, t95537: F, t1957: F, t25372: F, t98801: F, t25386: F, t2471: F, t28373: F, t1956: F, t233: F, t26493: F, t27353: F, t28411: F, t51436: F, t95872: F, t95876: F, t10867: F, t2061: F, t14481: F, t2062: F, t2782: F, t26519: F, t99257: F, t28341: F, t786: F, t789: F, t10073: F, t1579: F, t2066: F, t25390: F, t2722: F, t14978: F, t2067: F, t25317: F, t27312: F, t28309: F, t51570: F, t95825: F, t95888: F, t95891: F, t95893: F, t95899: F, t95900: F, t99300: F, t28448: F, t28314: F, t93364: F, t25416: F, t2645: F, t26489: F, t2723: F, t4533: F, t7398: F, t8007: F, t93126: F, t95902: F, t95905: F, t95911: F, t95914: F, t95925: F, t95927: F, t99360: F) -> (F, F, F, F, F, F, F) {
        let t103321 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2015::<F>(t99066, t99069, t99073, t99077, t93004, t93010, t93016, t95678, t95680, t95684, t99063, t99071, t99075);
        let t103335 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2016::<F>(t99085, t99091, t93026, t93028, t93031, t93035, t93043, t93045, t93049, t93055, t93058, t99081);
        let t103349 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2017::<F>(t99099, t99102, t99113, t93067, t93069, t93073, t93077, t93080, t93084, t93086, t93088, t93091, t93095);
        let (t103352, t103363, t103364) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2018::<F>(t103259, t103271, t103284, t103298, t103310, t103321, t103335, t103349, t136, t2457, t8015, t25299);
        let t103380 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2019::<F>(t2439, t780, t785, t7997, t103352, t103364, t15038, t1558, t1580, t213, t225, t25391, t25394, t25407, t257, t26441, t26550, t27199, t7403, t8016, t95832, t95834, t95836, t95847, t95855, t95857, t95894);
        let (t103382, t103391, t103393, t103394, t103396, t103399, t103400) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2020::<F>(t7407, t99272, t26482, t99404, t98849, t103363, t25305, t14991, t95936, t99373, t2435, t28390);
        let t103412 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2021::<F>(t102993, t25411, t103382, t103391, t103393, t103394, t103396, t103399, t103400, t231, t25383, t26547, t28340, t28418, t4534, t7070, t7071, t7076, t836, t886, t95859, t95862, t95866);
        let (t103421, t103422, t103424, t103431, t103432, t103435, t103437, t103438) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2022::<F>(t2470, t28359, t7064, t7997, t822, t28313, t25387, t95822, t98892, t95537, t1957, t26550);
        let t103451 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2023::<F>(t103438, t25372, t98801, t25386, t2471, t28373, t103352, t103422, t103424, t103432, t103435, t103437, t1956, t1957, t233, t25383, t25391, t25394, t26493, t26550, t27199, t27353, t28411, t51436, t95872, t95876);
        let (t103452, t103462, t103463, t103467, t103471) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2024::<F>(t10867, t2061, t14481, t2062, t2782, t26519, t99257, t28341, t786, t789, t10073, t1579, t2066, t25390);
        let (t103483, t103488) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2025::<F>(t2722, t7997, t103452, t103462, t103463, t103467, t103471, t14978, t2061, t2067, t231, t25317, t25391, t27312, t27353, t28309, t51570, t7070, t7071, t7076, t886, t95825, t95888, t95891, t95893, t95899, t95900, t99300);
        let t103519 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2026::<F>(t2435, t28448, t28314, t93364, t103483, t231, t25391, t25416, t2645, t26489, t26550, t27199, t2723, t4533, t7070, t7071, t7076, t7398, t7997, t8007, t93126, t95902, t95905, t95911, t95914, t95925, t95927, t99360);
    (t103380, t103412, t103421, t103431, t103451, t103488, t103519)
}
