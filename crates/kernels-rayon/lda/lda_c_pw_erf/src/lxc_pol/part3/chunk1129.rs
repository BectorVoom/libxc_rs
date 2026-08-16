//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1129/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1129(t13211: f64, t4804: f64, t5414: f64, t3794: f64, t3476: f64, t784: f64, t1325: f64, t1991: f64, t2954: f64, t5418: f64, t1976: f64, t4829: f64, t945: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13212 = 32.0_f64 / 45.0_f64 * t13211;
    let t13214 = 16.0_f64 / 15.0_f64 * t4804 * t5414;
    let t13216 = 16.0_f64 / 15.0_f64 * t3794 * t5414;
    let t13217 = t784 * t3476;
    let t13221 = 16.0_f64 / 9.0_f64 * t1325 * t1991 * t13217 * t2954;
    let t13223 = 16.0_f64 / 15.0_f64 * t4804 * t5418;
    let t13225 = 16.0_f64 / 15.0_f64 * t3794 * t5418;
    let t13229 = 8.0_f64 / 15.0_f64 * t1325 * t4829 * t1976 * t945;
    (t13212, t13214, t13216, t13221, t13223, t13225, t13229)
}
