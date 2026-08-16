//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 800/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk800(t1319: f64, t5405: f64, t2006: f64, t3859: f64, t1325: f64, t1251: f64, t784: f64, t940: f64, t1326: f64, t1976: f64, t348: f64, t4829: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5406 = t1319 * t5405;
    let t5409 = t3859 * t2006;
    let t5411 = 32.0_f64 / 135.0_f64 * t1325 * t5409;
    let t5412 = t784 * t1251;
    let t5413 = t5412 * t940;
    let t5414 = t1326 * t5413;
    let t5417 = t1976 * t348;
    let t5418 = t4829 * t5417;
    (t5406, t5409, t5411, t5413, t5414, t5417, t5418)
}
