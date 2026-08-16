//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta440 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1984;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta440<F: Float>(t11539: F, t4724: F, t1174: F, t15239: F, t475: F, t1214: F, t248: F, t3494: F, t4977: F, t4582: F, t3516: F, t12652: F, t4987: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t15522, t15524, t15525, t15527, t15530, t15531, t15534, t15535, t15540) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1984::<F>(t11539, t4724, t1174, t15239, t475, t1214, t248, t3494, t4977, t4582, t3516, t12652, t4987);
    (t15522, t15524, t15525, t15527, t15530, t15531, t15534, t15535, t15540)
}
