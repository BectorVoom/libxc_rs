//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta562 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2268;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta562<F: Float>(t372: F, t6163: F, t479: F, t471: F, t248: F, t3521: F, t5979: F, t1227: F, t1009: F, t6150: F, t1011: F, t1212: F) -> (F, F, F, F, F, F) {
        let (t19032, t19033, t19040, t19041, t19045, t19047) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2268::<F>(t372, t6163, t479, t471, t248, t3521, t5979, t1227, t1009, t6150, t1011, t1212);
    (t19032, t19033, t19040, t19041, t19045, t19047)
}
