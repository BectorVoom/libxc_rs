//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 891/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk891(t1499: f64, t1555: f64, t3155: f64, t486: f64, t1767: f64, t206: f64, t4068: f64, t4077: f64, t591: f64, t4080: f64, t4111: f64, t4084: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9402 = t1499 * t1555;
    let t9404 = t486 * t3155;
    let t9408 = 0.008082336938271605_f64 * t206 * t1767 * t4068;
    let t9410 = 8.0_f64 / 9.0_f64 * t4077 * t591;
    let t9412 = (2e-21_f64 as f64) * t4080 * t4111;
    let t9413 = t4084 * t591;
    (t9402, t9404, t9408, t9410, t9412, t9413)
}
