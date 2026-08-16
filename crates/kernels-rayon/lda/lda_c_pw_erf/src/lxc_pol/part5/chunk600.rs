//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 600/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk600(t22: f64, t4048: f64, t219: f64, t3589: f64, t1484: f64, t9: f64, t1210: f64, t168: f64, t671: f64, t270: f64, t2782: f64, t1143: f64, t466: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4049 = t22 * t4048;
    let t4050 = t219 * t3589;
    let t4062 = t9 * t1484;
    let t4084 = t168 * t1210 * t671;
    let t4091 = 0.19455129084526285_f64 * t168 * t2782 * t270;
    let t4092 = t466 * t1143;
    (t4049, t4050, t4062, t4084, t4091, t4092)
}
