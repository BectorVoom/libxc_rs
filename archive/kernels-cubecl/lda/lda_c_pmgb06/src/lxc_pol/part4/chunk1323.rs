//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1323/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1323<F: Float>(t17188: F, t1919: F, t493: F, t1444: F, t6766: F, t5463: F, t6765: F, t17143: F, t17147: F, t17152: F, t5470: F, t1464: F, t2093: F, t5071: F, t5138: F) -> (F, F, F, F, F, F, F) {
    let t17384 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t493 * t1919 * t17188;
    let t17386 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1444 * t6766;
    let t17389 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t493 * t5463 * t6765;
    let t17392 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t493 * t1919 * t17143;
    let t17395 = t493 * t1919 * t17147 / F::cast_from(27.0_f64);
    let t17398 = F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t493 * t5470 * t17152;
    let t17402 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t5138 * t2093 * t1464 * t5071;
    (t17384, t17386, t17389, t17392, t17395, t17398, t17402)
}
