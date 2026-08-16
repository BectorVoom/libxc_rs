//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 976/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk976(t8191: f64, t8195: f64, t1765: f64, t2942: f64, t1070: f64, t1775: f64, t1067: f64, t1799: f64, t8197: f64, t8199: f64, t8204: f64, t8206: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11333 = 48.0_f64 * t8191;
    let t11334 = 72.0_f64 * t8195;
    let t11335 = t1765 * t2942;
    let t11336 = 3.5089340384731225_f64 * t11335;
    let t11337 = t1070 * t1775;
    let t11338 = 96.0_f64 * t11337;
    let t11339 = t1067 * t1799;
    let t11340 = 36.0_f64 * t11339;
    let t11341 = 480.0_f64 * t8197;
    let t11342 = 144.0_f64 * t8199;
    let t11343 = 240.0_f64 * t8204;
    let t11344 = 12.0_f64 * t8206;
    (t11333, t11334, t11336, t11338, t11340, t11341, t11342, t11343, t11344)
}
