//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 778/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk778<F: Float>(t2012: F, t5168: F, t3216: F, t805: F, t439: F, t1600: F, t2088: F, t529: F, t1992: F, t493: F, t165: F, t511: F) -> (F, F, F, F, F, F, F) {
    let t5170 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t5168 * t2012;
    let t5171 = t3216 * t805;
    let t5173 = t439 * t5171 / F::cast_from(45.0_f64);
    let t5174 = t1600 * t2088;
    let t5175 = t5174 * t529;
    let t5176 = t1992 * t5175;
    let t5178 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t493 * t5176;
    let t5179 = t165 * t511;
    (t5170, t5171, t5173, t5175, t5176, t5178, t5179)
}
