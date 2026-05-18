//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1182/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1182<F: Float>(t29418: F, t3293: F, t132: F, t537: F, t7322: F, t1575: F, t25826: F, t3342: F, t571: F, t1054: F, t2139: F, t7356: F) -> (F, F, F, F) {
    let t40194 = t3293 * t29418;
    let t40195 = t132 * t537;
    let t40197 = t40194 * t40195 * t7322;
    let t40201 = t571 * t1575 * t3342 * t25826;
    let t40204 = t2139 * t1054 * t7356;
    (t40195, t40197, t40201, t40204)
}
