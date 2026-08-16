//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta634 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2170;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2171;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta634<F: Float>(t212: F, t5187: F, t12225: F, t2586: F, t16100: F, t782: F, t16093: F, t16097: F, t2566: F, t2559: F, t5194: F, t5198: F, t12214: F, t67: F, t792: F, t133: F, t1799: F, t40369: F, t6600: F, t131: F, t205: F, t40024: F, t1336: F, t242: F, t40042: F) -> (F, F, F, F, F, F, F, F) {
        let (t54668, t54670, t54676, t54701) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2170::<F>(t212, t5187, t12225, t2586, t16100, t782, t16093, t16097, t2566, t2559, t5194, t5198);
        let (t54702, t54718, t54725, t54728, t54744) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2171::<F>(t54701, t12214, t67, t792, t133, t1799, t40369, t6600, t131, t205, t40024, t1336, t242, t40042);
    (t54668, t54670, t54676, t54702, t54718, t54725, t54728, t54744)
}
