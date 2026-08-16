//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1337/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1337(t10409: f64, t10412: f64, t10414: f64, t22084: f64, t22086: f64, t22088: f64, t22093: f64, t22098: f64, t22102: f64, t22107: f64, t22109: f64, t22111: f64, t22113: f64) -> f64 {
    let t23289 = -t22084 + t22086 + t22088 + t22093 - t22098 + t22102 - t22107 - t22109 + t22111 - t22113 + 4.0_f64 * t10409 + t10412 + 0.0011033703703703704_f64 * t10414;
    t23289
}
