//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1015/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1015<F: Float>(t13911: F, t1423: F, t5226: F, t5254: F, t5211: F, t5295: F, t13892: F, t13894: F, t13896: F, t13899: F, t13904: F, t13906: F, t13908: F, t13910: F, t5248: F, t5264: F) -> (F, F, F, F, F, F, F) {
    let t13912 = 4.0 / 45.0 * t13911;
    let t13913 = t1423 * t5226;
    let t13914 = 8.0 / 45.0 * t13913;
    let t13915 = t1423 * t5254;
    let t13916 = 4.0 / 27.0 * t13915;
    let t13917 = t5211 * t5295;
    let t13918 = 2.0 / 9.0 * t13917;
    let t13919 = -t13892 - t13894 - t13896 + t13899 + t13904 - t13906 - t13908 + t13910 - t13912 - t13914 + t13916 + t13918;
    let t13920 = t5211 * t5248;
    let t13921 = 4.0 / 9.0 * t13920;
    let t13922 = t5211 * t5264;
    (t13912, t13914, t13916, t13918, t13919, t13921, t13922)
}
