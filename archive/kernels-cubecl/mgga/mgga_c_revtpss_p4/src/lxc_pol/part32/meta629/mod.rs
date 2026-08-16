//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta629 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2019;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2020;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2021;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2022;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2023;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2024;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2025;
use chunk7::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2026;
use chunk8::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2027;
use chunk9::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2028;
use chunk10::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2029;
use chunk11::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2030;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta629<F: Float>(t106068: F, t106070: F, t106072: F, t106074: F, t95673: F, t95674: F, t95675: F, t95678: F, t95680: F, t99044: F, t99050: F, t99058: F, t99065: F, t103315: F, t103316: F, t103318: F, t103320: F, t103324: F, t106080: F, t106082: F, t106085: F, t106088: F, t106090: F, t93035: F, t95684: F, t103336: F, t103337: F, t106093: F, t106099: F, t106102: F, t106104: F, t106106: F, t93049: F, t93067: F, t93073: F, t93088: F, t99091: F, t99113: F, t110378: F, t110385: F, t110393: F, t110406: F, t110414: F, t27216: F, t28360: F, t30384: F, t786: F, t789: F, t103184: F, t103196: F, t103216: F, t103219: F, t103220: F, t14587: F, t1579: F, t213: F, t225: F, t25317: F, t25391: F, t257: F, t27199: F, t28418: F, t28425: F, t30356: F, t7070: F, t886: F, t95807: F, t95808: F, t95811: F, t95813: F, t30395: F, t689: F, t25431: F, t25411: F, t6072: F, t7384: F, t5977: F, t7398: F, t103037: F, t103224: F, t103234: F, t103240: F, t103364: F, t106228: F, t18324: F, t25383: F, t25416: F, t2723: F, t27275: F, t27349: F, t30392: F, t7403: F, t8016: F, t93349: F, t95836: F, t30341: F, t686: F, t72: F, t25375: F, t28314: F, t99463: F, t27213: F, t103370: F, t103382: F, t103391: F, t103393: F, t103394: F, t103396: F, t103399: F, t106404: F, t18663: F, t2067: F, t29682: F, t95825: F, t95859: F, t95862: F, t28368: F, t99404: F, t98849: F, t30405: F, t103400: F, t103404: F, t103422: F, t103424: F, t106290: F, t106410: F, t26550: F, t27353: F, t28385: F, t30337: F, t62624: F, t8012: F, t92917: F, t99303: F, t110275: F, t93281: F, t103432: F, t103435: F, t103437: F, t103441: F, t103444: F, t106172: F, t106302: F, t18313: F, t18785: F, t26547: F, t28411: F, t28426: F, t28439: F, t30381: F, t30410: F, t6049: F, t7067: F, t93118: F, t1580: F, t28447: F, t25387: F, t103449: F, t103462: F, t103463: F, t1956: F, t1957: F, t231: F, t233: F, t28394: F, t28442: F, t30379: F, t4534: F, t7076: F, t836: F, t95888: F, t95891: F, t95893: F, t99191: F, t18797: F, t26497: F, t110322: F, t103467: F, t103471: F, t103490: F, t103494: F, t1558: F, t28340: F, t28378: F, t30396: F, t30401: F, t4533: F, t7071: F, t8006: F, t95899: F, t95902: F, t95905: F, t18805: F, t95936: F, t106143: F, t106360: F, t106365: F, t14495: F, t28405: F, t95911: F, t95914: F, t95925: F, t95927: F, t95930: F) -> (F, F, F, F, F, F, F, F) {
        let t110421 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2019::<F>(t106068, t106070, t106072, t106074, t95673, t95674, t95675, t95678, t95680, t99044, t99050, t99058, t99065);
        let t110429 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2020::<F>(t103315, t103316, t103318, t103320, t103324, t106080, t106082, t106085, t106088, t106090, t93035, t95684);
        let t110441 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2021::<F>(t103336, t103337, t106093, t106099, t106102, t106104, t106106, t93049, t93067, t93073, t93088, t99091, t99113);
        let (t110444, t110453, t110459) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2022::<F>(t110378, t110385, t110393, t110406, t110414, t110421, t110429, t110441, t27216, t28360, t30384, t786, t789);
        let t110466 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2023::<F>(t103184, t103196, t103216, t103219, t103220, t110444, t110453, t110459, t14587, t1579, t213, t225, t25317, t25391, t257, t27199, t28418, t28425, t30356, t7070, t886, t95807, t95808, t95811, t95813);
        let (t110493, t110499) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2024::<F>(t30395, t689, t25431, t25411, t6072, t7384, t5977, t7398, t103037, t103224, t103234, t103240, t103364, t106228, t18324, t25383, t25391, t25416, t2723, t27275, t27349, t28425, t30392, t7070, t7403, t8016, t93349, t95836);
        let (t110502, t110519) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2025::<F>(t30341, t686, t72, t25375, t28314, t99463, t27213, t28360, t103370, t103382, t103391, t103393, t103394, t103396, t103399, t106404, t18663, t2067, t25391, t29682, t7403, t95825, t95859, t95862);
        let t110551 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2026::<F>(t28368, t99404, t98849, t30405, t689, t25431, t25411, t103400, t103404, t103422, t103424, t106290, t106410, t25391, t26550, t27199, t27349, t27353, t28385, t30337, t62624, t8012, t92917, t99303);
        let t110576 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2027::<F>(t110275, t93281, t103432, t103435, t103437, t103441, t103444, t106172, t106302, t18313, t18785, t25391, t26547, t26550, t27199, t28411, t28426, t28439, t30381, t30410, t6049, t7067, t7070, t7403, t886, t93118);
        let t110607 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2028::<F>(t6049, t689, t7384, t1580, t28447, t110502, t25387, t103449, t103462, t103463, t110444, t1956, t1957, t231, t233, t25317, t28394, t28442, t30341, t30379, t4534, t7070, t7076, t836, t886, t95888, t95891, t95893, t99191);
        let t110635 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2029::<F>(t18797, t26497, t110322, t25375, t103467, t103471, t103490, t103494, t1558, t231, t25317, t25383, t27199, t28340, t28378, t30379, t30396, t30401, t4533, t7070, t7071, t7076, t8006, t886, t95899, t95902, t95905);
        let t110665 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2030::<F>(t18805, t95936, t103037, t103424, t106143, t106360, t106365, t110493, t14495, t231, t25391, t26547, t26550, t27199, t27353, t28405, t6072, t7070, t7076, t93349, t95911, t95914, t95925, t95927, t95930);
    (t110466, t110499, t110519, t110551, t110576, t110607, t110635, t110665)
}
