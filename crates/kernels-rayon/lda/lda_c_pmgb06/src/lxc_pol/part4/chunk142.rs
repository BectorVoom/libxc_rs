//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 142/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk142(t349: f64, t350: f64, t342: f64, t38: f64, t56: f64, t109: f64, t54: f64, t55: f64, t30: f64, t53: f64) -> (f64, f64, f64, f64) {
    let t352 = 0.48717083333333333_f64 * t349 * t350;
    let t355 = 2.923025_f64 * t38 * t56 * t342;
    let t359 = t54 * t55 * t109 * t56 / 12.0_f64;
    let t360 = t53 * t30;
    (t352, t355, t359, t360)
}
