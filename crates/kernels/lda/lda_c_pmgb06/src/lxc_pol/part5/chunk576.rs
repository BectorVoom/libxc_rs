//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 576/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk576<F: Float>(t3725: F, t696: F, t683: F, t957: F, t978: F, t1179: F, t282: F, t55: F, t691: F, t674: F, t962: F) -> (F, F, F, F, F, F) {
    let t3727 = F::cast_from(51.94757731704439_f64) * t696 * t3725;
    let t3729 = t978 * t957 * t683;
    let t3731 = F::cast_from(3.5089341735807875_f64) * t696 * t3729;
    let t3734 = t55 * t1179 * t282;
    let t3736 = F::cast_from(0.0005696894717424259_f64) * t691 * t3734;
    let t3738 = F::cast_from(1.0_f64) / t962 / t674;
    (t3727, t3729, t3731, t3734, t3736, t3738)
}
