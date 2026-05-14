//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 653/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk653<F: Float>(t1828: F, t6196: F, t1798: F, t1897: F, t1947: F, t2042: F, t6501: F, t6505: F, t6522: F, t6319: F, t6325: F, t6547: F, t6464: F, t1852: F, t6287: F, t1800: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7426 = t1828 * t6196;
    let t7430 = t1798 * t6196;
    let t7432 = t1897 * t1947;
    let t7433 = t7432 * t2042;
    let t7437 = 4.0 * t6501;
    let t7438 = 4.0 * t6505;
    let t7442 = 5.333333333333333 * t6522;
    let t7446 = 0.821419393556371 * t6319;
    let t7453 = 0.5476129290375806 * t6325;
    let t7454 = 0.4444444444444444 * t6547;
    let t7459 = 0.18253764301252687 * t6464;
    let t7466 = t1852 * t6287;
    let t7467 = t7466 * t1800;
    (t7426, t7430, t7433, t7437, t7438, t7442, t7446, t7453, t7454, t7459, t7467)
}
