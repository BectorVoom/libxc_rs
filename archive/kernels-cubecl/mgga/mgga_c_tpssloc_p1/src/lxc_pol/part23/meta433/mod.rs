//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta433 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1271;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1272;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta433<F: Float>(t13969: F, t22270: F, t3506: F, t1227: F, t22257: F, t21769: F, t248: F, t3521: F, t22157: F, t3577: F, t45124: F, t11697: F, t22287: F, t15569: F, t18371: F, t19051: F, t4993: F, t11784: F, t21762: F, t1174: F, t135: F, t22128: F, t22132: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t72470, t72495, t72501, t72512, t72530) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1271::<F>(t13969, t22270, t3506, t1227, t22257, t21769, t248, t3521, t22157, t3577, t45124, t11697, t22287);
        let (t72542, t72556, t72560, t72597, t72600) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1272::<F>(t15569, t18371, t19051, t4993, t11784, t1227, t21762, t248, t1174, t135, t22128, t22132);
    (t72470, t72495, t72501, t72512, t72530, t72542, t72556, t72560, t72597, t72600)
}
