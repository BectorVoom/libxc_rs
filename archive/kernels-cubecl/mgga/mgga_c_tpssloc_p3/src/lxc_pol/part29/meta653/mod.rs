//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta653 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2178;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta653<F: Float>(t2241: F, t72: F, t7431: F, t12648: F, t605: F, t12652: F, t12661: F, t4017: F, t645: F, t1433: F, t12568: F, t608: F) -> (F, F, F, F, F, F, F) {
        let (t90141, t90150, t90153, t90160, t90177, t90196, t90202) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2178::<F>(t2241, t72, t7431, t12648, t605, t12652, t12661, t4017, t645, t1433, t12568, t608);
    (t90141, t90150, t90153, t90160, t90177, t90196, t90202)
}
