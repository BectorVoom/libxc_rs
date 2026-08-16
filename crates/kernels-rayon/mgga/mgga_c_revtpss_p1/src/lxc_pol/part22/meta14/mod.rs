//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta14 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk113;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk114;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk115;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk116;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk117;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk118;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk119;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk120;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta14(t241: f64, t243: f64, t137: f64, t72: f64, t125: f64, t217: f64, t222: f64, t237: f64, t225: f64, t234: f64, t213: f64, t149: f64, t191: f64, t194: f64, t198: f64, t207: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t244, t245) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk113(t241, t243, t137);
        let t246 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk114(t245, t72);
        let t247 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk115(t125, t246);
        let t251 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk116(t244, t247, t217, t222, t237);
        let t252 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk117(t225, t251);
        let (t253, t256, t257) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk118(t234, t251, t213);
        let (t258, t261, t262) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk119(t252, t257, t213);
        let t265 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk120(t149, t191, t194, t198, t207, t262);
    (t245, t246, t247, t251, t252, t253, t256, t257, t258, t261, t262, t265)
}
