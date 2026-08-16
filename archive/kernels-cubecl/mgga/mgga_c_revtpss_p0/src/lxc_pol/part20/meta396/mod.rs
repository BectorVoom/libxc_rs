//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta396 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1457;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1458;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1459;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1460;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1461;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1462;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1463;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1464;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta396<F: Float>(t11289: F, t2919: F, t2866: F, t2923: F, t2927: F, t11380: F, t2869: F, t11384: F, t910: F, t11388: F, t275: F, t2872: F, t2922: F, t11387: F, t41500: F, t41245: F, t41250: F, t41255: F, t41260: F, t41265: F, t41267: F, t41273: F, t41275: F, t41279: F, t41281: F, t41283: F, t41285: F, t41287: F, t41289: F, t41306: F, t41292: F, t41299: F, t41303: F, t41341: F, t41344: F, t41347: F, t41350: F, t41361: F, t41363: F, t41369: F, t41373: F, t41384: F, t41387: F, t41316: F, t41323: F, t41353: F, t41356: F, t41359: F, t41396: F, t41402: F, t41404: F, t41406: F, t41409: F, t41412: F, t41414: F, t41417: F, t41419: F, t41308: F, t41312: F, t41320: F, t41327: F, t41330: F, t41332: F, t41334: F, t41336: F, t41365: F, t41367: F, t41433: F, t41436: F, t41439: F, t41441: F, t915: F, t935: F, t315: F, t41235: F, t11449: F, t941: F, t2941: F, t2966: F, t302: F, t2944: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t41577, t41580, t41582, t41585, t41588) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1457::<F>(t11289, t2919, t2866, t2923, t2927, t11380, t2869, t11384, t910, t11388, t275, t2872, t2922);
        let (t41591, t41606) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1458::<F>(t11387, t41500, t41588, t41245, t41250, t41255, t41260, t41265, t41267, t41273, t41275, t41279, t41281, t41283, t41285, t41287, t41289);
        let t41621 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1459::<F>(t41306, t41292, t41299, t41303, t41341, t41344, t41347, t41350, t41361, t41363, t41369, t41373, t41384, t41387);
        let t41637 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1460::<F>(t41316, t41323, t41353, t41356, t41359, t41396, t41402, t41404, t41406, t41409, t41412, t41414, t41417, t41419);
        let t41652 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1461::<F>(t41308, t41312, t41320, t41327, t41330, t41332, t41334, t41336, t41365, t41367, t41433, t41436, t41439, t41441);
        let (t41657, t41658, t41662, t41667) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1462::<F>(t41606, t41621, t41637, t41652, t915, t935, t315, t41235, t11449, t941, t2941, t2966, t302);
        let (t41668, t41686) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1463::<F>(t2944, t41245, t41250, t41255, t41260, t41265, t41267, t41273, t41275, t41279, t41281, t41283, t41285, t41287, t41289);
        let t41701 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1464::<F>(t41306, t41292, t41299, t41303, t41341, t41344, t41347, t41350, t41361, t41363, t41369, t41373, t41384, t41387);
    (t41577, t41580, t41582, t41585, t41591, t41657, t41658, t41662, t41667, t41668, t41686, t41701)
}
