//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta565 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2124;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta565<F: Float>(t10401: F, t10935: F, t3186: F, t3200: F, t11051: F, t3069: F, t10454: F, t3048: F, t10459: F, t3036: F, t3087: F, t3033: F, t3128: F) -> (F, F, F, F, F, F, F) {
        let (t42505, t42508, t42511, t42514, t42518, t42520, t42522) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2124::<F>(t10401, t10935, t3186, t3200, t11051, t3069, t10454, t3048, t10459, t3036, t3087, t3033, t3128);
    (t42505, t42508, t42511, t42514, t42518, t42520, t42522)
}
