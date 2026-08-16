//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta515 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2040;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta515<F: Float>(t3684: F, t39500: F, t2393: F, t2528: F, t677: F, t9722: F, t118: F, t2375: F, t3681: F, t12110: F, t9888: F, t9467: F) -> (F, F, F, F, F, F, F, F) {
        let (t39502, t39503, t39505, t39506, t39508, t39510, t39512, t39514) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2040::<F>(t3684, t39500, t2393, t2528, t677, t9722, t118, t2375, t3681, t12110, t9888, t9467);
    (t39502, t39503, t39505, t39506, t39508, t39510, t39512, t39514)
}
