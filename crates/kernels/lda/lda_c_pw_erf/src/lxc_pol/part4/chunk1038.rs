//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1038/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1038<F: Float>(t1191: F, t780: F, t1138: F, t1597: F, t485: F, t5932: F, t1904: F, t717: F, t2916: F, t5466: F, t1064: F, t1775: F, t1067: F, t1765: F, t2737: F, t1081: F, t5701: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14397 = t1191 * t780;
    let t14399 = t14397 * t1138 * t1597;
    let t14401 = t5932 * t485;
    let t14403 = t717 * t1904;
    let t14405 = t14403 * t1138 * t1597;
    let t14408 = t5466 * t2916 * t1597;
    let t14435 = t1064 * t1775;
    let t14443 = t1067 * t1775;
    let t14445 = t1765 * t2737;
    let t14447 = t5701 * t1081;
    (t14397, t14399, t14401, t14403, t14405, t14408, t14435, t14443, t14445, t14447)
}
