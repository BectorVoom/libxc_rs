//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta397 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1465;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1466;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1467;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1468;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1469;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1470;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1471;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1472;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta397(t41316: f64, t41323: f64, t41353: f64, t41356: f64, t41359: f64, t41396: f64, t41402: f64, t41404: f64, t41406: f64, t41409: f64, t41412: f64, t41414: f64, t41417: f64, t41419: f64, t41308: f64, t41312: f64, t41320: f64, t41327: f64, t41330: f64, t41332: f64, t41334: f64, t41336: f64, t41365: f64, t41367: f64, t41433: f64, t41436: f64, t41439: f64, t41441: f64, t2966: f64, t302: f64, t2969: f64, t11571: f64, t964: f64, t2979: f64, t3011: f64, t11506: f64, t960: f64, t315: f64, t41224: f64, t2962: f64, t2935: f64, t2942: f64, t11452: f64, t11453: f64, t11456: f64, t11461: f64, t11466: f64, t11502: f64, t11509: f64, t11510: f64, t11557: f64, t2945: f64, t2968: f64, t2970: f64, t2982: f64, t2987: f64, t3007: f64, t3015: f64, t41225: f64, t41238: f64, t41464: f64, t41505: f64, t41658: f64, t41662: f64, t41667: f64, t41668: f64, t41686: f64, t41701: f64, t946: f64, t954: f64, t973: f64, t974: f64, t11408: f64, t941: f64, t2986: f64, t11465: f64, t11585: f64, t945: f64, t2967: f64, t3006: f64, t11399: f64, t11411: f64, t11444: f64, t11445: f64, t11450: f64, t11468: f64, t11501: f64, t11507: f64, t11513: f64, t11548: f64, t2938: f64, t2943: f64, t2963: f64, t2971: f64, t2988: f64, t2989: f64, t3012: f64, t3014: f64, t41513: f64, t955: f64, t2876: f64, t2918: f64, t2924: f64, t11385: f64, t11387: f64, t2875: f64, t11112: f64, t11528: f64, t11116: f64, t11294: f64, t11409: f64, t11525: f64, t41445: f64, t41570: f64, t41573: f64, t41577: f64, t41580: f64, t41582: f64, t41585: f64, t41591: f64, t41657: f64, t965: f64, t972: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t41717 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1465(t41316, t41323, t41353, t41356, t41359, t41396, t41402, t41404, t41406, t41409, t41412, t41414, t41417, t41419);
        let t41732 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1466(t41308, t41312, t41320, t41327, t41330, t41332, t41334, t41336, t41365, t41367, t41433, t41436, t41439, t41441);
        let (t41740, t41742, t41746, t41751, t41756, t41759, t41763) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1467(t2966, t302, t2969, t11571, t964, t2979, t3011, t11506, t960, t315, t41224, t2962);
        let t41778 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1468(t2935, t2942, t11452, t11453, t11456, t11461, t11466, t11502, t11509, t11510, t11557, t2945, t2968, t2970, t2982, t2987, t3007, t3015, t41225, t41238, t41464, t41505, t41658, t41662, t41667, t41668, t41686, t41701, t41717, t41732, t41740, t41742, t41746, t41751, t41756, t41759, t41763, t946, t954, t973, t974);
        let (t41779, t41785, t41788, t41794, t41799, t41813) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1469(t11408, t941, t2979, t2986, t11465, t960, t11585, t945, t2935, t2967, t11509, t3006);
        let t41825 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1470(t11399, t11411, t11444, t11445, t11450, t11466, t11468, t11501, t11507, t11513, t11548, t2938, t2943, t2945, t2962, t2963, t2968, t2970, t2971, t2987, t2988, t2989, t3006, t3012, t3014, t3015, t41225, t41513, t41668, t41779, t41785, t41788, t41794, t41799, t41813, t955, t974);
        let (t41832, t41841, t41845, t41847, t41849) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1471(t11501, t3014, t2876, t2918, t2924, t11385, t11387, t2875, t11112, t11528, t11116, t11294);
        let t41853 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1472(t11409, t11461, t11525, t2962, t2971, t3012, t3014, t41445, t41464, t41570, t41573, t41577, t41580, t41582, t41585, t41591, t41657, t41832, t41841, t41845, t41847, t41849, t965, t972, t973);
    (t41763, t41778, t41825, t41841, t41845, t41847, t41849, t41853)
}
