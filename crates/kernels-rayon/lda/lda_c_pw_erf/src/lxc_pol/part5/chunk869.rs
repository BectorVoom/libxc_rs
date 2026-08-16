//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 869/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk869(t143: f64, t2211: f64, t279: f64, t405: f64, t4129: f64, t4136: f64, t4140: f64, t4144: f64, t5933: f64, t7072: f64, t7077: f64, t777: f64, t7913: f64, t8001: f64, t8004: f64, t8068: f64) -> f64 {
    let t8074 = -0.03592270203076383_f64 * t7072 - 2.0_f64 * t777 * t8001 + 9.0_f64 * t2211 * t8004 + 0.05987117005127304_f64 * t7077 + t8068 * t279 + 3.0_f64 * t405 * t143 * t7913 - 5.4655730795145296e-05_f64 * t5933 - t4129 + t4136 - t4140 - t4144;
    t8074
}
