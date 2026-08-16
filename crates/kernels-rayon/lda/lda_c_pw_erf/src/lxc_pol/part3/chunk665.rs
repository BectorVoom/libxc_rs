//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 665/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk665(t173: f64, t4008: f64, t184: f64, t199: f64, t3638: f64, t3625: f64, t3627: f64, t3629: f64, t3631: f64, t3635: f64, t3641: f64, t3644: f64, t3646: f64, t3649: f64, t3652: f64) -> (f64, f64, f64, f64, f64) {
    let t4009 = t173 * t4008;
    let t4010 = t4009 * t184;
    let t4012 = 2.0_f64 / 15.0_f64 * t4010 * t199;
    let t4013 = 0.005877407407407408_f64 * t3638;
    let t4024 = t4013 + 0.002518888888888889_f64 * t3627 - 0.0012594444444444445_f64 * t3631 + 0.003778333333333333_f64 * t3646 - 0.0018891666666666666_f64 * t3629 + 0.002099074074074074_f64 * t3635 - 0.007556666666666666_f64 * t3649 + 0.003778333333333333_f64 * t3652 + 0.011335_f64 * t3641 - 0.011335_f64 * t3644 + 0.0018891666666666666_f64 * t3625;
    (t4009, t4010, t4012, t4013, t4024)
}
