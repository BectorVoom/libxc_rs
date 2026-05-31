//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 277/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk277<F: Float>(t942: F, t945: F, t947: F, t951: F, t953: F, t955: F) -> F {
    let t957 = -F::cast_from(0.5753888888888888_f64) * t942 + F::cast_from(1.1507777777777777_f64) * t945 + F::cast_from(0.4025666666666667_f64) * t947 + F::cast_from(0.0366775_f64) * t951 + F::cast_from(0.073355_f64) * t953 + F::cast_from(0.137975_f64) * t955;
    t957
}
