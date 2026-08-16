//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 598/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk598(t1245: f64, t494: f64, t940: f64, t1991: f64, t1325: f64, t1459: f64, t529: f64, t1246: f64, t542: f64, t519: f64, t1252: f64, t1326: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3398 = t494 * t1245 * t940;
    let t3399 = t1991 * t3398;
    let t3401 = 8.0_f64 / 9.0_f64 * t1325 * t3399;
    let t3402 = t1459 * t529;
    let t3403 = t1246 * t542;
    let t3404 = t3402 * t3403;
    let t3406 = 4.0_f64 / 9.0_f64 * t519 * t3404;
    let t3407 = t1252 * t494;
    let t3408 = t1326 * t3407;
    (t3398, t3399, t3401, t3402, t3403, t3404, t3406, t3407, t3408)
}
