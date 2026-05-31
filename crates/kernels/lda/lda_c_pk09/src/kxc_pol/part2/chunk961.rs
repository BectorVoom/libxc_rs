//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 961/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk961<F: Float>(t2595: F, t5819: F, t1435: F, t2571: F, t10162: F, t1451: F, t2596: F, t5404: F, t5632: F, t5783: F, t9623: F, t9631: F, t9635: F, t9742: F, t9750: F) -> F {
    let t10164 = t2595 * t5819;
    let t10174 = t2571 * t1435;
    let t10177 = t10162 / F::cast_from(18.0_f64) - t10164 * t1451 / F::cast_from(6.0_f64) - t2596 * t5632 / F::cast_from(6.0_f64) + F::cast_from(0.10237773105191754_f64) * t9623 + F::cast_from(0.03412591035063918_f64) * t9631 + F::cast_from(0.10237773105191754_f64) * t9635 + F::cast_from(0.10237773105191754_f64) * t9742 + F::cast_from(0.10237773105191754_f64) * t9750 + t10174 / F::cast_from(18.0_f64) + F::cast_from(0.04991874779241519_f64) * t5404 - t5783;
    t10177
}
