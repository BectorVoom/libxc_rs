//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1072/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1072<F: Float>(t1420: F, t5268: F, t2948: F, t439: F, t5267: F, t1074: F, t1385: F, t5231: F, t1906: F, t3115: F, t5273: F, t10148: F, t5272: F) -> (F, F, F, F, F, F) {
    let t12730 = t1420 * t5268 / F::cast_from(15.0_f64);
    let t12733 = t439 * t2948 * t5267 / F::cast_from(15.0_f64);
    let t12737 = t439 * t1385 * t5231 * t1074 / F::cast_from(15.0_f64);
    let t12741 = t439 * t1385 * t1906 * t3115 / F::cast_from(45.0_f64);
    let t12743 = t1420 * t5273 / F::cast_from(9.0_f64);
    let t12746 = t439 * t10148 * t5272 / F::cast_from(9.0_f64);
    (t12730, t12733, t12737, t12741, t12743, t12746)
}
