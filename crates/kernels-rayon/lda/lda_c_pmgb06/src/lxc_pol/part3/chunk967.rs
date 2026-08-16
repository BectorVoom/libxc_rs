//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 967/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk967(t342: f64, t38: f64, t5809: f64, t1227: f64, t2221: f64, t8300: f64, t11398: f64, t11401: f64, t11403: f64, t11406: f64, t11407: f64, t11408: f64, t1282: f64, t2229: f64, t3559: f64, t4394: f64, t5740: f64, t63: f64) -> (f64, f64, f64, f64) {
    let t11413 = 17.53815_f64 * t38 * t5809 * t342;
    let t11426 = 17.53815_f64 * t38 * t2221 * t1227;
    let t11427 = 1.9486833333333333_f64 * t8300;
    let t11428 = 1.95872_f64 * t11398 - t11401 - t11403 - t11406 - 18.0_f64 * t11407 * t11408 + t11413 + 17.62848_f64 * t63 * t1282 * t4394 * t342 + 17.62848_f64 * t63 * t5740 * t1227 + 5.87616_f64 * t63 * t2229 * t3559 + t11426 - t11427;
    (t11413, t11426, t11427, t11428)
}
