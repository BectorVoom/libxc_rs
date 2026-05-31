//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 572/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk572<F: Float>(t161: F, t3043: F, t1395: F, t1629: F, t137: F, t132: F, t1630: F, t435: F, t1631: F, t432: F, t1547: F, t478: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3044 = t161 * t3043;
    let t3045 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t3044;
    let t3046 = t1395 * t1629;
    let t3047 = t137 * t3046;
    let t3049 = t132 * t3047 / F::cast_from(10.0_f64);
    let t3050 = t435 * t1630;
    let t3051 = t132 * t3050;
    let t3052 = t3051 / F::cast_from(15.0_f64);
    let t3054 = t432 * t1631 / F::cast_from(10.0_f64);
    let t3055 = t1547 * t478;
    (t3044, t3045, t3046, t3047, t3049, t3050, t3051, t3052, t3054, t3055)
}
