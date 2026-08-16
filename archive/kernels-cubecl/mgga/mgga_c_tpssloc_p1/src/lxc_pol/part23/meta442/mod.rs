//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta442 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1286;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta442<F: Float>(t16288: F, t6417: F, t12385: F, t20497: F, t20433: F, t3866: F, t16336: F, t6431: F, t1831: F, t57021: F, t53945: F, t6396: F) -> (F, F, F, F, F, F) {
        let (t74217, t74228, t74256, t74258, t74260, t74274) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1286::<F>(t16288, t6417, t12385, t20497, t20433, t3866, t16336, t6431, t1831, t57021, t53945, t6396);
    (t74217, t74228, t74256, t74258, t74260, t74274)
}
