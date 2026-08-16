//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta708 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2541;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2542;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta708<F: Float>(t2841: F, t4351: F, t10701: F, t1543: F, t10810: F, t1561: F, t14363: F, t942: F, t2929: F, t4446: F, t1568: F, t2886: F, t2860: F, t4408: F, t10770: F, t10811: F, t14255: F, t892: F, t2791: F, t10660: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t49269, t49274, t49285, t49404, t49411, t49422) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2541::<F>(t2841, t4351, t10701, t1543, t10810, t1561, t14363, t942, t2929, t4446, t1568, t2886);
        let (t49427, t49430, t49478, t49483, t49486, t49489) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2542::<F>(t2860, t4408, t10770, t1561, t10811, t1568, t14255, t892, t2791, t4351, t10660, t1543);
    (t49269, t49274, t49285, t49404, t49411, t49422, t49427, t49430, t49478, t49483, t49486, t49489)
}
