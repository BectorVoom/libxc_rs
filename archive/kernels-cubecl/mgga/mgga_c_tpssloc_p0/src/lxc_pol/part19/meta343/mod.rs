//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta343 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1228;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1229;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta343<F: Float>(t2427: F, t9909: F, t39568: F, t761: F, t2535: F, t9716: F, t39382: F, t2531: F, t9713: F, t39302: F, t39563: F, t39585: F, t39590: F, t39593: F, t40818: F, t172: F, t763: F, t9915: F, t184: F, t4194: F, t607: F, t9258: F, t12939: F, t2244: F, t9681: F, t2371: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t41252, t41254, t41256, t41258, t41260, t41262, t41263) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1228::<F>(t2427, t9909, t39568, t761, t2535, t9716, t39382, t2531, t9713, t39302, t39563, t39585, t39590, t39593, t40818);
        let (t41266, t41270, t41273, t41274) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1229::<F>(t172, t763, t9915, t184, t4194, t607, t9258, t12939, t2244, t9681, t2371, t9716);
    (t41252, t41254, t41256, t41258, t41260, t41262, t41263, t41266, t41270, t41273, t41274)
}
