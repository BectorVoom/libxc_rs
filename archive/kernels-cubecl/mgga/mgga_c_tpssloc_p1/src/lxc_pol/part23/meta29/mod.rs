//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta29 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk219;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk220;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk221;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk222;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta29<F: Float>(t144: F, t193: F, t523: F, t525: F, t533: F, t571: F, t113: F, t510: F, t513: F, t111: F, t112: F, t11: F, t2: F, t10: F, t3: F, t9: F, t16: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t574 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk219::<F>(t144, t193, t523, t525, t533, t571);
        let t576 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk220::<F>(t113, t510, t513, t574);
        let t577 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk221::<F>(t111, t112);
        let (t580, t581, t582, t583, t584, t586, t587, t588) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk222::<F>(t576, t577, t11, t2, t10, t3, t9, t16);
    (t574, t576, t577, t580, t581, t582, t583, t584, t586, t587, t588)
}
