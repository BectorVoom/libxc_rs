//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 898/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk898<F: Float>(t1898: F, t2002: F, t1902: F, t1893: F, t5482: F, t439: F, t153: F, t1962: F, t1864: F, t4619: F, t1859: F, t2386: F, t2918: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6488 = F::new(4.0) / F::new(45.0) * t2002 * t1898;
    let t6490 = F::new(2.0) / F::new(27.0) * t2002 * t1902;
    let t6491 = t5482 * t1893;
    let t6493 = F::new(2.0) / F::new(45.0) * t439 * t6491;
    let t6494 = t1962 * t153;
    let t6495 = t6494 * t1864;
    let t6497 = F::new(4.0) / F::new(45.0) * t439 * t6495;
    let t6498 = t4619 * t153;
    let t6499 = t6498 * t1859;
    let t6501 = F::new(2.0) / F::new(27.0) * t439 * t6499;
    let t6502 = t2918 * t2386;
    (t6488, t6490, t6491, t6493, t6494, t6495, t6497, t6498, t6499, t6501, t6502)
}
