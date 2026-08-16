//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 27/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk27(t22: f64, t29: f64, t31: f64, t44: f64, t52: f64, t27: f64) -> (f64, f64) {
    let pi = (M_PI as f64);
    let t53 = (3.44851_f64 - pi * t31 * t44 * t29 / t22 / 12.0_f64) * t52;
    let t54 = t53 * t27;
    (t53, t54)
}
