//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 742/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk742<F: Float>(t3228: F, t871: F, t3113: F, t931: F, t12411: F, t295: F, t3276: F, t7301: F, t943: F, t883: F, t9603: F, t7296: F) -> (F, F, F, F, F, F) {
    let t12573 = t3228 * t871;
    let t12574 = t931 * t3113;
    let t12580 = t295 * t12411;
    let t12604 = t3276 * t7301;
    let t12605 = t943 * t12604;
    let t12607 = t883 * t9603;
    let t12608 = t7296 * t12607;
    (t12573, t12574, t12580, t12604, t12605, t12608)
}
