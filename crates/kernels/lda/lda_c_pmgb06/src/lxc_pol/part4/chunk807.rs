//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 807/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk807<F: Float>(t1915: F, t4852: F, t176: F, t3238: F, t1821: F, t1919: F, t4880: F, t3248: F) -> (F, F, F, F, F) {
    let t5458 = t1915 * t4852;
    let t5463 = t3238 * t176;
    let t5464 = t5463 * t1821;
    let t5467 = t1919 * t4880;
    let t5470 = t3248 * t176;
    (t5458, t5463, t5464, t5467, t5470)
}
