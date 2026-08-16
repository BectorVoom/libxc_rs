//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta253 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk914;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta253<F: Float>(t1193: F, t6109: F, t248: F, t3570: F, t6230: F, t3515: F, t1243: F, t19045: F, t225: F, t6151: F, t6153: F, t6239: F) -> (F, F, F, F, F, F, F) {
        let (t19090, t19095, t19096, t19201, t19232, t19234, t19249) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk914::<F>(t1193, t6109, t248, t3570, t6230, t3515, t1243, t19045, t225, t6151, t6153, t6239);
    (t19090, t19095, t19096, t19201, t19232, t19234, t19249)
}
