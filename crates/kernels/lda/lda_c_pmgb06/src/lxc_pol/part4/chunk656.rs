//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 656/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk656<F: Float>(t208: F, t4093: F, t213: F, t1687: F, t97: F, t588: F, t1680: F, t574: F, t581: F, t211: F, t410: F, t209: F, t1684: F, t591: F, t1688: F, t125: F, t586: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4094 = t4093 * t208;
    let t4095 = t4094 * t213;
    let t4096 = t1687 * t97;
    let t4097 = t4096 * t588;
    let t4099 = t574 * t1680;
    let t4102 = 2.0 / 9.0 * t581 * t1680;
    let t4103 = t211 * t410;
    let t4105 = 8.0 / 81.0 * t209 * t4103;
    let t4106 = t1684 * t591;
    let t4108 = t1688 * t591;
    let t4111 = t586 * t125 * t208;
    (t4094, t4095, t4096, t4097, t4099, t4102, t4103, t4105, t4106, t4108, t4111)
}
