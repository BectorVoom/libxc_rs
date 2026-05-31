//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1036/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1036<F: Float>(t5375: F, t591: F, t4111: F, t5378: F, t5382: F, t5386: F, t5391: F, t1542: F, t1887: F, t138: F, t4676: F, t9175: F) -> (F, F, F, F, F, F, F) {
    let t12304 = t5375 * t591;
    let t12306 = t5378 * t4111;
    let t12307 = F::cast_from(2e-21_f64) * t12306;
    let t12308 = t5382 * t591;
    let t12310 = t5386 * t591;
    let t12311 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t12310;
    let t12312 = t5391 * t4111;
    let t12313 = F::cast_from(2e-21_f64) * t12312;
    let t12315 = t1887 * t1542 / F::cast_from(10.0_f64);
    let t12325 = t138 * t9175 * t4676;
    (t12304, t12307, t12308, t12311, t12313, t12315, t12325)
}
