//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 601/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk601<F: Float>(t132: F, t3295: F, t1596: F, t432: F, t1919: F, t2924: F, t493: F, t1901: F, t3104: F, t439: F, t1555: F, t486: F) -> (F, F, F, F, F, F, F, F) {
    let t3296 = t132 * t3295;
    let t3297 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t3296;
    let t3299 = t432 * t1596 / F::cast_from(5.0_f64);
    let t3300 = t1919 * t2924;
    let t3302 = t493 * t3300 / F::cast_from(9.0_f64);
    let t3303 = t1901 * t3104;
    let t3305 = t439 * t3303 / F::cast_from(9.0_f64);
    let t3306 = t486 * t1555;
    (t3296, t3297, t3299, t3300, t3302, t3303, t3305, t3306)
}
