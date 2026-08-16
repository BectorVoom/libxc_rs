//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 784/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk784(t1967: f64, t5237: f64, t519: f64, t1446: f64, t2031: f64, t1278: f64, t789: f64, t1313: f64, t1991: f64, t4624: f64, t197: f64, t3893: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5238 = t5237 * t1967;
    let t5240 = 16.0_f64 / 81.0_f64 * t519 * t5238;
    let t5242 = 8.0_f64 / 45.0_f64 * t1446 * t2031;
    let t5243 = t789 * t1278;
    let t5244 = t1313 * t5243;
    let t5246 = 4.0_f64 / 45.0_f64 * t519 * t5244;
    let t5247 = t1991 * t4624;
    let t5249 = 4.0_f64 / 27.0_f64 * t519 * t5247;
    let t5250 = t3893 * t197;
    (t5238, t5240, t5242, t5243, t5244, t5246, t5247, t5249, t5250)
}
