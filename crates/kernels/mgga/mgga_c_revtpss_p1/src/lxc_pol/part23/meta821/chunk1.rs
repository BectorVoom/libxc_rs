//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2672/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2672<F: Float>(t1058: F, t19858: F, t15688: F, t16509: F, t19869: F, t3201: F, t6318: F, t1011: F, t15987: F, t18926: F, t18930: F, t15689: F, t19985: F, t53405: F) -> (F, F, F, F, F, F, F) {
    let t66093 = t19858 * t1058;
    let t66114 = t16509 * t15688;
    let t66139 = t19869 * t1058;
    let t66141 = t6318 * t3201;
    let t66155 = t1011 * t15987 * t18926;
    let t66158 = t1011 * t15987 * t18930;
    let t66176 = t15689 * t53405 * t19985;
    (t66093, t66114, t66139, t66141, t66155, t66158, t66176)
}
