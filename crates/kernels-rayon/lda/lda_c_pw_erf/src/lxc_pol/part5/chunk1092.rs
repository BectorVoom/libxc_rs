//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1092/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1092(t9003: f64, t9017: f64, t102: f64, t120: f64, t20283: f64, t19571: f64, t19574: f64, t19577: f64, t127: f64, t19580: f64, t19584: f64, t19590: f64, t19593: f64, t19604: f64, t19614: f64, t436: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20318 = 1.5156425925925925_f64 * t9003;
    let t20319 = 1.2991222222222223_f64 * t9017;
    let t20324 = 2.923025_f64 * t102 * t120 * t20283;
    let t20328 = 1.9486833333333333_f64 * t19571;
    let t20329 = 0.9743416666666667_f64 * t19574;
    let t20330 = 1.4615125_f64 * t19577;
    let t20337 = -t20324 - 1.46904_f64 * t127 * t436 * t20283 - t20328 + t20329 + t20330 + 44.0712_f64 * t19580 - 17.62848_f64 * t19584 + 6.0_f64 * t19590 - 3.0_f64 * t19593 - 3.0_f64 / 2.0_f64 * t19604 - 8.81424_f64 * t19614;
    (t20318, t20319, t20324, t20328, t20329, t20330, t20337)
}
