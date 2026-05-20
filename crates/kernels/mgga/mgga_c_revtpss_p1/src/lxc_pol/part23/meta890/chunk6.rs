//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2837/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2837<F: Float>(t10905: F, t23275: F, t10777: F, t10779: F, t6035: F, t61956: F, t1559: F, t40725: F, t5988: F, t14923: F, t23301: F, t125: F, t23114: F) -> (F, F, F, F, F) {
    let t76677 = t10905 * t23275;
    let t76689 = t10777 * t10779 * t61956 * t6035;
    let t76701 = t10777 * t40725 * t5988 * t1559;
    let t76703 = t14923 * t23301;
    let t76705 = t125 * t23114;
    (t76677, t76689, t76701, t76703, t76705)
}
