//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta720 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2564;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta720<F: Float>(t10482: F, t3120: F, t10470: F, t11064: F, t381: F, t1057: F, t49864: F, t3199: F, t49649: F, t11045: F, t14538: F, t225: F) -> (F, F, F, F, F, F) {
        let (t50510, t50516, t50535, t50592, t50610, t50622) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2564::<F>(t10482, t3120, t10470, t11064, t381, t1057, t49864, t3199, t49649, t11045, t14538, t225);
    (t50510, t50516, t50535, t50592, t50610, t50622)
}
