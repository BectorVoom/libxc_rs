//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 239/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk239(t108: f64, t348: f64, t352: f64, t659: f64, t661: f64, t266: f64, t9: f64) -> (f64, f64) {
    let t665 = (4.0_f64 / 3.0_f64 * t659 * t348 + 4.0_f64 / 3.0_f64 * t661 * t352) * t108;
    let t668 = t266 * t9;
    (t665, t668)
}
