//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta445 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1992;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta445<F: Float>(t15621: F, t4582: F, t11721: F, t3507: F, t4977: F, t3509: F, t1216: F, t15553: F, t13969: F, t4979: F, t3506: F, t4973: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t15622, t15625, t15626, t15627, t15630, t15631, t15636, t15637, t15640, t15642, t15643) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1992::<F>(t15621, t4582, t11721, t3507, t4977, t3509, t1216, t15553, t13969, t4979, t3506, t4973);
    (t15622, t15625, t15626, t15627, t15630, t15631, t15636, t15637, t15640, t15642, t15643)
}
