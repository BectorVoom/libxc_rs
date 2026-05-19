//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1319/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1319<F: Float>(t17110: F, t17113: F, t17131: F, t17133: F, t17136: F, t17138: F, t17140: F, t17145: F, t17149: F, t17154: F, t17157: F, t17162: F) -> F {
    let t17347 = F::new(0.04534) * t17110 - F::cast_from(0.02518888888888889_f64) * t17113 - F::cast_from(0.05541555555555556_f64) * t17131 + F::cast_from(0.005037777777777778_f64) * t17133 + F::new(0.011335) * t17136 + F::cast_from(0.002518888888888889_f64) * t17138 - F::cast_from(0.0008396296296296296_f64) * t17140 + F::cast_from(0.002518888888888889_f64) * t17145 + F::cast_from(0.0012594444444444445_f64) * t17149 + F::cast_from(0.002099074074074074_f64) * t17154 - F::cast_from(0.007556666666666666_f64) * t17157 - F::cast_from(0.003778333333333333_f64) * t17162;
    t17347
}
