//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 748/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk748(t35: f64, t3521: f64, t3523: f64, t3525: f64, t3531: f64, t3569: f64, t3583: f64, t360: f64, t7024: f64, t7026: f64, t7027: f64, t7031: f64, t7035: f64) -> f64 {
    let t7039 = -t3521 - t3523 + t3525 - 2.0_f64 / 9.0_f64 * t3531 - 0.48968_f64 * t3569 + t7024 - 0.97936_f64 * t3583 - t7026 + 3.0_f64 * t360 * t35 * t7027 + 3.0_f64 / 2.0_f64 * t360 * t35 * t7031 - 6.0_f64 * t360 * t35 * t7035;
    t7039
}
