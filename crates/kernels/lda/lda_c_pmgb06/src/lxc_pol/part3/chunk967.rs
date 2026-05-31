//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 967/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk967<F: Float>(t342: F, t38: F, t5809: F, t1227: F, t2221: F, t8300: F, t11398: F, t11401: F, t11403: F, t11406: F, t11407: F, t11408: F, t1282: F, t2229: F, t3559: F, t4394: F, t5740: F, t63: F) -> (F, F, F, F) {
    let t11413 = F::cast_from(17.53815_f64) * t38 * t5809 * t342;
    let t11426 = F::cast_from(17.53815_f64) * t38 * t2221 * t1227;
    let t11427 = F::cast_from(1.9486833333333333_f64) * t8300;
    let t11428 = F::cast_from(1.95872_f64) * t11398 - t11401 - t11403 - t11406 - F::cast_from(18.0_f64) * t11407 * t11408 + t11413 + F::cast_from(17.62848_f64) * t63 * t1282 * t4394 * t342 + F::cast_from(17.62848_f64) * t63 * t5740 * t1227 + F::cast_from(5.87616_f64) * t63 * t2229 * t3559 + t11426 - t11427;
    (t11413, t11426, t11427, t11428)
}
