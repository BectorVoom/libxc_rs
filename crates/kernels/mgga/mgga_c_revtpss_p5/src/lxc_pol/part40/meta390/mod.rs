//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta390 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1410;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1411;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1412;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1413;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1414;
use chunk5::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1415;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta390<F: Float>(t17240: F, t5052: F, t1222: F, t16738: F, t5308: F, t16742: F, t16733: F, t16771: F, t247: F, t3719: F, t3636: F, t5391: F, t5381: F, t1260: F, t12966: F, t16775: F, t1261: F, t17232: F, t17237: F, t5384: F, t5386: F, t16750: F, t482: F, t371: F, t372: F, t1803: F, t3666: F, t1208: F, t5215: F, t225: F, t480: F, t3678: F, t5327: F, t5323: F, t1235: F, t1238: F, t12800: F, t12976: F, t1791: F, t1808: F, t3644: F, t3663: F, t3667: F, t5320: F, t5362: F, t1789: F, t676: F, t1769: F, t3565: F, t3650: F, t16708: F, t16710: F, t16712: F, t12297: F, t12299: F, t12301: F, t12303: F, t12678: F, t16706: F, t16717: F, t16722: F, t16727: F, t16731: F, t16735: F, t16740: F, t16744: F, t16748: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17243, t17244, t17247, t17250, t17254, t17258) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1410::<F>(t17240, t5052, t1222, t16738, t5308, t16742, t16733, t16771, t247, t3719, t3636, t5391);
        let t17268 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1411::<F>(t3636, t5381, t1260, t12966, t16775, t247, t3719, t1222, t1261, t17232, t17237, t17243, t17244, t17247, t17250, t17254, t17258, t5384, t5386);
        let (t17280, t17283, t17288, t17289, t17290, t17296) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1412::<F>(t16750, t482, t371, t372, t1803, t3666, t1208, t5215, t225, t480, t3678, t5327);
        let t17299 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1413::<F>(t3678, t5323, t1235, t1238, t12800, t12976, t17280, t17283, t17290, t17296, t1791, t1808, t3644, t3663, t3667, t5320, t5327, t5391);
        let (t17301, t17304, t17306, t17307, t17308, t17311) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1414::<F>(t3667, t5362, t1789, t371, t676, t1235, t1769, t3565, t225, t480, t1803, t3650);
        let t17330 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1415::<F>(t16708, t16710, t16712, t12297, t12299, t12301, t12303, t12678, t16706, t16717, t16722, t16727, t16731, t16735, t16740, t16744, t16748);
    (t17268, t17288, t17289, t17299, t17301, t17304, t17306, t17307, t17308, t17311, t17330)
}
