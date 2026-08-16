//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta238 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk968;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta238<F: Float>(t11189: F, t409: F, t1117: F, t3265: F, t3315: F, t11135: F, t1102: F, t3270: F, t3279: F, t3287: F, t10292: F, t281: F, t415: F) -> (F, F, F, F, F, F, F, F) {
        let (t11190, t11191, t11192, t11194, t11195, t11197, t11200, t11203) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk968::<F>(t11189, t409, t1117, t3265, t3315, t11135, t1102, t3270, t3279, t3287, t10292, t281, t415);
    (t11190, t11191, t11192, t11194, t11195, t11197, t11200, t11203)
}
