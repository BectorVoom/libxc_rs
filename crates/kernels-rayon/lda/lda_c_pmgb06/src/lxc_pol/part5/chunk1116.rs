//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1116/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1116(t1864: f64, t2605: f64, t5077: f64, t1859: f64, t5083: f64, t20380: f64, t20382: f64, t20386: f64, t20390: f64, t20394: f64, t20397: f64, t20400: f64, t20403: f64, t20406: f64, t20409: f64) -> (f64, f64, f64) {
    let t20412 = 4.0_f64 / 15.0_f64 * t5077 * t2605 * t1864;
    let t20415 = 2.0_f64 / 9.0_f64 * t5083 * t2605 * t1859;
    let t20416 = t20380 + t20382 + t20386 + t20390 + t20394 + t20397 - t20400 - t20403 + t20406 + t20409 - t20412 + t20415;
    (t20412, t20415, t20416)
}
