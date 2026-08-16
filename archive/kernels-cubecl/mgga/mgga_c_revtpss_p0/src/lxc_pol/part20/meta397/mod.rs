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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1465;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1466;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1467;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1468;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1469;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1470;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1471;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1472;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta397<F: Float>(t41316: F, t41323: F, t41353: F, t41356: F, t41359: F, t41396: F, t41402: F, t41404: F, t41406: F, t41409: F, t41412: F, t41414: F, t41417: F, t41419: F, t41308: F, t41312: F, t41320: F, t41327: F, t41330: F, t41332: F, t41334: F, t41336: F, t41365: F, t41367: F, t41433: F, t41436: F, t41439: F, t41441: F, t2966: F, t302: F, t2969: F, t11571: F, t964: F, t2979: F, t3011: F, t11506: F, t960: F, t315: F, t41224: F, t2962: F, t2935: F, t2942: F, t11452: F, t11453: F, t11456: F, t11461: F, t11466: F, t11502: F, t11509: F, t11510: F, t11557: F, t2945: F, t2968: F, t2970: F, t2982: F, t2987: F, t3007: F, t3015: F, t41225: F, t41238: F, t41464: F, t41505: F, t41658: F, t41662: F, t41667: F, t41668: F, t41686: F, t41701: F, t946: F, t954: F, t973: F, t974: F, t11408: F, t941: F, t2986: F, t11465: F, t11585: F, t945: F, t2967: F, t3006: F, t11399: F, t11411: F, t11444: F, t11445: F, t11450: F, t11468: F, t11501: F, t11507: F, t11513: F, t11548: F, t2938: F, t2943: F, t2963: F, t2971: F, t2988: F, t2989: F, t3012: F, t3014: F, t41513: F, t955: F, t2876: F, t2918: F, t2924: F, t11385: F, t11387: F, t2875: F, t11112: F, t11528: F, t11116: F, t11294: F, t11409: F, t11525: F, t41445: F, t41570: F, t41573: F, t41577: F, t41580: F, t41582: F, t41585: F, t41591: F, t41657: F, t965: F, t972: F) -> (F, F, F, F, F, F, F, F) {
        let t41717 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1465::<F>(t41316, t41323, t41353, t41356, t41359, t41396, t41402, t41404, t41406, t41409, t41412, t41414, t41417, t41419);
        let t41732 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1466::<F>(t41308, t41312, t41320, t41327, t41330, t41332, t41334, t41336, t41365, t41367, t41433, t41436, t41439, t41441);
        let (t41740, t41742, t41746, t41751, t41756, t41759, t41763) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1467::<F>(t2966, t302, t2969, t11571, t964, t2979, t3011, t11506, t960, t315, t41224, t2962);
        let t41778 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1468::<F>(t2935, t2942, t11452, t11453, t11456, t11461, t11466, t11502, t11509, t11510, t11557, t2945, t2968, t2970, t2982, t2987, t3007, t3015, t41225, t41238, t41464, t41505, t41658, t41662, t41667, t41668, t41686, t41701, t41717, t41732, t41740, t41742, t41746, t41751, t41756, t41759, t41763, t946, t954, t973, t974);
        let (t41779, t41785, t41788, t41794, t41799, t41813) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1469::<F>(t11408, t941, t2979, t2986, t11465, t960, t11585, t945, t2935, t2967, t11509, t3006);
        let t41825 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1470::<F>(t11399, t11411, t11444, t11445, t11450, t11466, t11468, t11501, t11507, t11513, t11548, t2938, t2943, t2945, t2962, t2963, t2968, t2970, t2971, t2987, t2988, t2989, t3006, t3012, t3014, t3015, t41225, t41513, t41668, t41779, t41785, t41788, t41794, t41799, t41813, t955, t974);
        let (t41832, t41841, t41845, t41847, t41849) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1471::<F>(t11501, t3014, t2876, t2918, t2924, t11385, t11387, t2875, t11112, t11528, t11116, t11294);
        let t41853 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1472::<F>(t11409, t11461, t11525, t2962, t2971, t3012, t3014, t41445, t41464, t41570, t41573, t41577, t41580, t41582, t41585, t41591, t41657, t41832, t41841, t41845, t41847, t41849, t965, t972, t973);
    (t41763, t41778, t41825, t41841, t41845, t41847, t41849, t41853)
}
