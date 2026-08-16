//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 796/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk796(t1480: f64, t2146: f64, t1488: f64, t3727: f64, t826: f64, t1472: f64, t2153: f64, t1287: f64, t2156: f64, t1466: f64, t1318: f64, t1278: f64, t2186: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5348 = 4.0_f64 / 45.0_f64 * t2146 * t1480;
    let t5350 = 4.0_f64 / 27.0_f64 * t2146 * t1488;
    let t5352 = 4.0_f64 / 45.0_f64 * t3727 * t826;
    let t5354 = 16.0_f64 / 45.0_f64 * t1472 * t2153;
    let t5355 = t2156 * t1287;
    let t5356 = t1466 * t5355;
    let t5358 = 4.0_f64 / 15.0_f64 * t1318 * t5356;
    let t5359 = t2186 * t1278;
    (t5348, t5350, t5352, t5354, t5355, t5356, t5358, t5359)
}
