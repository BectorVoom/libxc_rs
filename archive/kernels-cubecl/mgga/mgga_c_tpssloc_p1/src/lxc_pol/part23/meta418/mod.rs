//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta418 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1238;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1239;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1240;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1241;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1242;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta418<F: Float>(t13360: F, t5628: F, t67441: F, t842: F, t5611: F, t9975: F, t21064: F, t225: F, t262: F, t5527: F, t21152: F, t690: F, t21155: F, t21146: F, t21149: F, t21160: F, t699: F, t21167: F, t21123: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t68201, t68203, t68246, t68322, t68371, t68442) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1238::<F>(t13360, t5628, t67441, t842, t5611, t9975, t21064, t225, t262, t5527, t21152, t690);
        let t68444 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1239::<F>(t21155, t690);
        let t68446 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1240::<F>(t21146, t690);
        let t68448 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1241::<F>(t21149, t690);
        let (t68452, t68454, t68494) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1242::<F>(t21160, t699, t21167, t21123, t690);
    (t68201, t68203, t68246, t68322, t68371, t68442, t68444, t68446, t68448, t68452, t68454, t68494)
}
