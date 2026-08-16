//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta133 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk752;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk753;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk754;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk755;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk756;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk757;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk758;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta133(t3363: f64, t3618: f64, t247: f64, t474: f64, t479: f64, t3089: f64, t1285: f64, t1264: f64, t828: f64, t1248: f64, t73: f64, t1121: f64, t471: f64, t606: f64, t126: f64, t1263: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3620, t3623) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk752(t3363, t3618, t247, t474, t479);
        let t3624 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk753(t3089, t3623);
        let t3625 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk754(t1285, t3624);
        let t3626 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk755(t1264, t828);
        let (t3627, t3628, t3629) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk756(t1248, t73, t1121, t471, t606);
        let t3630 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk757(t3627, t3629);
        let (t3631, t3634) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk758(t3626, t3630, t126, t1263);
    (t3620, t3623, t3624, t3625, t3626, t3627, t3628, t3629, t3630, t3631, t3634)
}
