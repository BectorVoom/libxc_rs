//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta604 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2084;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2085;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2086;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2087;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2088;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2089;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta604<F: Float>(t5517: F, t651: F, t7002: F, t2028: F, t27980: F, t13790: F, t4102: F, t685: F, t72: F, t25875: F, t1444: F, t5740: F, t675: F, t94395: F, t14109: F, t25900: F, t94649: F, t1892: F, t786: F, t25877: F, t25881: F, t25931: F, t14224: F, t689: F, t25894: F, t25921: F, t25924: F, t25966: F, t26046: F, t27837: F, t27841: F, t4131: F, t7295: F, t7920: F, t94378: F, t94388: F, t94392: F, t94399: F, t122: F, t3916: F, t7910: F, t25895: F, t1398: F, t543: F, t5774: F, t1903: F, t4056: F, t25930: F, t27903: F, t27960: F, t28003: F, t7274: F, t7296: F, t94405: F, t94409: F, t94411: F, t94580: F, t94584: F, t94591: F, t2022: F, t9990: F, t1426: F, t7911: F, t3917: F, t14230: F, t25926: F, t27868: F, t27973: F, t3999: F, t4077: F, t48020: F, t48074: F, t49393: F, t94593: F, t94598: F, t94602: F, t94605: F, t94656: F, t94705: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t97666, t97676, t97680, t97682, t97685) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2084::<F>(t5517, t651, t7002, t2028, t27980, t13790, t4102, t685, t72, t25875, t1444, t5740, t675);
        let (t97687, t97690, t97698, t97699, t97702, t97703, t97705) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2085::<F>(t94395, t97685, t14109, t25900, t94649, t1892, t786, t25877, t25881, t2028, t25931, t14224, t689);
        let t97716 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2086::<F>(t25894, t97703, t97705, t25921, t25924, t25966, t26046, t27837, t27841, t4131, t7295, t7920, t94378, t94388, t94392, t94399, t97682, t97687, t97690, t97698, t97702);
        let (t97719, t97732, t97734, t97737, t97742) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2087::<F>(t25875, t97703, t97705, t122, t3916, t72, t7910, t25895, t1398, t543, t5774, t1903, t4056);
        let t97752 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2088::<F>(t1444, t25921, t25930, t25931, t27903, t27960, t28003, t5774, t7274, t7295, t7296, t94405, t94409, t94411, t94580, t94584, t94591, t97719, t97734, t97737, t97742);
        let t97791 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2089::<F>(t2022, t9990, t1426, t786, t7911, t3917, t14230, t25924, t25926, t27837, t27868, t27973, t27980, t3999, t4077, t4131, t48020, t48074, t49393, t7274, t7295, t7296, t7910, t7920, t94593, t94598, t94602, t94605, t94656, t94705);
    (t97666, t97676, t97680, t97685, t97699, t97716, t97732, t97752, t97791)
}
