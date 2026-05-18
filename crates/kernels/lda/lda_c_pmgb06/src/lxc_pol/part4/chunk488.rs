//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 488/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk488<F: Float>(t132: F, t1928: F, t436: F, t802: F, t489: F, t843: F) -> (F, F, F, F, F) {
    let t1929 = t132 * t1928;
    let t1930 = t1929 / F::new(45.0);
    let t1931 = t802 * t436;
    let t1932 = t1931 / F::new(45.0);
    let t1933 = t489 * t843;
    (t1929, t1930, t1931, t1932, t1933)
}
