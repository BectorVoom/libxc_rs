//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 607/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk607<F: Float>(t695: F, t719: F, t1060: F, t1894: F, t5184: F, t5182: F, t716: F, t654: F, sigma2: F) -> (F, F, F, F) {
    let t5185 = t719 * t695;
    let t5186 = t1060 * t1894;
    let t5187 = t5185 * t5186;
    let t5188 = t5184 * t5187;
    let t5189 = t5182 * t5188;
    let t5191 = t716 * sigma2;
    let t5192 = t5191 * t654;
    (t5185, t5188, t5189, t5192)
}
