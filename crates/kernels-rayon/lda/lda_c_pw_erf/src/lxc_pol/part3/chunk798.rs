//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 798/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk798(t2162: f64, t3899: f64, t571: f64, t1381: f64, t2161: f64, t1466: f64, t2167: f64, t3787: f64, t1325: f64, t1278: f64, t2166: f64, t1440: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5371 = t3899 * t2162;
    let t5373 = 16.0_f64 / 45.0_f64 * t571 * t5371;
    let t5374 = t2161 * t1381;
    let t5375 = t1466 * t5374;
    let t5378 = t3787 * t2167;
    let t5380 = 16.0_f64 / 45.0_f64 * t1325 * t5378;
    let t5381 = t2166 * t1278;
    let t5382 = t1440 * t5381;
    (t5371, t5373, t5374, t5375, t5378, t5380, t5381, t5382)
}
