//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta817 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2879;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2880;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta817(t4370: f64, t2798: f64, t17292: f64, t699: f64, t136: f64, t59682: f64, t908: f64, t2403: f64, t5720: f64, t59690: f64, t5723: f64, t60149: f64, t894: f64, t48155: f64, t48157: f64, t48159: f64, t48161: f64, t48163: f64, t48165: f64, t48167: f64, t59657: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t60160, t60161, t60163, t60166, t60168, t60171, t60173, t60176) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2879(t4370, t2798, t17292, t699, t136, t59682, t908, t2403, t5720, t59690, t5723, t60149, t894);
        let t60185 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2880(t48155, t48157, t48159, t48161, t48163, t48165, t48167, t59657, t60161, t60163, t60166, t60168, t60171, t60173, t60176);
    (t60160, t60161, t60163, t60166, t60168, t60171, t60173, t60176, t60185)
}
