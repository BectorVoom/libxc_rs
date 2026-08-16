//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 986/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk986(t1025: f64, t3675: f64, t3868: f64, t633: f64, t964: f64) -> (f64, f64, f64, f64) {
    let t8594 = 578.9512619529313_f64 * t3675 * t3868 * t1025;
    let t8595 = t1025 * t1025;
    let t8598 = 24.0_f64 * t3675 * t8595 * t633;
    let t8599 = t964 * t964;
    (t8594, t8595, t8598, t8599)
}
