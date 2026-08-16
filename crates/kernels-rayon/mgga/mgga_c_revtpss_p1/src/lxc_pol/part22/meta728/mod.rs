//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta728 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2784;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2785;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta728(t2394: f64, t2475: f64, t10069: f64, t10929: f64, t138: f64, t785: f64, t9302: f64, t2786: f64, t10073: f64, t10920: f64, t10871: f64, t2645: f64, t234: f64, t39545: f64, t685: f64, t875: f64, t2760: f64, t2783: f64, t786: f64, t2778: f64, t39515: f64, t39501: f64, t871: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40236, t40267, t40270, t40271, t40273, t40284) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2784(t2394, t2475, t10069, t10929, t138, t785, t9302, t2786, t10073, t10920, t10871, t2645);
        let (t40294, t40297, t40303, t40314, t40316) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2785(t234, t39545, t685, t875, t2760, t2783, t786, t10069, t10920, t2778, t39515, t39501, t871);
    (t40236, t40267, t40270, t40271, t40273, t40284, t40294, t40297, t40303, t40314, t40316)
}
