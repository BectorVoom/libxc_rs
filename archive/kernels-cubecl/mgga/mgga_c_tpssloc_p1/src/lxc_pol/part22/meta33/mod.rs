//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta33 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk241;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk242;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk243;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk244;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk245;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk246;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta33<F: Float>(t5: F, t601: F, t605: F, t645: F, t86: F, t112: F, t111: F, t89: F, t107: F, t626: F, t106: F, t38: F, t606: F, tau0: F, t95: F, t103: F, t100: F, t92: F, t96: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t649, t650) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk241::<F>(t5, t601, t605, t645, t86, t112);
        let t652 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk242::<F>(t111, t89);
        let (t654, t655, t656) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk243::<F>(t107, t626, t106);
        let (t657, t659) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk244::<F>(t38, t606, tau0);
        let (t660, t662) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk245::<F>(t659, t95);
        let (t663, t666) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk246::<F>(t103, t662, t100, t657, t660, t92, t96);
    (t649, t650, t652, t654, t655, t656, t657, t659, t662, t663, t666)
}
