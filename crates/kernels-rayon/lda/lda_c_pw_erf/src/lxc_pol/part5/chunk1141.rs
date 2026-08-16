//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1141/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1141(t1308: f64, t2065: f64, t2415: f64, t571: f64, t504: f64, t7792: f64, t1313: f64, t348: f64, t519: f64, t16127: f64, t16129: f64, t16134: f64) -> (f64, f64, f64, f64, f64) {
    let t21051 = 8.0_f64 / 15.0_f64 * t571 * t1308 * t2415 * t2065;
    let t21052 = t7792 * t504;
    let t21056 = 4.0_f64 / 45.0_f64 * t519 * t1313 * t21052 * t348;
    let t21057 = 64.0_f64 / 45.0_f64 * t16127;
    let t21058 = 32.0_f64 / 45.0_f64 * t16129;
    let t21059 = 16.0_f64 / 15.0_f64 * t16134;
    (t21051, t21056, t21057, t21058, t21059)
}
