//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta193 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk829;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta193<F: Float>(t3311: F, t419: F, t409: F, t11135: F, t10292: F, t281: F, t415: F, t241: F, t3439: F, t407: F, t410: F, t417: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11189, t11190, t11195, t11203, t11204, t11219, t11243, t11247, t11265) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk829::<F>(t3311, t419, t409, t11135, t10292, t281, t415, t241, t3439, t407, t410, t417);
    (t11189, t11190, t11195, t11203, t11204, t11219, t11243, t11247, t11265)
}
