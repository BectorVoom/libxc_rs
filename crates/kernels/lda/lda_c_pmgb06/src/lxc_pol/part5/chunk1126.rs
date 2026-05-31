//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1126/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1126<F: Float>(t16964: F, t16966: F, t16968: F, t16970: F, t16992: F, t17004: F, t17006: F, t432: F, t7736: F, t486: F, t7808: F, t10185: F, t161: F, t166: F, t7806: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t20515 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t16964;
    let t20516 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t16966;
    let t20517 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t16968;
    let t20518 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t16970;
    let t20519 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t16992;
    let t20520 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t17004;
    let t20521 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t17006;
    let t20523 = t432 * t7736 / F::cast_from(10.0_f64);
    let t20525 = t486 * t7808 / F::cast_from(5.0_f64);
    let t20529 = t161 * t166 * t10185 * t7806 / F::cast_from(5.0_f64);
    (t20515, t20516, t20517, t20518, t20519, t20520, t20521, t20523, t20525, t20529)
}
