//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1213/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1213(t2171: f64, t3888: f64, t1325: f64, t1440: f64, t2166: f64, t3545: f64, t10162: f64, t2167: f64, t3787: f64, t5381: f64, t3794: f64, t4953: f64) -> (f64, f64, f64, f64, f64) {
    let t14307 = 4.0_f64 / 45.0_f64 * t2171 * t3888;
    let t14311 = 4.0_f64 / 15.0_f64 * t1325 * t1440 * t2166 * t3545;
    let t14313 = t1325 * t10162 * t2167;
    let t14314 = 8.0_f64 / 45.0_f64 * t14313;
    let t14316 = t1325 * t3787 * t5381;
    let t14317 = 8.0_f64 / 15.0_f64 * t14316;
    let t14319 = 8.0_f64 / 5.0_f64 * t3794 * t4953;
    (t14307, t14311, t14314, t14317, t14319)
}
