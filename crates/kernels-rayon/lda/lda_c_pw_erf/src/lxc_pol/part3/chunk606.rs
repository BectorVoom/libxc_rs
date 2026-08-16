//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 606/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk606(t3455: f64, t496: f64, t1234: f64, t511: f64, t1280: f64, t1298: f64, t1302: f64, t2960: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3457 = 4.0_f64 / 5.0_f64 * t3455 * t496;
    let t3458 = t511 * t1234;
    let t3459 = 8.0_f64 / 15.0_f64 * t3458;
    let t3461 = 2.0_f64 / 5.0_f64 * t511 * t1280;
    let t3463 = 4.0_f64 / 5.0_f64 * t1298 * t1302;
    let t3464 = 3.0_f64 * t2960;
    (t3457, t3458, t3459, t3461, t3463, t3464)
}
