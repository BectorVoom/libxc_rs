//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1103/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1103(t2146: f64, t3834: f64, t3425: f64, t4763: f64, t10012: f64, t1278: f64, t1440: f64, t4885: f64, t519: f64, t4059: f64, t1325: f64, t4880: f64, t944: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12900 = 4.0_f64 / 9.0_f64 * t2146 * t3834;
    let t12902 = 8.0_f64 / 15.0_f64 * t4763 * t3425;
    let t12903 = 32.0_f64 / 45.0_f64 * t10012;
    let t12907 = 4.0_f64 / 5.0_f64 * t519 * t1440 * t4885 * t1278;
    let t12908 = t2146 * t4059;
    let t12909 = 8.0_f64 / 45.0_f64 * t12908;
    let t12913 = 4.0_f64 / 5.0_f64 * t1325 * t1440 * t4880 * t944;
    (t12900, t12902, t12903, t12907, t12909, t12913)
}
