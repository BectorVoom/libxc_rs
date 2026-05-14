//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1143/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1143<F: Float>(t1083: F, t6759: F, t36: F, t506: F, t350: F, t6802: F, t4641: F, t6808: F, t16905: F, t2909: F, t16910: F, t9507: F, t1476: F, t16354: F, t6813: F, t405: F, t6882: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17160 = t6759 * t1083;
    let t17162 = t36 * t506 * t17160;
    let t17164 = t350 * t6802;
    let t17166 = t4641 * t6808;
    let t17169 = t36 * t2909 * t16905;
    let t17172 = t36 * t9507 * t16910;
    let t17175 = t36 * t1476 * t16354;
    let t17177 = t350 * t6813;
    let t17185 = t405 * t6882;
    (t17160, t17162, t17164, t17166, t17169, t17172, t17175, t17177, t17185)
}
