//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 458/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk458<F: Float>(t122: F, t569: F, t886: F, t486: F, t844: F, t350: F, t839: F, t1464: F, t764: F) -> (F, F, F, F) {
    let t1813 = t122 * t569 * t886;
    let t1816 = t486 * t844 / F::new(30.0);
    let t1818 = t350 * t839;
    let t1820 = t1464 * t764;
    (t1813, t1816, t1818, t1820)
}
