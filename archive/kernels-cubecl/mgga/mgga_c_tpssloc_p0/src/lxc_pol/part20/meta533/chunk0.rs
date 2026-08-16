//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2069/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2069<F: Float>(t12226: F, t16094: F, t3719: F, t686: F, t3736: F, t40018: F, t59: F, t9223: F, t116: F, t120: F, t212: F, t22815: F, t67: F) -> (F, F, F, F) {
    let t40376 = t16094 * t686 * t12226 * t3719;
    let t40387 = t40018 * t3736;
    let t40394 = t59 * t9223;
    let t40399 = t116 * t67 * t22815 * t120 * t212;
    (t40376, t40387, t40394, t40399)
}
