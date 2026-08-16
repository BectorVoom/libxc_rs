//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta551 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2096;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta551<F: Float>(t2250: F, t4194: F, t607: F, t750: F, t2617: F, t9670: F, t831: F, t236: F, t40931: F, t2638: F, t9612: F, t10021: F, t812: F, t815: F) -> (F, F, F, F, F, F, F) {
        let (t41295, t41340, t41341, t41347, t41354, t41355, t41362) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2096::<F>(t2250, t4194, t607, t750, t2617, t9670, t831, t236, t40931, t2638, t9612, t10021, t812, t815);
    (t41295, t41340, t41341, t41347, t41354, t41355, t41362)
}
