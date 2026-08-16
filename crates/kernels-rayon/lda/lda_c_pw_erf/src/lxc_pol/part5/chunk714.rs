//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 714/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk714(t1472: f64, t2389: f64, t2065: f64, t816: f64, t1308: f64, t571: f64, t1954: f64, t833: f64, t4841: f64, t2415: f64, t549: f64, t1319: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6395 = 8.0_f64 / 45.0_f64 * t1472 * t2389;
    let t6396 = t816 * t2065;
    let t6397 = t1308 * t6396;
    let t6399 = 8.0_f64 / 45.0_f64 * t571 * t6397;
    let t6400 = t1954 * t833;
    let t6401 = t4841 * t6400;
    let t6403 = 16.0_f64 / 45.0_f64 * t571 * t6401;
    let t6404 = t2415 * t549;
    let t6405 = t1319 * t6404;
    (t6395, t6396, t6397, t6399, t6400, t6401, t6403, t6404, t6405)
}
