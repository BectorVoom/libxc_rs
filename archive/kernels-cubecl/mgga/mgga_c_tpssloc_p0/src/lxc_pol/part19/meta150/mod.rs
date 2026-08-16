//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta150 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk759;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk760;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta150<F: Float>(t193: F, t533: F, t131: F, t3732: F, t205: F, t242: F, t3788: F, t1336: F, t557: F, t67: F, t246: F) -> (F, F, F, F, F, F) {
        let (t5160, t5194, t5195, t5245, t5246) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk759::<F>(t193, t533, t131, t3732, t205, t242, t3788, t1336);
        let t5248 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk760::<F>(t557, t67, t246);
    (t5160, t5194, t5195, t5245, t5246, t5248)
}
