//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 635/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk635(t3703: f64, t682: f64, t963: f64, t696: f64, t278: f64, t962: f64) -> (f64, f64, f64) {
    let t3705 = t963 * t3703 * t682;
    let t3707 = 3.5089341735807875_f64 * t696 * t3705;
    let t3709 = 1.0_f64 / t962 / t278;
    (t3705, t3707, t3709)
}
