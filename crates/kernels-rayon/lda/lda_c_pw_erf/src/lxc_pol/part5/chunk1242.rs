//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1242/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1242(t1318: f64, t1466: f64, t22318: f64, t549: f64, t571: f64, t593: f64, t7513: f64, t9237: f64, t1325: f64, t3787: f64, t7588: f64, t18608: f64, t826: f64) -> (f64, f64, f64, f64) {
    let t22322 = 8.0_f64 / 5.0_f64 * t1318 * t1466 * t22318 * t549;
    let t22327 = 16.0_f64 / 5.0_f64 * t571 * t1466 * t9237 * t7513 * t593;
    let t22329 = t1325 * t3787 * t7588;
    let t22330 = 8.0_f64 / 15.0_f64 * t22329;
    let t22332 = 8.0_f64 / 15.0_f64 * t18608 * t826;
    (t22322, t22327, t22330, t22332)
}
