//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta201 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1272;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1273;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1274;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1275;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1276;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1277;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1278;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1279;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1280;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1281;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta201(t1066: f64, t4583: f64, t247: f64, t1062: f64, t1659: f64, t3204: f64, t3116: f64, t4757: f64, t127: f64, t1663: f64, t371: f64, t1025: f64, t1063: f64, t1068: f64, t1675: f64, t3106: f64, t3112: f64, t3127: f64, t3174: f64, t3188: f64, t4818: f64, t4821: f64, t4825: f64, t373: f64, t4772: f64, t372: f64, t225: f64, t4746: f64, t366: f64, t4589: f64, t4592: f64, t4594: f64, t4597: f64, t4634: f64, t4638: f64, t4716: f64, t4718: f64, t4721: f64, t4723: f64, t4727: f64, t4731: f64, t4736: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t4831 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1272(t1066, t4583, t247);
        let t4834 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1273(t1062, t1659);
        let t4837 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1274(t1062, t3204);
        let t4839 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1275(t3116, t4757, t247);
        let t4845 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1276(t127, t1663, t371);
        let (t4846, t4848) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1277(t1025, t4845, t1063, t1068, t1675, t3106, t3112, t3127, t3174, t3188, t4818, t4821, t4825, t4831, t4834, t4837, t4839);
        let (t4852, t4854) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1278(t373, t4772, t371, t372);
        let t4857 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1279(t225, t4746);
        let t4858 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1280(t366, t4857);
        let t4866 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1281(t4589, t4592, t4594, t4597, t4634, t4638, t4716, t4718, t4721, t4723, t4727, t4731, t4736);
    (t4831, t4834, t4837, t4839, t4845, t4846, t4848, t4852, t4854, t4857, t4858, t4866)
}
