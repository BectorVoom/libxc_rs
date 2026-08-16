//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta357 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1154;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta357<F: Float>(t273: F, t41654: F, t242: F, t281: F, t283: F, t275: F, t2790: F, t2840: F, t2843: F, t2928: F, t315: F, t2931: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t41942, t41959, t41961, t41962, t42028, t42086, t42087, t42100, t42102, t42110, t42111, t42112) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1154::<F>(t273, t41654, t242, t281, t283, t275, t2790, t2840, t2843, t2928, t315, t2931);
    (t41942, t41959, t41961, t41962, t42028, t42086, t42087, t42100, t42102, t42110, t42111, t42112)
}
