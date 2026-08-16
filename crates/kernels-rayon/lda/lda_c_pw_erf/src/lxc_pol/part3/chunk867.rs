//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 867/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk867(t260: f64, t34: f64, t343: f64, t262: f64, t3154: f64, t344: f64, t339: f64, t311: f64, t1062: f64, t22: f64, t19: f64, t301: f64, t305: f64, t732: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8315 = 1.0_f64 / t260;
    let t8327 = t34 * t343;
    let t8334 = 1.0_f64 / t262;
    let t8356 = 16.0_f64 * t344 * t3154;
    let t8357 = t339 * t3154;
    let t8359 = t311 * t311;
    let t8363 = 1.0_f64 / t22 / t1062;
    let t8368 = 0.3407285805772476_f64 * t305 / t8359 * t8363 * t301 * t19 * t732;
    (t8315, t8327, t8334, t8356, t8357, t8363, t8368)
}
