//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 731/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk731<F: Float>(t1308: F, t4872: F, t571: F, t2193: F, t3416: F, t1450: F, t2171: F, t2098: F, t529: F, t494: F, t1440: F, t1325: F, t1390: F, t542: F, t519: F, t1476: F, t2146: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4873 = t1308 * t4872;
    let t4875 = 4.0 / 45.0 * t571 * t4873;
    let t4877 = 8.0 / 15.0 * t3416 * t2193;
    let t4879 = 16.0 / 135.0 * t2171 * t1450;
    let t4880 = t529 * t2098;
    let t4881 = t4880 * t494;
    let t4882 = t1440 * t4881;
    let t4884 = 8.0 / 15.0 * t1325 * t4882;
    let t4885 = t1390 * t2098;
    let t4886 = t4885 * t542;
    let t4887 = t1440 * t4886;
    let t4889 = 8.0 / 15.0 * t519 * t4887;
    let t4891 = 16.0 / 135.0 * t2146 * t1476;
    (t4873, t4875, t4877, t4879, t4881, t4882, t4884, t4886, t4887, t4889, t4891)
}
