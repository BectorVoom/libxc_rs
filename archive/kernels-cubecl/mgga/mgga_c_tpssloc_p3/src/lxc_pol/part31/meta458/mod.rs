//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta458 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1610;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta458<F: Float>(t23237: F, t7488: F, t1880: F, t4300: F, t6571: F, t6553: F, t1519: F, t214: F) -> (F, F, F, F, F, F) {
        let (t25213, t25214, t25216, t25217, t25218, t25224) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1610::<F>(t23237, t7488, t1880, t4300, t6571, t6553, t1519, t214);
    (t25213, t25214, t25216, t25217, t25218, t25224)
}
