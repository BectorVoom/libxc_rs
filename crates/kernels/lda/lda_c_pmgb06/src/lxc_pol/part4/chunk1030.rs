//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1030/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1030<F: Float>(t1190: F, t4189: F, t1187: F, t4197: F, t115: F, t8173: F, t247: F, t413: F, t113: F, t642: F, t8193: F, t1321: F) -> (F, F, F, F, F, F, F, F) {
    let t10520 = F::new(0.2508) * t4189 * t1190;
    let t10522 = F::cast_from(0.39013333333333333_f64) * t1187 * t4197;
    let t10524 = t8173 * t115;
    let t10525 = t10524 / F::new(2.0);
    let t10528 = F::cast_from(0.007532237109403992_f64) * t413 * t247 * t115;
    let t10531 = F::cast_from(0.015064474218807983_f64) * t113 * t642 * t115;
    let t10533 = F::new(60.0) * t8193;
    let t10547 = t1321 * t1321;
    (t10520, t10522, t10524, t10525, t10528, t10531, t10533, t10547)
}
