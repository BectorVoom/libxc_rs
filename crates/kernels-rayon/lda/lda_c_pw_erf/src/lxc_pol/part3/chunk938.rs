//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 938/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk938(t10463: f64, t1325: f64, t1328: f64, t3783: f64, t529: f64, t1314: f64, t519: f64, t3393: f64, t3859: f64, t3454: f64, t518: f64, t4025: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10465 = t1325 * t10463 * t1328;
    let t10467 = t3783 * t529;
    let t10469 = t519 * t10467 * t1314;
    let t10472 = t1325 * t3859 * t3393;
    let t10474 = t3454 * t518;
    let t10488 = t4025 * t518;
    (t10465, t10467, t10469, t10472, t10474, t10488)
}
