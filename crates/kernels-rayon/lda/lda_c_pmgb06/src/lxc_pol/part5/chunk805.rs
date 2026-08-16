//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 805/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk805(t6522: f64, t760: f64, t2864: f64, t439: f64, t3249: f64, t7295: f64, t3248: f64, t493: f64, t2002: f64, t2481: f64, t2485: f64, t1962: f64, t2480: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7562 = t6522 * t760;
    let t7563 = t2864 * t7562;
    let t7565 = 2.0_f64 / 15.0_f64 * t439 * t7563;
    let t7566 = t3249 * t7295;
    let t7567 = t3248 * t7566;
    let t7569 = 8.0_f64 / 81.0_f64 * t493 * t7567;
    let t7571 = t2002 * t2481 / 15.0_f64;
    let t7573 = t2002 * t2485 / 9.0_f64;
    let t7574 = t1962 * t2480;
    (t7562, t7563, t7565, t7566, t7567, t7569, t7571, t7573, t7574)
}
