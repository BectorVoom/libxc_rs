//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta447 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1689;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1690;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1691;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1692;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1693;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1694;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta447(t17240: f64, t5052: f64, t1222: f64, t16738: f64, t5308: f64, t16742: f64, t16733: f64, t16771: f64, t247: f64, t3719: f64, t3636: f64, t5391: f64, t5381: f64, t1260: f64, t12966: f64, t16775: f64, t1261: f64, t17232: f64, t17237: f64, t5384: f64, t5386: f64, t16750: f64, t482: f64, t371: f64, t372: f64, t1803: f64, t3666: f64, t1208: f64, t5215: f64, t225: f64, t480: f64, t3678: f64, t5327: f64, t5323: f64, t1235: f64, t1238: f64, t12800: f64, t12976: f64, t1791: f64, t1808: f64, t3644: f64, t3663: f64, t3667: f64, t5320: f64, t5362: f64, t1789: f64, t676: f64, t1769: f64, t3565: f64, t3650: f64, t16708: f64, t16710: f64, t16712: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12678: f64, t16706: f64, t16717: f64, t16722: f64, t16727: f64, t16731: f64, t16735: f64, t16740: f64, t16744: f64, t16748: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17243, t17244, t17247, t17250, t17254, t17258) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1689(t17240, t5052, t1222, t16738, t5308, t16742, t16733, t16771, t247, t3719, t3636, t5391);
        let t17268 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1690(t3636, t5381, t1260, t12966, t16775, t247, t3719, t1222, t1261, t17232, t17237, t17243, t17244, t17247, t17250, t17254, t17258, t5384, t5386);
        let (t17280, t17283, t17288, t17289, t17290, t17296) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1691(t16750, t482, t371, t372, t1803, t3666, t1208, t5215, t225, t480, t3678, t5327);
        let t17299 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1692(t3678, t5323, t1235, t1238, t12800, t12976, t17280, t17283, t17290, t17296, t1791, t1808, t3644, t3663, t3667, t5320, t5327, t5391);
        let (t17301, t17304, t17306, t17307, t17308, t17311) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1693(t3667, t5362, t1789, t371, t676, t1235, t1769, t3565, t225, t480, t1803, t3650);
        let t17330 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1694(t16708, t16710, t16712, t12297, t12299, t12301, t12303, t12678, t16706, t16717, t16722, t16727, t16731, t16735, t16740, t16744, t16748);
    (t17268, t17288, t17289, t17299, t17301, t17304, t17306, t17307, t17308, t17311, t17330)
}
