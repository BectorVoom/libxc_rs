//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 652/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk652(t22: f64, t3892: f64, t197: f64, t3518: f64, t2954: f64, t519: f64, t213: f64, t9: f64) -> (f64, f64, f64, f64, f64) {
    let t3893 = t22 * t3892;
    let t3894 = t197 * t3518;
    let t3895 = t3894 * t2954;
    let t3896 = t3893 * t3895;
    let t3898 = 32.0_f64 / 81.0_f64 * t519 * t3896;
    let t3899 = t9 * t213;
    (t3893, t3895, t3896, t3898, t3899)
}
