//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1176/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1176(t9947: f64, t184: f64, t2441: f64, t494: f64, t786: f64, t2067: f64, t2425: f64, t784: f64, t793: f64, t2131: f64, t493: f64, t514: f64, t7798: f64) -> (f64, f64, f64, f64, f64) {
    let t21432 = 16.0_f64 / 405.0_f64 * t9947;
    let t21436 = 4.0_f64 / 5.0_f64 * t494 * t2441 * t184 * t786;
    let t21438 = 2.0_f64 / 5.0_f64 * t2425 * t2067;
    let t21440 = t784 * t793 * t184;
    let t21442 = 8.0_f64 / 5.0_f64 * t21440 * t2131;
    let t21444 = t493 * t514 * t7798;
    (t21432, t21436, t21438, t21442, t21444)
}
