//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 461/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk461<F: Float>(t5: F, t153: F, t1872: F, t137: F, t132: F, t460: F, t802: F, t332: F, t760: F, t1: F, t395: F, t44: F, t131: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t1873 = t1872 * t153;
    let t1874 = t137 * t1873;
    let t1876 = t132 * t1874 / 30.0;
    let t1878 = t802 * t460 / 30.0;
    let t1879 = t332 * t760;
    let t1881 = t5 * t1;
    let t1885 = piecewise3(t6, 0.0, 4.0 * t1881 * t395 + 2.0 * t1879);
    let t1886 = t1885 * t44;
    let t1887 = t1886 * t131;
    (t1873, t1874, t1876, t1878, t1879, t1881, t1886, t1887)
}
