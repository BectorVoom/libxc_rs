//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 203/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk203(t188: f64, t540: f64, t184: f64, t27: f64, t186: f64, t34: f64, t55: f64) -> (f64, f64, f64) {
    let t542 = 4.0_f64 / 3.0_f64 * t540 * t188;
    let t543 = t184 * t27;
    let t545 = t55 * t34 * t186;
    (t542, t543, t545)
}
