//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 147/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk147(t379: f64, t75: f64, t323: f64, t325: f64, t329: f64, t331: f64) -> (f64, f64) {
    let t380 = t75 * t379;
    let t385 = -0.8630833333333333_f64 * t323 - 0.301925_f64 * t325 - 0.05501625_f64 * t329 - 0.082785_f64 * t331;
    (t380, t385)
}
