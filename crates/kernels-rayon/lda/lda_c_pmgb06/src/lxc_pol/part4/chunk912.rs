//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 912/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk912(t477: f64, t6637: f64, t6636: f64, t5077: f64, t332: f64, t5094: f64, t5084: f64, t5083: f64, t2563: f64, t513: f64, t1848: f64, t844: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6638 = t6637 * t477;
    let t6639 = t6636 * t6638;
    let t6641 = 4.0_f64 / 45.0_f64 * t5077 * t6639;
    let t6642 = t6637 * t332;
    let t6643 = t5094 * t6642;
    let t6645 = 4.0_f64 / 45.0_f64 * t5077 * t6643;
    let t6646 = t5084 * t6642;
    let t6648 = 2.0_f64 / 27.0_f64 * t5083 * t6646;
    let t6650 = t2563 * t513 / 30.0_f64;
    let t6652 = t1848 * t844 / 15.0_f64;
    (t6638, t6639, t6641, t6642, t6643, t6645, t6646, t6648, t6650, t6652)
}
