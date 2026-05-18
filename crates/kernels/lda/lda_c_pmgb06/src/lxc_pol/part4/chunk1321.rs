//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1321/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1321<F: Float>(t161: F, t166: F, t17321: F, t17333: F, t17347: F, t17361: F, t176: F, t1499: F, t2555: F, t486: F, t6833: F, t5051: F, t802: F) -> (F, F, F, F) {
    let t17367 = t161 * t166 * (t17321 + t17333 + t17347 + t17361) * t176 / F::new(30.0);
    let t17369 = t1499 * t2555 / F::new(30.0);
    let t17371 = t486 * t6833 / F::new(15.0);
    let t17372 = t802 * t5051;
    (t17367, t17369, t17371, t17372)
}
