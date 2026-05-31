//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1024/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1024<F: Float>(t439: F, t4650: F, t5253: F, t3010: F, t760: F, t9220: F, t5260: F, t1: F, t1069: F, t3098: F, t1901: F, t2010: F) -> (F, F, F, F, F) {
    let t12174 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t439 * t5253 * t4650;
    let t12176 = t9220 * t760 * t3010;
    let t12179 = F::cast_from(32.0_f64) / F::cast_from(27.0_f64) * t439 * t5260 * t12176;
    let t12181 = t3098 * t1 * t1069;
    let t12184 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t2010 * t1901 * t12181;
    (t12174, t12176, t12179, t12181, t12184)
}
