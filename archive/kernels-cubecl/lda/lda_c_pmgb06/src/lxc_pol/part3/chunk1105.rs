//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1105/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1105<F: Float>(t3190: F, t439: F, t4619: F, t1966: F, t3033: F, t822: F, t9647: F, t1444: F, t4589: F, t1972: F, t3204: F, t1962: F, t3011: F) -> (F, F, F, F, F) {
    let t13144 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t439 * t4619 * t3190;
    let t13149 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t439 * t1966 * t9647 * t822 * t3033;
    let t13151 = t1444 * t4589 / F::cast_from(9.0_f64);
    let t13153 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1972 * t3204;
    let t13156 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t439 * t1962 * t3011;
    (t13144, t13149, t13151, t13153, t13156)
}
