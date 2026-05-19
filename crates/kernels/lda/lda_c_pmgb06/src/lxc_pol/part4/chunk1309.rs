//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1309/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1309<F: Float>(t405: F, t6891: F, t4913: F, t6894: F, t6897: F, t6900: F, t103: F, t15503: F, t1576: F, t16073: F, t16354: F, t17143: F, t17147: F, t17152: F, t17160: F, t17188: F, t3358: F, t525: F) -> F {
    let t17215 = t405 * t6891;
    let t17217 = t4913 * t6894;
    let t17222 = t405 * t6897;
    let t17224 = t405 * t6900;
    let t17229 = F::cast_from(0.02666666666666667_f64) * t103 * t525 * t15503 + F::cast_from(0.013333333333333334_f64) * t103 * t525 * t17160 + F::cast_from(0.013333333333333334_f64) * t103 * t1576 * t17188 - F::cast_from(0.0044444444444444444_f64) * t103 * t1576 * t17143 - F::cast_from(0.0022222222222222222_f64) * t103 * t1576 * t17147 - F::cast_from(0.002962962962962963_f64) * t103 * t3358 * t17152 + F::cast_from(0.05333333333333334_f64) * t17215 + F::cast_from(0.2311111111111111_f64) * t17217 - F::new(0.04) * t103 * t525 * t16073 - F::cast_from(0.017777777777777778_f64) * t17222 + F::cast_from(0.002962962962962963_f64) * t17224 + F::cast_from(0.013333333333333334_f64) * t103 * t1576 * t16354;
    t17229
}
