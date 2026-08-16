//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta8 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk62;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk63;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk64;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk65;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk66;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk67;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk68;
use chunk7::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk69;
use chunk8::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk70;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta8(t61: f64, t119: f64, t133: f64, t123: f64, t126: f64, t129: f64, t125: f64, t32: f64, t40: f64, zeta_threshold: f64, t74: f64, t52: f64, t77: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t134 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk62(t61);
        let t135 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk63(t119, t134);
        let t136 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk64(t133, t135);
        let (t138, t141, t142) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk65(t123, t126, t129, t136);
        let t144 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk66(t125, t142);
        let t145 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk67(t32);
        let (t147, t148) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk68(t40, zeta_threshold);
        let t152 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk69(t40, t148, t74, t52, t77, zeta_threshold);
        let t153 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk70(t145, t152);
    (t134, t135, t136, t138, t141, t142, t144, t145, t147, t148, t152, t153)
}
