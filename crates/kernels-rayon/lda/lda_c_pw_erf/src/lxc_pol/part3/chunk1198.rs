//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1198/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1198(t14066: f64, t14070: f64, t14072: f64, t14074: f64, t14076: f64, t14078: f64, t14083: f64, t14088: f64, t14090: f64, t14093: f64, t14096: f64, t14099: f64, t14100: f64) -> f64 {
    let t14102 = t14066 + t14070 - t14072 + t14074 + t14076 - t14078 - t14083 - t14088 + t14090 + 0.3246312408709453_f64 * t14093 + t14096 + t14099 + 0.3246312408709453_f64 * t14100;
    t14102
}
