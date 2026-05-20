//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta608 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2072;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2073;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2074;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta608<F: Float>(t28177: F, t7235: F, t28056: F, t4254: F, t5517: F, t651: F, t7002: F, t2028: F, t27980: F, t13790: F, t4102: F, t685: F, t72: F, t25875: F, t1444: F, t5740: F, t675: F, t94395: F, t14109: F, t25900: F, t94649: F, t1892: F, t786: F, t25877: F, t25881: F, t25931: F, t14224: F, t689: F, t25894: F, t25921: F, t25924: F, t25966: F, t26046: F, t27837: F, t27841: F, t4131: F, t7295: F, t7920: F, t94378: F, t94388: F, t94392: F, t94399: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t97661, t97663, t97666, t97676, t97680) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2072::<F>(t28177, t7235, t28056, t4254, t5517, t651, t7002, t2028, t27980, t13790, t4102, t685, t72);
        let (t97682, t97685, t97687, t97690, t97698, t97699) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2073::<F>(t25875, t97676, t97680, t1444, t5740, t675, t685, t94395, t14109, t25900, t94649, t1892, t786);
        let (t97703, t97705, t97716) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2074::<F>(t25877, t97699, t25881, t2028, t25931, t14224, t689, t25894, t25921, t25924, t25966, t26046, t27837, t27841, t4131, t7295, t7920, t94378, t94388, t94392, t94399, t97682, t97687, t97690, t97698);
    (t97661, t97663, t97666, t97676, t97680, t97685, t97699, t97703, t97705, t97716)
}
