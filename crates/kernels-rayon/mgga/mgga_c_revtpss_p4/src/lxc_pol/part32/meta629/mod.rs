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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta629(t106068: f64, t106070: f64, t106072: f64, t106074: f64, t95673: f64, t95674: f64, t95675: f64, t95678: f64, t95680: f64, t99044: f64, t99050: f64, t99058: f64, t99065: f64, t103315: f64, t103316: f64, t103318: f64, t103320: f64, t103324: f64, t106080: f64, t106082: f64, t106085: f64, t106088: f64, t106090: f64, t93035: f64, t95684: f64, t103336: f64, t103337: f64, t106093: f64, t106099: f64, t106102: f64, t106104: f64, t106106: f64, t93049: f64, t93067: f64, t93073: f64, t93088: f64, t99091: f64, t99113: f64, t110378: f64, t110385: f64, t110393: f64, t110406: f64, t110414: f64, t27216: f64, t28360: f64, t30384: f64, t786: f64, t789: f64, t103184: f64, t103196: f64, t103216: f64, t103219: f64, t103220: f64, t14587: f64, t1579: f64, t213: f64, t225: f64, t25317: f64, t25391: f64, t257: f64, t27199: f64, t28418: f64, t28425: f64, t30356: f64, t7070: f64, t886: f64, t95807: f64, t95808: f64, t95811: f64, t95813: f64, t30395: f64, t689: f64, t25431: f64, t25411: f64, t6072: f64, t7384: f64, t5977: f64, t7398: f64, t103037: f64, t103224: f64, t103234: f64, t103240: f64, t103364: f64, t106228: f64, t18324: f64, t25383: f64, t25416: f64, t2723: f64, t27275: f64, t27349: f64, t30392: f64, t7403: f64, t8016: f64, t93349: f64, t95836: f64, t30341: f64, t686: f64, t72: f64, t25375: f64, t28314: f64, t99463: f64, t27213: f64, t103370: f64, t103382: f64, t103391: f64, t103393: f64, t103394: f64, t103396: f64, t103399: f64, t106404: f64, t18663: f64, t2067: f64, t29682: f64, t95825: f64, t95859: f64, t95862: f64, t28368: f64, t99404: f64, t98849: f64, t30405: f64, t103400: f64, t103404: f64, t103422: f64, t103424: f64, t106290: f64, t106410: f64, t26550: f64, t27353: f64, t28385: f64, t30337: f64, t62624: f64, t8012: f64, t92917: f64, t99303: f64, t110275: f64, t93281: f64, t103432: f64, t103435: f64, t103437: f64, t103441: f64, t103444: f64, t106172: f64, t106302: f64, t18313: f64, t18785: f64, t26547: f64, t28411: f64, t28426: f64, t28439: f64, t30381: f64, t30410: f64, t6049: f64, t7067: f64, t93118: f64, t1580: f64, t28447: f64, t25387: f64, t103449: f64, t103462: f64, t103463: f64, t1956: f64, t1957: f64, t231: f64, t233: f64, t28394: f64, t28442: f64, t30379: f64, t4534: f64, t7076: f64, t836: f64, t95888: f64, t95891: f64, t95893: f64, t99191: f64, t18797: f64, t26497: f64, t110322: f64, t103467: f64, t103471: f64, t103490: f64, t103494: f64, t1558: f64, t28340: f64, t28378: f64, t30396: f64, t30401: f64, t4533: f64, t7071: f64, t8006: f64, t95899: f64, t95902: f64, t95905: f64, t18805: f64, t95936: f64, t106143: f64, t106360: f64, t106365: f64, t14495: f64, t28405: f64, t95911: f64, t95914: f64, t95925: f64, t95927: f64, t95930: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t110421 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2019(t106068, t106070, t106072, t106074, t95673, t95674, t95675, t95678, t95680, t99044, t99050, t99058, t99065);
        let t110429 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2020(t103315, t103316, t103318, t103320, t103324, t106080, t106082, t106085, t106088, t106090, t93035, t95684);
        let t110441 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2021(t103336, t103337, t106093, t106099, t106102, t106104, t106106, t93049, t93067, t93073, t93088, t99091, t99113);
        let (t110444, t110453, t110459) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2022(t110378, t110385, t110393, t110406, t110414, t110421, t110429, t110441, t27216, t28360, t30384, t786, t789);
        let t110466 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2023(t103184, t103196, t103216, t103219, t103220, t110444, t110453, t110459, t14587, t1579, t213, t225, t25317, t25391, t257, t27199, t28418, t28425, t30356, t7070, t886, t95807, t95808, t95811, t95813);
        let (t110493, t110499) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2024(t30395, t689, t25431, t25411, t6072, t7384, t5977, t7398, t103037, t103224, t103234, t103240, t103364, t106228, t18324, t25383, t25391, t25416, t2723, t27275, t27349, t28425, t30392, t7070, t7403, t8016, t93349, t95836);
        let (t110502, t110519) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2025(t30341, t686, t72, t25375, t28314, t99463, t27213, t28360, t103370, t103382, t103391, t103393, t103394, t103396, t103399, t106404, t18663, t2067, t25391, t29682, t7403, t95825, t95859, t95862);
        let t110551 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2026(t28368, t99404, t98849, t30405, t689, t25431, t25411, t103400, t103404, t103422, t103424, t106290, t106410, t25391, t26550, t27199, t27349, t27353, t28385, t30337, t62624, t8012, t92917, t99303);
        let t110576 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2027(t110275, t93281, t103432, t103435, t103437, t103441, t103444, t106172, t106302, t18313, t18785, t25391, t26547, t26550, t27199, t28411, t28426, t28439, t30381, t30410, t6049, t7067, t7070, t7403, t886, t93118);
        let t110607 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2028(t6049, t689, t7384, t1580, t28447, t110502, t25387, t103449, t103462, t103463, t110444, t1956, t1957, t231, t233, t25317, t28394, t28442, t30341, t30379, t4534, t7070, t7076, t836, t886, t95888, t95891, t95893, t99191);
        let t110635 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2029(t18797, t26497, t110322, t25375, t103467, t103471, t103490, t103494, t1558, t231, t25317, t25383, t27199, t28340, t28378, t30379, t30396, t30401, t4533, t7070, t7071, t7076, t8006, t886, t95899, t95902, t95905);
        let t110665 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2030(t18805, t95936, t103037, t103424, t106143, t106360, t106365, t110493, t14495, t231, t25391, t26547, t26550, t27199, t27353, t28405, t6072, t7070, t7076, t93349, t95911, t95914, t95925, t95927, t95930);
    (t110466, t110499, t110519, t110551, t110576, t110607, t110635, t110665)
}
