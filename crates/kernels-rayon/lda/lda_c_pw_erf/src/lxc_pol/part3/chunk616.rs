//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 616/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk616(t1294: f64, t565: f64, t1289: f64, t2104: f64, t1524: f64, t595: f64, t1382: f64, t514: f64, t211: f64, t590: f64, t933: f64, t1378: f64, t331: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3570 = t565 * t1294;
    let t3571 = 8.0_f64 / 15.0_f64 * t3570;
    let t3573 = 4.0_f64 / 5.0_f64 * t2104 * t1289;
    let t3575 = 4.0_f64 / 5.0_f64 * t1524 * t595;
    let t3576 = t514 * t1382;
    let t3577 = t211 * t3576;
    let t3578 = 4.0_f64 / 15.0_f64 * t3577;
    let t3579 = t933 * t590;
    let t3581 = t331 * t1378;
    (t3570, t3571, t3573, t3575, t3576, t3577, t3578, t3579, t3581)
}
