//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta771 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2672;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta771<F: Float>(t39328: F, t39330: F, t39339: F, t39341: F, t54323: F, t54325: F, t16153: F, t19631: F, t3918: F, t3919: F, t39338: F, t39346: F, t39349: F, t39356: F, t5122: F, t5126: F) -> (F, F, F, F, F, F, F) {
        let (t56149, t56150, t56151, t56152, t56159, t56160, t56161) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2672::<F>(t39328, t39330, t39339, t39341, t54323, t54325, t16153, t19631, t3918, t3919, t39338, t39346, t39349, t39356, t5122, t5126);
    (t56149, t56150, t56151, t56152, t56159, t56160, t56161)
}
