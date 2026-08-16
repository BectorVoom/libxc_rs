//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 782/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk782(t1958: f64, t202: f64, t184: f64, t551: f64, t172: f64, t1980: f64, t496: f64, t1245: f64, t806: f64, t940: f64, t3402: f64, t519: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5210 = t202 * t1958;
    let t5211 = t5210 * t184;
    let t5213 = 8.0_f64 / 15.0_f64 * t5211 * t551;
    let t5214 = t172 * t1980;
    let t5215 = t5214 * t184;
    let t5217 = 8.0_f64 / 15.0_f64 * t5215 * t496;
    let t5220 = t806 * t1245;
    let t5221 = t5220 * t940;
    let t5222 = t3402 * t5221;
    let t5224 = 4.0_f64 / 27.0_f64 * t519 * t5222;
    (t5210, t5211, t5213, t5214, t5215, t5217, t5220, t5221, t5222, t5224)
}
