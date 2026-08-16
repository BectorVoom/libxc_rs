//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1063/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1063(t12450: f64, t3965: f64, t5147: f64, t12031: f64, t12389: f64, t3619: f64, t4506: f64, t5151: f64, t10011: f64, t5138: f64, t5143: f64, t5148: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12453 = 8.0_f64 / 9.0_f64 * t3965 * t5147 * t12450;
    let t12456 = 64.0_f64 / 27.0_f64 * t3965 * t12031 * t12389;
    let t12459 = 8.0_f64 / 15.0_f64 * t4506 * t5151 * t3619;
    let t12460 = t10011 * t5138;
    let t12461 = 32.0_f64 / 45.0_f64 * t12460;
    let t12462 = t10011 * t5143;
    let t12463 = 64.0_f64 / 45.0_f64 * t12462;
    let t12464 = t10011 * t5148;
    (t12453, t12456, t12459, t12461, t12463, t12464)
}
