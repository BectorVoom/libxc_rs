//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta942 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3093;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3094;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3095;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta942(t1196: f64, t20891: f64, t24375: f64, t43752: f64, t16840: f64, t20574: f64, t17092: f64, t20577: f64, t1149: f64, t12248: f64, t24221: f64, t3433: f64, t5104: f64, t6439: f64, t12361: f64, t24212: f64, t3384: f64, t5105: f64, t6470: f64, t24765: f64, t3531: f64, t16988: f64, t20472: f64, t1733: f64, t20447: f64, t12243: f64, t24215: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81589, t81591, t81593, t81596, t81599) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3093(t1196, t20891, t24375, t43752, t16840, t20574, t17092, t20577, t1149, t12248, t24221, t3433, t5104, t6439);
        let (t81601, t81604, t81606, t81609, t81612) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3094(t12361, t24212, t3384, t5105, t6470, t24765, t3531, t1196, t16988, t20472, t1733, t20447);
        let (t81614, t81615) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3095(t12243, t24215, t81589, t81591, t81593, t81596, t81599, t81601, t81604, t81606, t81609, t81612);
    (t81589, t81591, t81593, t81596, t81599, t81601, t81604, t81606, t81609, t81612, t81614, t81615)
}
