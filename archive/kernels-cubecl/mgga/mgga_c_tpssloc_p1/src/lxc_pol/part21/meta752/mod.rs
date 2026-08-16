//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta752 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2625;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2626;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta752<F: Float>(t40281: F, t5303: F, t12211: F, t16300: F, t5247: F, t820: F, t12250: F, t1824: F, t16288: F, t3853: F, t12384: F, t5234: F, t3795: F, t5293: F, t12283: F, t16405: F, t40167: F, t1799: F, t3791: F, t40138: F, t5259: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t53997, t54003, t54013, t54014, t54034, t54042) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2625::<F>(t40281, t5303, t12211, t16300, t5247, t820, t12250, t1824, t16288, t3853, t12384, t5234);
        let (t54043, t54047, t54059, t54063, t54068, t54086) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2626::<F>(t3795, t54042, t40281, t5293, t12283, t16405, t40167, t820, t1799, t3791, t40138, t5259);
    (t53997, t54003, t54013, t54014, t54034, t54043, t54047, t54059, t54063, t54068, t54086)
}
