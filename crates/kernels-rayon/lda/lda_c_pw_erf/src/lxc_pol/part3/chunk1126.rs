//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1126/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1126(t1124: f64, t188: f64, t1325: f64, t4958: f64, t2171: f64, t3735: f64, t3739: f64, t3787: f64, t4952: f64, t1440: f64, t3675: f64, t3677: f64, t784: f64) -> (f64, f64, f64, f64, f64) {
    let t13172 = t1124 * t188;
    let t13174 = t1325 * t13172 * t4958;
    let t13175 = 4.0_f64 / 3.0_f64 * t13174;
    let t13176 = t2171 * t3735;
    let t13177 = 16.0_f64 / 45.0_f64 * t13176;
    let t13179 = 8.0_f64 / 15.0_f64 * t2171 * t3739;
    let t13181 = t1325 * t3787 * t4952;
    let t13182 = 16.0_f64 / 15.0_f64 * t13181;
    let t13187 = 8.0_f64 / 5.0_f64 * t1325 * t1440 * t3675 * t784 * t3677;
    (t13175, t13177, t13179, t13182, t13187)
}
