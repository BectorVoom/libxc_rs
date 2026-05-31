//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 939/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk939<F: Float>(t35: F, t3521: F, t3523: F, t3525: F, t3531: F, t3569: F, t3583: F, t360: F, t7024: F, t7026: F, t7027: F, t7031: F, t7035: F) -> F {
    let t7039 = -t3521 - t3523 + t3525 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3531 - F::cast_from(0.48968_f64) * t3569 + t7024 - F::cast_from(0.97936_f64) * t3583 - t7026 + F::cast_from(3.0_f64) * t360 * t35 * t7027 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t360 * t35 * t7031 - F::cast_from(6.0_f64) * t360 * t35 * t7035;
    t7039
}
