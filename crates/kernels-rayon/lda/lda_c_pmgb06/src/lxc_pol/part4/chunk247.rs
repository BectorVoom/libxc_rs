//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 247/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk247(t12: f64, t336: f64, t764: f64, t763: f64, zeta_threshold: f64) -> (f64, f64) {
    let t13 = t12 <= zeta_threshold;
    let t765 = t336 * t764;
    let t767 = piecewise3(t13, 0.0_f64, 2.0_f64 / 3.0_f64 * t765);
    let t769 = t763 / 2.0_f64 + t767 / 2.0_f64;
    (t765, t769)
}
