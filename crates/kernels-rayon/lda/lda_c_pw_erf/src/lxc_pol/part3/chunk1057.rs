//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1057/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1057(t3518: f64, t3892: f64, t529: f64, t12114: f64, t4488: f64, t12362: f64, t12364: f64, t4501: f64, t1245: f64, t4722: f64, t494: f64, t739: f64, t940: f64) -> (f64, f64, f64, f64) {
    let t12380 = t3892 * t529 * t3518;
    let t12383 = 32.0_f64 / 27.0_f64 * t4488 * t12380 * t12114;
    let t12386 = 16.0_f64 / 9.0_f64 * t12362 * t4501 * t12364;
    let t12387 = t4722 * t1245;
    let t12389 = t739 * t940 * t494;
    (t12383, t12386, t12387, t12389)
}
