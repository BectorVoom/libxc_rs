//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 681/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk681<F: Float>(t12656: F, t7428: F, t7427: F, t969: F, t825: F, t3209: F, t7290: F, t2365: F, t6111: F, t10037: F, t7785: F, t12651: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12657 = t7428 * t12656;
    let t12658 = t7427 * t12657;
    let t12660 = t969 * t12656;
    let t12661 = t825 * t12660;
    let t12663 = t7290 * t3209;
    let t12664 = t2365 * t12663;
    let t12665 = t6111 * t12664;
    let t12667 = t10037 * t7785;
    let t12669 = t969 * t12651;
    (t12657, t12658, t12660, t12661, t12663, t12664, t12665, t12667, t12669)
}
