//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta224 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk928;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta224<F: Float>(t1021: F, t10877: F, t248: F, t1015: F, t10478: F, t10472: F, t10481: F, t360: F, t1030: F, t3036: F, t3033: F, t3041: F, t3101: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t10879, t10882, t10883, t10884, t10886, t10889, t10890, t10891, t10895) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk928::<F>(t1021, t10877, t248, t1015, t10478, t10472, t10481, t360, t1030, t3036, t3033, t3041, t3101);
    (t10879, t10882, t10883, t10884, t10886, t10889, t10890, t10891, t10895)
}
