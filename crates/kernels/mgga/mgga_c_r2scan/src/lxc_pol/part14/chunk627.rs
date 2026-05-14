//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 627/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk627<F: Float>(t322: F, t1013: F, t1120: F, t1300: F, t327: F, t3509: F, t3730: F, t834: F, t330: F, t1018: F, t1125: F, t3729: F) -> (F, F, F, F, F, F) {
    let t332 = 0.25e1 < t322;
    let t3735 = t1120 * t1013;
    let t3740 = -0.64e0 * t3730 * t327 - 0.128e1 * t3509 * t1013 - 0.128e1 * t1300 * t3735 - 0.64e0 * t834 * t3730;
    let t3741 = t3740 * t330;
    let t3742 = t1125 * t1018;
    let t3743 = t3742 * t330;
    let t3745 = piecewise3(t332, 0.0, t3729);
    (t3735, t3740, t3741, t3742, t3743, t3745)
}
