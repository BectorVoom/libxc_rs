//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta557 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2059;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2060;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta557<F: Float>(t116: F, t786: F, t9534: F, t133: F, t6600: F, t776: F, t39568: F, t761: F, t39382: F, t2531: F, t9713: F, t39302: F, t31: F, t717: F, t607: F, t707: F, t9862: F, t2617: F, t9670: F, t9973: F, t236: F, t40931: F, t10021: F, t812: F, t815: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t41214, t41217, t41254, t41258, t41259, t41262) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2059::<F>(t116, t786, t9534, t133, t6600, t776, t39568, t761, t39382, t2531, t9713, t39302);
        let (t41284, t41291, t41340, t41344, t41347, t41362) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2060::<F>(t31, t717, t607, t707, t9862, t2617, t9670, t9973, t236, t40931, t10021, t812, t815);
    (t41214, t41217, t41254, t41258, t41259, t41262, t41284, t41291, t41340, t41344, t41347, t41362)
}
