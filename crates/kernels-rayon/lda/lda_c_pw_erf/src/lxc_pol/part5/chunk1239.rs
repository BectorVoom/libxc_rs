//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1239/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1239(t348: f64, t7639: f64, t1326: f64, t519: f64, t7635: f64, t1991: f64, t2429: f64, t34: f64, t4829: f64, t1318: f64, t1319: f64, t549: f64, t7422: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22277 = t7639 * t348;
    let t22280 = 32.0_f64 / 15.0_f64 * t519 * t1326 * t22277;
    let t22281 = t7635 * t348;
    let t22284 = 16.0_f64 / 3.0_f64 * t519 * t1991 * t22281;
    let t22285 = t2429 * t34;
    let t22288 = 16.0_f64 / 5.0_f64 * t519 * t4829 * t22285;
    let t22292 = 16.0_f64 / 15.0_f64 * t1318 * t1319 * t7422 * t549;
    (t22277, t22280, t22281, t22284, t22285, t22288, t22292)
}
