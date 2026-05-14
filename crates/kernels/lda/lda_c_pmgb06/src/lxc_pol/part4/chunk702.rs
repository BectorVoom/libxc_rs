//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 702/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk702<F: Float>(t1822: F, t350: F, t1083: F, t1820: F, t1476: F, t36: F, t2911: F, t764: F, t1080: F, t2909: F, t1576: F, t4857: F, t4861: F, t525: F, t2082: F, t405: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4878 = t350 * t1822;
    let t4879 = 0.015996296296296297 * t4878;
    let t4880 = t1820 * t1083;
    let t4881 = t1476 * t4880;
    let t4882 = t36 * t4881;
    let t4884 = t2911 * t764;
    let t4885 = t4884 * t1080;
    let t4886 = t2909 * t4885;
    let t4887 = t36 * t4886;
    let t4889 = t1576 * t4857;
    let t4892 = t525 * t4861;
    let t4896 = 0.017777777777777778 * t405 * t2082;
    (t4878, t4879, t4880, t4881, t4882, t4885, t4886, t4887, t4889, t4892, t4896)
}
