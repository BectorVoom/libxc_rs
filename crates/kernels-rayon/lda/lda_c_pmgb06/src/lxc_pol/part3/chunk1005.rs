//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1005/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1005(t9404: f64, t1560: f64, t5220: f64, t1420: f64, t5198: f64, t136: f64, t1540: f64, t1968: f64, t439: f64, t9408: f64, t9410: f64, t9412: f64, t9413: f64, t9417: f64, t9418: f64, t9422: f64) -> (f64, f64, f64, f64, f64) {
    let t11951 = 2.0_f64 / 45.0_f64 * t9404;
    let t11952 = t5220 * t1560;
    let t11953 = 4.0_f64 / 45.0_f64 * t11952;
    let t11955 = 2.0_f64 / 5.0_f64 * t1420 * t5198;
    let t11959 = t439 * t136 * t1540 * t1968 / 5.0_f64;
    let t11963 = -t11951 - t11953 + t11955 + t11959 - t9408 + t9410 + t9412 + 2.0_f64 / 9.0_f64 * t9413 - t9417 + 4.0_f64 / 3.0_f64 * t9418 + 2.0_f64 * t9422;
    (t11951, t11953, t11955, t11959, t11963)
}
