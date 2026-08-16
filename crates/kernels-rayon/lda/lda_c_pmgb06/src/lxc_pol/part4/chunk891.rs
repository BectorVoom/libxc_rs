//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 891/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk891(t1897: f64, t6160: f64, t439: f64, t1901: f64, t6165: f64, t6374: f64, t6378: f64, t6381: f64, t6384: f64, t6386: f64, t6389: f64, t6393: f64, t6397: f64, t6401: f64, t6405: f64, t6409: f64, t6411: f64, t6415: f64) -> (f64, f64, f64, f64, f64) {
    let t6416 = t1897 * t6160;
    let t6418 = 2.0_f64 / 45.0_f64 * t439 * t6416;
    let t6419 = t1901 * t6165;
    let t6421 = t439 * t6419 / 27.0_f64;
    let t6422 = -t6374 + t6378 + t6381 - t6384 - t6386 - t6389 - t6393 + t6397 + t6401 + t6405 + t6409 - t6411 - t6415 - t6418 + t6421;
    (t6416, t6418, t6419, t6421, t6422)
}
