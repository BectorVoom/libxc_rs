//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1029/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1029<F: Float>(t15256: F, t11750: F, t11757: F, t11762: F, t11765: F, t1596: F, t2592: F, t1: F, t851: F, t13672: F, t529: F, t6559: F, t11904: F, t6630: F, t12497: F, t5068: F, t6629: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15257 = 4.0 / 45.0 * t15256;
    let t15258 = 4.0 / 45.0 * t11750;
    let t15259 = 4.0 / 135.0 * t11757;
    let t15260 = 4.0 / 45.0 * t11762;
    let t15261 = 2.0 / 45.0 * t11765;
    let t15263 = t2592 * t1596 / 15.0;
    let t15264 = t1 * t851;
    let t15268 = 16.0 / 45.0 * t13672 * t6559 * t15264 * t529;
    let t15270 = 8.0 / 45.0 * t11904 * t6630;
    let t15273 = 8.0 / 45.0 * t5068 * t12497 * t6629;
    (t15257, t15258, t15259, t15260, t15261, t15263, t15264, t15268, t15270, t15273)
}
