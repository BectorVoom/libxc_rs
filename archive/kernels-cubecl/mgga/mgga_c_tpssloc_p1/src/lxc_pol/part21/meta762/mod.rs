//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta762 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2636;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2637;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta762<F: Float>(t16235: F, t54566: F, t1811: F, t40005: F, t12283: F, t16265: F, t16257: F, t16398: F, t1358: F, t16347: F, t40281: F, t5259: F, t1336: F, t1361: F, t242: F, t12189: F, t5206: F, t40406: F, t5202: F, t16115: F, t3726: F, t12199: F, t16111: F, t1804: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t54567, t54582, t54585, t54607, t54609, t54611) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2636::<F>(t16235, t54566, t1811, t40005, t12283, t16265, t16257, t16398, t1358, t16347, t40281, t5259);
        let (t54614, t54631, t54633, t54635, t54637, t54639) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2637::<F>(t1336, t1361, t242, t12189, t5206, t40406, t5202, t16115, t3726, t12199, t16111, t1804, t40005);
    (t54567, t54582, t54585, t54607, t54609, t54611, t54614, t54631, t54633, t54635, t54637, t54639)
}
