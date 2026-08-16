//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 679/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk679(t1131: f64, t1578: f64, t485: f64, t1138: f64, t1597: f64, t2877: f64, t3381: f64, t3383: f64, t3386: f64, t3389: f64, t3392: f64, t3396: f64, t3401: f64, t3406: f64, t3410: f64, t3415: f64, t3418: f64, t3423: f64, t3427: f64, t3432: f64, t3434: f64, t3436: f64) -> (f64, f64, f64) {
    let t4172 = 0.01975389032890948_f64 * t1578 * t1131 * t485;
    let t4175 = 0.0034679929861433484_f64 * t2877 * t1138 * t1597;
    let t4176 = t3381 + t3383 + t3386 + t3389 + t3392 + t3396 + t3401 - t3406 - t3410 - t3415 + t3418 + t3423 + t3427 + t3432 - t3434 + t3436;
    (t4172, t4175, t4176)
}
