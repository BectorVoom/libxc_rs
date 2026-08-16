//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 810/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk810(t2563: f64, t844: f64, t2469: f64, t4588: f64, t493: f64, t1972: f64, t2466: f64, t498: f64, t7300: f64, t496: f64, t2470: f64, t3189: f64, t7284: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7633 = t2563 * t844 / 10.0_f64;
    let t7634 = t4588 * t2469;
    let t7636 = t493 * t7634 / 9.0_f64;
    let t7638 = t1972 * t2466 / 15.0_f64;
    let t7639 = t498 * t7300;
    let t7640 = t496 * t7639;
    let t7642 = t493 * t7640 / 45.0_f64;
    let t7644 = t1972 * t2470 / 9.0_f64;
    let t7645 = t3189 * t7284;
    (t7633, t7634, t7636, t7638, t7639, t7640, t7642, t7644, t7645)
}
