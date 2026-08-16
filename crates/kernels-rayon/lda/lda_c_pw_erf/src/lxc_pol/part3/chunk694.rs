//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 694/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk694(t100: f64, t3222: f64, t125: f64, t143: f64, t1735: f64, t3251: f64, t405: f64, t4117: f64, t4122: f64, t4125: f64, t4129: f64, t4132: f64, t4136: f64, t4140: f64, t4144: f64, t4252: f64, t4280: f64) -> (f64, f64) {
    let t4283 = t3222 * t100;
    let t4286 = 9.0_f64 * t4117 * t1735 - 0.0008717022455366076_f64 * t4122 - 0.0017434044910732151_f64 * t4125 - t4129 + 0.004067943812504169_f64 * t4132 + t4136 - t4140 - t4144 + 3.0_f64 * t405 * t143 * t3251 + (t4252 + t4280) * t125 + 6.0_f64 * t4283 * t143;
    (t4283, t4286)
}
