//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 806/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk806(t7376: f64, t85: f64, t2995: f64, t3000: f64, t3009: f64, t3011: f64, t3016: f64, t3118: f64, t3121: f64, t3125: f64, t3133: f64, t3155: f64, t7353: f64) -> (f64, f64) {
    let t7377 = t7376 * t85;
    let t7378 = 0.019751789702565206_f64 * t7377;
    let t7379 = -t7353 + t2995 - t3000 - t3009 - t3011 + t3016 + t7378 + t3155 + t3118 - t3121 + t3125 + t3133;
    (t7378, t7379)
}
