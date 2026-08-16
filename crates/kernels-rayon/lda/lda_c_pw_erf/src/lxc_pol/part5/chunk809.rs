//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 809/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk809(t4160: f64, t4163: f64, t4165: f64, t4172: f64, t4175: f64, t4254: f64, t4258: f64, t5440: f64, t7222: f64, t7294: f64, t7298: f64, t7300: f64) -> f64 {
    let t7400 = -0.09451622166942335_f64 * t7298 - 0.005926167098672845_f64 * t7222 - 0.07184540406152766_f64 * t5440 + 0.02694202652307287_f64 * t7294 + 0.09451622166942335_f64 * t7300 - t4160 - t4163 - t4165 + t4172 + t4175 + t4254 + t4258;
    t7400
}
