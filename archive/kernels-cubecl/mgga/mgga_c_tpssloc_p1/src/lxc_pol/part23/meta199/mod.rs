//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta199 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk840;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta199<F: Float>(t1208: F, t478: F, t10477: F, t483: F, t11713: F, t3508: F, t475: F) -> (F, F, F, F, F, F, F) {
        let (t11714, t11715, t11716, t11717, t11718, t11719, t11721) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk840::<F>(t1208, t478, t10477, t483, t11713, t3508, t475);
    (t11714, t11715, t11716, t11717, t11718, t11719, t11721)
}
