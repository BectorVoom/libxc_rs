//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 808/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk808(t36: f64, t7609: f64, t497: f64, t7300: f64, t506: f64, t2900: f64, t4911: f64, t6800: f64, t6811: f64, t6819: f64, t7596: f64, t7600: f64, t7603: f64, t7607: f64) -> (f64, f64, f64, f64, f64) {
    let t7610 = t36 * t7609;
    let t7612 = t497 * t7300;
    let t7613 = t506 * t7612;
    let t7614 = t36 * t7613;
    let t7616 = t2900 + 0.002518888888888889_f64 * t4911 - 0.0012594444444444445_f64 * t6800 + 0.003778333333333333_f64 * t6811 - 0.0018891666666666666_f64 * t6819 + 0.002099074074074074_f64 * t7596 - 0.007556666666666666_f64 * t7600 + 0.003778333333333333_f64 * t7603 + 0.011335_f64 * t7607 - 0.011335_f64 * t7610 + 0.0018891666666666666_f64 * t7614;
    (t7610, t7612, t7613, t7614, t7616)
}
