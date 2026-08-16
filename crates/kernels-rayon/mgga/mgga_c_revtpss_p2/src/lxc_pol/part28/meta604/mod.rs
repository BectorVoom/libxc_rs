//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta604 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2084;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2085;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2086;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2087;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2088;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2089;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta604(t5517: f64, t651: f64, t7002: f64, t2028: f64, t27980: f64, t13790: f64, t4102: f64, t685: f64, t72: f64, t25875: f64, t1444: f64, t5740: f64, t675: f64, t94395: f64, t14109: f64, t25900: f64, t94649: f64, t1892: f64, t786: f64, t25877: f64, t25881: f64, t25931: f64, t14224: f64, t689: f64, t25894: f64, t25921: f64, t25924: f64, t25966: f64, t26046: f64, t27837: f64, t27841: f64, t4131: f64, t7295: f64, t7920: f64, t94378: f64, t94388: f64, t94392: f64, t94399: f64, t122: f64, t3916: f64, t7910: f64, t25895: f64, t1398: f64, t543: f64, t5774: f64, t1903: f64, t4056: f64, t25930: f64, t27903: f64, t27960: f64, t28003: f64, t7274: f64, t7296: f64, t94405: f64, t94409: f64, t94411: f64, t94580: f64, t94584: f64, t94591: f64, t2022: f64, t9990: f64, t1426: f64, t7911: f64, t3917: f64, t14230: f64, t25926: f64, t27868: f64, t27973: f64, t3999: f64, t4077: f64, t48020: f64, t48074: f64, t49393: f64, t94593: f64, t94598: f64, t94602: f64, t94605: f64, t94656: f64, t94705: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97666, t97676, t97680, t97682, t97685) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2084(t5517, t651, t7002, t2028, t27980, t13790, t4102, t685, t72, t25875, t1444, t5740, t675);
        let (t97687, t97690, t97698, t97699, t97702, t97703, t97705) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2085(t94395, t97685, t14109, t25900, t94649, t1892, t786, t25877, t25881, t2028, t25931, t14224, t689);
        let t97716 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2086(t25894, t97703, t97705, t25921, t25924, t25966, t26046, t27837, t27841, t4131, t7295, t7920, t94378, t94388, t94392, t94399, t97682, t97687, t97690, t97698, t97702);
        let (t97719, t97732, t97734, t97737, t97742) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2087(t25875, t97703, t97705, t122, t3916, t72, t7910, t25895, t1398, t543, t5774, t1903, t4056);
        let t97752 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2088(t1444, t25921, t25930, t25931, t27903, t27960, t28003, t5774, t7274, t7295, t7296, t94405, t94409, t94411, t94580, t94584, t94591, t97719, t97734, t97737, t97742);
        let t97791 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2089(t2022, t9990, t1426, t786, t7911, t3917, t14230, t25924, t25926, t27837, t27868, t27973, t27980, t3999, t4077, t4131, t48020, t48074, t49393, t7274, t7295, t7296, t7910, t7920, t94593, t94598, t94602, t94605, t94656, t94705);
    (t97666, t97676, t97680, t97685, t97699, t97716, t97732, t97752, t97791)
}
