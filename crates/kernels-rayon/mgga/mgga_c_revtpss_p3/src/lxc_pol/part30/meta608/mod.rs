//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta608 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2072;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2073;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2074;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta608(t28177: f64, t7235: f64, t28056: f64, t4254: f64, t5517: f64, t651: f64, t7002: f64, t2028: f64, t27980: f64, t13790: f64, t4102: f64, t685: f64, t72: f64, t25875: f64, t1444: f64, t5740: f64, t675: f64, t94395: f64, t14109: f64, t25900: f64, t94649: f64, t1892: f64, t786: f64, t25877: f64, t25881: f64, t25931: f64, t14224: f64, t689: f64, t25894: f64, t25921: f64, t25924: f64, t25966: f64, t26046: f64, t27837: f64, t27841: f64, t4131: f64, t7295: f64, t7920: f64, t94378: f64, t94388: f64, t94392: f64, t94399: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97661, t97663, t97666, t97676, t97680) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2072(t28177, t7235, t28056, t4254, t5517, t651, t7002, t2028, t27980, t13790, t4102, t685, t72);
        let (t97682, t97685, t97687, t97690, t97698, t97699) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2073(t25875, t97676, t97680, t1444, t5740, t675, t685, t94395, t14109, t25900, t94649, t1892, t786);
        let (t97703, t97705, t97716) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2074(t25877, t97699, t25881, t2028, t25931, t14224, t689, t25894, t25921, t25924, t25966, t26046, t27837, t27841, t4131, t7295, t7920, t94378, t94388, t94392, t94399, t97682, t97687, t97690, t97698);
    (t97661, t97663, t97666, t97676, t97680, t97685, t97699, t97703, t97705, t97716)
}
