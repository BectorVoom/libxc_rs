//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta610 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2376;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta610<F: Float>(t677: F, t9919: F, t3684: F, t2393: F, t2535: F, t12110: F, t9882: F, t2420: F, t701: F, t9778: F) -> (F, F, F, F, F, F) {
        let (t39516, t39518, t39519, t39521, t39522, t39529) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2376::<F>(t677, t9919, t3684, t2393, t2535, t12110, t9882, t2420, t701, t9778);
    (t39516, t39518, t39519, t39521, t39522, t39529)
}
