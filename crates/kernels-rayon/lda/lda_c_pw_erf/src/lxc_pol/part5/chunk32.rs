//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 32/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk32(t50: f64, t52: f64, t46: f64, t49: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let cbrt2 = (M_CBRT2 as f64);
    let t51 = t50 <= zeta_threshold;
    let t53 = t52 * t50;
    let t54 = piecewise3(t51, t46, t53);
    let t55 = t49 + t54 - 2.0_f64;
    let t56 = cbrt2;
    (t53, t55, t56)
}
