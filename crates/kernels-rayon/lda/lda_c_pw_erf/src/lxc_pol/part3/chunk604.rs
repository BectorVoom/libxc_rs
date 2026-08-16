//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 604/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk604(t1405: f64, t565: f64, t1284: f64, t1397: f64, t1404: f64, t514: f64, t211: f64, t1508: f64, t544: f64, t1302: f64, t2114: f64, t1513: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3442 = 4.0_f64 / 5.0_f64 * t565 * t1405;
    let t3443 = t1284 * t1397;
    let t3444 = 16.0_f64 / 15.0_f64 * t3443;
    let t3445 = t514 * t1404;
    let t3446 = t211 * t3445;
    let t3447 = 8.0_f64 / 15.0_f64 * t3446;
    let t3449 = 2.0_f64 / 5.0_f64 * t1508 * t544;
    let t3451 = 4.0_f64 / 5.0_f64 * t2114 * t1302;
    let t3453 = 4.0_f64 / 5.0_f64 * t1513 * t544;
    (t3442, t3443, t3444, t3445, t3446, t3447, t3449, t3451, t3453)
}
