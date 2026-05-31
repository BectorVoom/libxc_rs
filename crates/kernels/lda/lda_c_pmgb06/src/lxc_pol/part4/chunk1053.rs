//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1053/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1053<F: Float>(t247: F, t28: F, t769: F, t8276: F, t3615: F, t63: F, t370: F, t38: F, t8281: F, t2195: F, t642: F, t2203: F) -> (F, F, F, F, F, F) {
    let t11227 = t769 * t28 * t247;
    let t11228 = t8276 * t11227;
    let t11230 = t63 * t3615;
    let t11234 = t38 * t370;
    let t11237 = t8281 * t11227;
    let t11259 = F::cast_from(16.0_f64) * t2195 * t642;
    let t11282 = F::cast_from(16.0_f64) * t2203 * t642;
    (t11228, t11230, t11234, t11237, t11259, t11282)
}
