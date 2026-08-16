//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 143/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk143(t358: f64, t40: f64, t67: f64, t62: f64, t323: f64, t325: f64, t329: f64, t331: f64) -> (f64, f64, f64, f64, f64) {
    let t359 = t40 * t358;
    let t363 = t67 * t67;
    let t364 = 1.0_f64 / t363;
    let t365 = t62 * t364;
    let t370 = -1.176575_f64 * t323 - 0.516475_f64 * t325 - 0.2103875_f64 * t329 - 0.104195_f64 * t331;
    (t359, t363, t364, t365, t370)
}
