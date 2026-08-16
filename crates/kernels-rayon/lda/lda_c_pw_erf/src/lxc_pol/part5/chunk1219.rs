//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1219/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1219(t21977: f64, t13515: f64, t13508: f64, t13512: f64, t13518: f64, t21949: f64, t21954: f64, t21958: f64, t21962: f64, t21965: f64, t21968: f64, t21971: f64, t21973: f64, t21975: f64) -> (f64, f64, f64) {
    let t21978 = 8.0_f64 / 15.0_f64 * t21977;
    let t21979 = 16.0_f64 / 135.0_f64 * t13515;
    let t21980 = -t21949 + t21954 - t21958 - t21962 + t21965 - t21968 + t21971 + t21973 + t21975 + t21978 + t13508 - t13512 + t21979 - t13518;
    (t21978, t21979, t21980)
}
