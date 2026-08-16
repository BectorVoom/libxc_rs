//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 909/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk909(t3794: f64, t3860: f64, t3675: f64, t522: f64, t3445: f64, t565: f64, t2104: f64, t3390: f64, t1284: f64, t3564: f64, t514: f64, t548: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9340 = t3794 * t3860;
    let t9351 = t522 * t3675;
    let t9359 = t565 * t3445;
    let t9361 = t2104 * t3390;
    let t9366 = t1284 * t3390;
    let t9369 = t548 * t514 * t3564;
    (t9340, t9351, t9359, t9361, t9366, t9369)
}
