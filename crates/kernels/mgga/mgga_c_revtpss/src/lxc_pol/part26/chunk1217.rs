//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1217/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1217<F: Float>(t94383: F, t96221: F, t213: F, t26333: F, t2453: F, t26264: F, t9676: F, t26072: F, t26271: F, t26231: F, t94921: F, t10073: F, t1444: F, t2102: F, t25929: F) -> (F, F, F, F, F, F) {
    let t96510 = t94383 * t96221;
    let t96512 = t213 * t26333;
    let t96515 = t2453 * t26264;
    let t96516 = t96515 * t9676;
    let t96527 = t26072 * t26271;
    let t96542 = t94921 * t26231;
    let t96546 = t10073 * t25929 * t2102 * t1444;
    (t96510, t96512, t96516, t96527, t96542, t96546)
}
