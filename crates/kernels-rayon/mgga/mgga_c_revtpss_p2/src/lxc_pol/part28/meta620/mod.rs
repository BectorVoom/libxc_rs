//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta620 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2183;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2184;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2185;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2186;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2187;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2188;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta620(t10073: f64, t25403: f64, t27198: f64, t14939: f64, t1955: f64, t99270: f64, t2471: f64, t27202: f64, t1559: f64, t2828: f64, t15003: f64, t93194: f64, t1579: f64, t2722: f64, t231: f64, t27266: f64, t686: f64, t72: f64, t7058: f64, t1959: f64, t25391: f64, t25392: f64, t27353: f64, t51436: f64, t51698: f64, t7079: f64, t93242: f64, t93252: f64, t93262: f64, t93272: f64, t93273: f64, t7759: f64, t822: f64, t25310: f64, t27279: f64, t27186: f64, t93321: f64, t93374: f64, t122: f64, t2466: f64, t25387: f64, t4533: f64, t836: f64, t2470: f64, t27340: f64, t1580: f64, t25317: f64, t25394: f64, t27316: f64, t27349: f64, t7070: f64, t886: f64, t92864: f64, t93186: f64, t93276: f64, t93278: f64, t93283: f64, t93286: f64, t2723: f64, t7063: f64, t99271: f64, t7060: f64, t136: f64, t2457: f64, t7778: f64, t25299: f64, t25412: f64, t25431: f64, t1568: f64, t786: f64, t25410: f64, t25413: f64, t14587: f64, t25383: f64, t2718: f64, t27189: f64, t27287: f64, t27292: f64, t27300: f64, t27312: f64, t27313: f64, t27357: f64, t2829: f64, t51574: f64, t7048: f64, t92917: f64, t93297: f64, t93304: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t99297, t99300, t99303, t99307, t99309, t99313) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2183(t10073, t25403, t27198, t14939, t1955, t99270, t2471, t27202, t1559, t2828, t15003, t93194);
        let (t99315, t99321, t99332) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2184(t1579, t2722, t231, t27266, t686, t72, t7058, t1959, t25391, t25392, t27353, t51436, t51698, t7079, t93242, t93252, t93262, t93272, t93273, t99297, t99300, t99303, t99307, t99309, t99313);
        let (t99334, t99342, t99344, t99346, t99348, t99349, t99351, t99360) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2185(t7759, t822, t25310, t27279, t27186, t93321, t93374, t122, t72, t2466, t25387, t231, t4533, t836);
        let (t99365, t99368) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2186(t2470, t27340, t25387, t1580, t25317, t25391, t25392, t25394, t27316, t27349, t7070, t886, t92864, t93186, t93276, t93278, t93283, t93286, t99334, t99342, t99344, t99346, t99351, t99360);
        let (t99369, t99375, t99380, t99381, t99389, t99391) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2187(t2723, t99315, t7063, t99271, t7060, t136, t2457, t7778, t25299, t25412, t99348, t25431);
        let (t99403, t99409) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2188(t1568, t786, t25410, t25413, t14587, t25383, t25391, t2718, t27189, t27287, t27292, t27300, t27312, t27313, t27353, t27357, t2829, t51574, t7048, t92864, t92917, t93297, t93304, t99369, t99375, t99381, t99391);
    (t99303, t99321, t99332, t99349, t99365, t99368, t99380, t99389, t99403, t99409)
}
