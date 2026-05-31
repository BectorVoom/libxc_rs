//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 876/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk876<F: Float>(t6175: F, t6224: F, t465: F, t137: F, t132: F, t2553: F, t517: F, t529: F, t166: F, t161: F, t2563: F, t531: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6225 = t6175 + t6224;
    let t6226 = t465 * t6225;
    let t6227 = t137 * t6226;
    let t6229 = t132 * t6227 / F::cast_from(30.0_f64);
    let t6230 = t2553 * t517;
    let t6231 = t6230 * t529;
    let t6232 = t166 * t6231;
    let t6234 = t161 * t6232 / F::cast_from(30.0_f64);
    let t6236 = t2563 * t531 / F::cast_from(30.0_f64);
    (t6225, t6226, t6227, t6229, t6230, t6231, t6232, t6234, t6236)
}
