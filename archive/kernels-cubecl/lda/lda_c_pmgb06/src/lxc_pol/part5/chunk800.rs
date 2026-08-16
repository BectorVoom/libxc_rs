//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 800/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk800<F: Float>(t153: F, t7501: F, t137: F, t132: F, t6423: F, t6425: F, t2549: F, t851: F, t1380: F, t493: F, t6759: F, t764: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7502 = t7501 * t153;
    let t7503 = t137 * t7502;
    let t7505 = t132 * t7503 / F::cast_from(30.0_f64);
    let t7506 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t6423;
    let t7507 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t6425;
    let t7508 = t2549 * t851;
    let t7509 = t1380 * t7508;
    let t7511 = t493 * t7509 / F::cast_from(15.0_f64);
    let t7512 = t6759 * t764;
    (t7502, t7503, t7505, t7506, t7507, t7508, t7509, t7511, t7512)
}
