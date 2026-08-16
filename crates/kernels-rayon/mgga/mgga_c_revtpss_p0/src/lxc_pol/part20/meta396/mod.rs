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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1457;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1458;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1459;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1460;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1461;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1462;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1463;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1464;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta396(t11289: f64, t2919: f64, t2866: f64, t2923: f64, t2927: f64, t11380: f64, t2869: f64, t11384: f64, t910: f64, t11388: f64, t275: f64, t2872: f64, t2922: f64, t11387: f64, t41500: f64, t41245: f64, t41250: f64, t41255: f64, t41260: f64, t41265: f64, t41267: f64, t41273: f64, t41275: f64, t41279: f64, t41281: f64, t41283: f64, t41285: f64, t41287: f64, t41289: f64, t41306: f64, t41292: f64, t41299: f64, t41303: f64, t41341: f64, t41344: f64, t41347: f64, t41350: f64, t41361: f64, t41363: f64, t41369: f64, t41373: f64, t41384: f64, t41387: f64, t41316: f64, t41323: f64, t41353: f64, t41356: f64, t41359: f64, t41396: f64, t41402: f64, t41404: f64, t41406: f64, t41409: f64, t41412: f64, t41414: f64, t41417: f64, t41419: f64, t41308: f64, t41312: f64, t41320: f64, t41327: f64, t41330: f64, t41332: f64, t41334: f64, t41336: f64, t41365: f64, t41367: f64, t41433: f64, t41436: f64, t41439: f64, t41441: f64, t915: f64, t935: f64, t315: f64, t41235: f64, t11449: f64, t941: f64, t2941: f64, t2966: f64, t302: f64, t2944: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41577, t41580, t41582, t41585, t41588) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1457(t11289, t2919, t2866, t2923, t2927, t11380, t2869, t11384, t910, t11388, t275, t2872, t2922);
        let (t41591, t41606) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1458(t11387, t41500, t41588, t41245, t41250, t41255, t41260, t41265, t41267, t41273, t41275, t41279, t41281, t41283, t41285, t41287, t41289);
        let t41621 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1459(t41306, t41292, t41299, t41303, t41341, t41344, t41347, t41350, t41361, t41363, t41369, t41373, t41384, t41387);
        let t41637 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1460(t41316, t41323, t41353, t41356, t41359, t41396, t41402, t41404, t41406, t41409, t41412, t41414, t41417, t41419);
        let t41652 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1461(t41308, t41312, t41320, t41327, t41330, t41332, t41334, t41336, t41365, t41367, t41433, t41436, t41439, t41441);
        let (t41657, t41658, t41662, t41667) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1462(t41606, t41621, t41637, t41652, t915, t935, t315, t41235, t11449, t941, t2941, t2966, t302);
        let (t41668, t41686) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1463(t2944, t41245, t41250, t41255, t41260, t41265, t41267, t41273, t41275, t41279, t41281, t41283, t41285, t41287, t41289);
        let t41701 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1464(t41306, t41292, t41299, t41303, t41341, t41344, t41347, t41350, t41361, t41363, t41369, t41373, t41384, t41387);
    (t41577, t41580, t41582, t41585, t41591, t41657, t41658, t41662, t41667, t41668, t41686, t41701)
}
