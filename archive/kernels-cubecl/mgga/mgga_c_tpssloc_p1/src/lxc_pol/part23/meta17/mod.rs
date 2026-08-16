//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta17 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk133;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk134;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk135;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk136;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk137;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk138;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk139;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta17<F: Float>(t273: F, t276: F, t279: F, t285: F, t315: F, t293: F, t300: F, t302: F, t311: F, t194: F, t241: F, zeta_threshold: F, t131: F, t39: F, t271: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t320, t323, t324) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk133::<F>(t273, t276, t279, t285);
        let (t328, t330, t334, t335) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk134::<F>(t315, t324, t293, t300, t302, t311, t194, t241, zeta_threshold);
        let t336 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk135::<F>(t334, t335);
        let t337 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk136::<F>(t335);
        let t338 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk137::<F>(t131, t337);
        let t339 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk138::<F>(t338, t39);
        let t340 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk139::<F>(t271);
    (t320, t323, t324, t328, t330, t334, t335, t336, t337, t338, t339, t340)
}
