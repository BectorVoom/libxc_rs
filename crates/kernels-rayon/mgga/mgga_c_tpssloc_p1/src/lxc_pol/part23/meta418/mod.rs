//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta418 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1238;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1239;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1240;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1241;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1242;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta418(t13360: f64, t5628: f64, t67441: f64, t842: f64, t5611: f64, t9975: f64, t21064: f64, t225: f64, t262: f64, t5527: f64, t21152: f64, t690: f64, t21155: f64, t21146: f64, t21149: f64, t21160: f64, t699: f64, t21167: f64, t21123: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t68201, t68203, t68246, t68322, t68371, t68442) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1238(t13360, t5628, t67441, t842, t5611, t9975, t21064, t225, t262, t5527, t21152, t690);
        let t68444 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1239(t21155, t690);
        let t68446 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1240(t21146, t690);
        let t68448 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1241(t21149, t690);
        let (t68452, t68454, t68494) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1242(t21160, t699, t21167, t21123, t690);
    (t68201, t68203, t68246, t68322, t68371, t68442, t68444, t68446, t68448, t68452, t68454, t68494)
}
