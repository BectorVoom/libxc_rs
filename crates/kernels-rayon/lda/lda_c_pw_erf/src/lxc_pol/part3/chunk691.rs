//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 691/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk691(t163: f64, t169: f64, t616: f64, t717: f64, t171: f64, t4150: f64, t4153: f64, t4156: f64, t4160: f64, t4163: f64, t4165: f64, t4168: f64, t4172: f64, t4175: f64, t4239: f64, t4246: f64) -> (f64, f64) {
    let t4250 = t169 * t717 * t616 * t163;
    let t4252 = -0.005926167098672845_f64 * t4150 - 0.01185233419734569_f64 * t4153 - 0.0014862827083471494_f64 * t4156 - t4160 - t4163 - t4165 + 0.01975389032890948_f64 * t4168 + t4172 + t4175 - 0.005388405304614574_f64 * t169 * t171 * t4239 * t163 + 0.02694202652307287_f64 * t4246 - 0.07184540406152766_f64 * t4250;
    (t4250, t4252)
}
