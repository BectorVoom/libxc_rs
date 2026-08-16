//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta116 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk784;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk785;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk786;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk787;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk788;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk789;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta116<F: Float>(t3236: F, t1143: F, t1147: F, t1146: F, t445: F, t440: F, t3293: F, t448: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3363, t3371, t3374, t3375) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk784::<F>(t3236, t1143, t1147, t1146, t445);
        let t3376 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk785::<F>(t3375, t440);
        let (t3383, t3390, t3399) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk786::<F>(t3236, t3293, t1146);
        let t3400 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk787::<F>(t3399);
        let t3401 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk788::<F>(t3400, t440);
        let (t3402, t3403) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk789::<F>(t448);
    (t3363, t3371, t3374, t3375, t3376, t3383, t3390, t3399, t3400, t3401, t3402, t3403)
}
