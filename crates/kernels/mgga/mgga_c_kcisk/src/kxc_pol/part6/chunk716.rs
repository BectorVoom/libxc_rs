//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 716/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk716<F: Float>(t12716: F, t3138: F, t12499: F, t151: F, t12435: F, t3086: F, t3107: F, t955: F, t3216: F, t196: F, t967: F, t119: F, t181: F) -> (F, F, F, F, F, F, F) {
    let t12717 = t3138 * t12716;
    let t12723 = t151 * t12499;
    let t12730 = t3086 * t12435;
    let t12734 = t955 * t3107;
    let t12735 = t3216 * t12734;
    let t12741 = t196 * t967;
    let t12747 = t119 * t181;
    (t12717, t12723, t12730, t12734, t12735, t12741, t12747)
}
