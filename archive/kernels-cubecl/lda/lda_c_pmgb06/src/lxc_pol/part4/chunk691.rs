//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 691/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk691<F: Float>(t208: F, t4087: F, t213: F, t1683: F, t97: F, t588: F, t1696: F, t398: F, t1687: F, t1680: F, t574: F, t581: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4088 = t4087 * t208;
    let t4089 = t4088 * t213;
    let t4090 = t1683 * t97;
    let t4091 = t4090 * t588;
    let t4093 = t398 * t1696;
    let t4094 = t4093 * t208;
    let t4095 = t4094 * t213;
    let t4096 = t1687 * t97;
    let t4097 = t4096 * t588;
    let t4099 = t574 * t1680;
    let t4102 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t581 * t1680;
    (t4088, t4089, t4091, t4093, t4094, t4095, t4096, t4097, t4099, t4102)
}
