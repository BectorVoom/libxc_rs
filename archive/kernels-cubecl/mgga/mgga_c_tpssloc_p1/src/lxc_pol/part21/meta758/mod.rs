//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta758 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2632;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta758<F: Float>(t5154: F, t9713: F, t9905: F, t15968: F, t67: F, t758: F, t17: F, t750: F, t2225: F, t5166: F, t15921: F, t592: F) -> (F, F, F, F, F, F) {
        let (t54389, t54392, t54395, t54398, t54400, t54402) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2632::<F>(t5154, t9713, t9905, t15968, t67, t758, t17, t750, t2225, t5166, t15921, t592);
    (t54389, t54392, t54395, t54398, t54400, t54402)
}
