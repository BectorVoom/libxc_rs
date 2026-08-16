//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta26 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk197;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk198;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk199;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk200;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk201;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta26<F: Float>(t28: F, t517: F, t148: F, t516: F, t157: F, zeta_threshold: F, t184: F, t25: F, t17: F, t182: F, t514: F, t194: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t518, t520, t521) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk197::<F>(t28, t517, t148, t516, t157, zeta_threshold);
        let t522 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk198::<F>(t184, t521);
        let (t523, t525, t526, t528, t531) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk199::<F>(t25, t28, t17, t522, t182, t521, t514, t194, t517, zeta_threshold);
        let t532 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk200::<F>(t531);
        let t533 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk201::<F>(t531, t532);
    (t518, t520, t521, t522, t523, t525, t526, t528, t531, t532, t533)
}
