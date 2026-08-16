//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 186/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk186(t493: f64, t500: f64, t138: f64, t163: f64, t449: f64, t139: f64, t165: f64) -> (f64, f64, f64, f64) {
    let t502 = t493 * t500 / 45.0_f64;
    let t504 = t138 * t449 * t163;
    let t505 = 0.0018891666666666666_f64 * t504;
    let t506 = t139 * t165;
    (t502, t504, t505, t506)
}
