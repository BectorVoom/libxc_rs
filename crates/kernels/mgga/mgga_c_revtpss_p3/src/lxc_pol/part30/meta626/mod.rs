//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta626 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2168;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2169;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2170;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2171;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2172;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2173;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta626<F: Float>(t10073: F, t25403: F, t27198: F, t14939: F, t1955: F, t99270: F, t2471: F, t27202: F, t1559: F, t2828: F, t15003: F, t93194: F, t1579: F, t2722: F, t231: F, t27266: F, t686: F, t72: F, t7058: F, t1959: F, t25391: F, t25392: F, t27353: F, t51436: F, t51698: F, t7079: F, t93242: F, t93252: F, t93262: F, t93272: F, t93273: F, t7759: F, t822: F, t25310: F, t27279: F, t27186: F, t93321: F, t93374: F, t122: F, t2466: F, t25387: F, t4533: F, t836: F, t2470: F, t27340: F, t1580: F, t25317: F, t25394: F, t27316: F, t27349: F, t7070: F, t886: F, t92864: F, t93186: F, t93276: F, t93278: F, t93283: F, t93286: F, t2723: F, t7063: F, t99271: F, t7060: F, t136: F, t2457: F, t7778: F, t25299: F, t25412: F, t25431: F, t1568: F, t786: F, t25410: F, t25413: F, t14587: F, t25383: F, t2718: F, t27189: F, t27287: F, t27292: F, t27300: F, t27312: F, t27313: F, t27357: F, t2829: F, t51574: F, t7048: F, t92917: F, t93297: F, t93304: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t99297, t99300, t99303, t99307, t99309, t99313) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2168::<F>(t10073, t25403, t27198, t14939, t1955, t99270, t2471, t27202, t1559, t2828, t15003, t93194);
        let (t99315, t99321, t99332) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2169::<F>(t1579, t2722, t231, t27266, t686, t72, t7058, t1959, t25391, t25392, t27353, t51436, t51698, t7079, t93242, t93252, t93262, t93272, t93273, t99297, t99300, t99303, t99307, t99309, t99313);
        let (t99334, t99342, t99344, t99346, t99348, t99349, t99351, t99360) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2170::<F>(t7759, t822, t25310, t27279, t27186, t93321, t93374, t122, t72, t2466, t25387, t231, t4533, t836);
        let (t99365, t99368) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2171::<F>(t2470, t27340, t25387, t1580, t25317, t25391, t25392, t25394, t27316, t27349, t7070, t886, t92864, t93186, t93276, t93278, t93283, t93286, t99334, t99342, t99344, t99346, t99351, t99360);
        let (t99369, t99375, t99380, t99381, t99389, t99391) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2172::<F>(t2723, t99315, t7063, t99271, t7060, t136, t2457, t7778, t25299, t25412, t99348, t25431);
        let (t99403, t99409) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2173::<F>(t1568, t786, t25410, t25413, t14587, t25383, t25391, t2718, t27189, t27287, t27292, t27300, t27312, t27313, t27353, t27357, t2829, t51574, t7048, t92864, t92917, t93297, t93304, t99369, t99375, t99381, t99391);
    (t99303, t99321, t99332, t99349, t99365, t99368, t99380, t99389, t99403, t99409)
}
