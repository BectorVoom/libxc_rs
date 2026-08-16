//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 843/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk843(t5268: f64, t5274: f64, t5278: f64, t5281: f64, t5284: f64, t5288: f64, t5294: f64, t5298: f64, t5301: f64, t5304: f64, t5309: f64, t5312: f64, t5314: f64, t5319: f64, t5324: f64, t5329: f64, t5331: f64) -> f64 {
    let t5868 = t5268 + t5274 + t5278 + t5281 + t5284 + t5288 + t5294 + t5298 + t5301 - t5304 + t5309 + t5312 - t5314 - t5319 + t5324 + t5329 - t5331;
    t5868
}
