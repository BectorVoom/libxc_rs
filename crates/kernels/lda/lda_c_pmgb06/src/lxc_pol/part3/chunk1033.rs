//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1033/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1033<F: Float>(t4803: F, t486: F, t490: F, t5432: F, t1504: F, t1848: F, t3073: F, t831: F, t132: F, t435: F, t4681: F, t1842: F, t642: F) -> (F, F, F, F, F, F) {
    let t12273 = t486 * t4803 / F::cast_from(5.0_f64);
    let t12274 = t5432 * t490;
    let t12275 = t12274 / F::cast_from(15.0_f64);
    let t12276 = t1848 * t1504;
    let t12277 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t12276;
    let t12278 = t831 * t3073;
    let t12279 = t12278 / F::cast_from(15.0_f64);
    let t12281 = t132 * t435 * t4681;
    let t12282 = t12281 / F::cast_from(15.0_f64);
    let t12294 = F::cast_from(48.0_f64) * t1842 * t642;
    (t12273, t12275, t12277, t12279, t12282, t12294)
}
