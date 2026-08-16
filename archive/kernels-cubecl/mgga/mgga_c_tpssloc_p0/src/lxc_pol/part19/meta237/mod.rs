//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta237 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk967;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta237<F: Float>(t11177: F, t300: F, t1098: F, t3256: F, t1119: F, t3259: F, t3308: F, t1094: F, t3312: F, t3316: F, t3311: F, t419: F) -> (F, F, F, F, F, F, F) {
        let (t11179, t11180, t11182, t11184, t11185, t11187, t11189) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk967::<F>(t11177, t300, t1098, t3256, t1119, t3259, t3308, t1094, t3312, t3316, t3311, t419);
    (t11179, t11180, t11182, t11184, t11185, t11187, t11189)
}
