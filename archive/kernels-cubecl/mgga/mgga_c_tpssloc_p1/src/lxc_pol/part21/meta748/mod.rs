//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta748 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2620;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta748<F: Float>(t15908: F, t9882: F, t118: F, t2375: F, t5151: F, t16169: F, t2663: F, t15892: F, t2371: F, t5154: F, t9919: F, t5173: F, t591: F) -> (F, F, F, F, F, F) {
        let (t53779, t53782, t53787, t53796, t53798, t53825) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2620::<F>(t15908, t9882, t118, t2375, t5151, t16169, t2663, t15892, t2371, t5154, t9919, t5173, t591);
    (t53779, t53782, t53787, t53796, t53798, t53825)
}
