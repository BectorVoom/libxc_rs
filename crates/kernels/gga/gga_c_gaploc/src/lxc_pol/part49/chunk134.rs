//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 134/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk134<F: Float>(t1: F, t231: F, t362: F, t46: F, t375: F, t268: F, t378: F, t61: F) -> (F, F, F) {
    let t643 = t231 * t1;
    let t645 = 0.18311555036753159941e-3 * t643 * t362;
    let t646 = t231 * t46;
    let t648 = 0.58482233974552040708e0 * t646 * t375;
    let t650 = t61 * t378 * t268;
    (t645, t648, t650)
}
