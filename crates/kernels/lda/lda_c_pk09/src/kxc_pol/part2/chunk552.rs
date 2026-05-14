//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 552/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk552<F: Float>(t262: F, t4: F, t261: F, t1174: F, t1179: F, t1178: F, t270: F, t4837: F, t1151: F, t266: F, t1161: F, t1197: F, t253: F, t1185: F, t212: F, t1207: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4840 = 1.0 / t262 / t4;
    let t4842 = 0.219824473458288 * t261 * t4840;
    let t4847 = t1174 * t1179;
    let t4852 = 1.0 / t1178 / t270;
    let t4861 = 12.0 * t4837;
    let t4875 = t1151 * t266;
    let t4878 = t1197 * t1161;
    let t4882 = t253 * t1174;
    let t4886 = t1197 * t1185;
    let t4895 = 1.0 / t212;
    let t4910 = t1207 * t1161;
    (t4842, t4847, t4852, t4861, t4875, t4878, t4882, t4886, t4895, t4910)
}
