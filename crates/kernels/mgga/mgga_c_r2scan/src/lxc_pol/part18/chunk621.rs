//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 621/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk621<F: Float>(t322: F, t1013: F, t1074: F, t1300: F, t327: F, t3373: F, t3633: F, t834: F, t330: F, t1018: F, t1079: F, t3632: F) -> (F, F, F, F, F, F) {
    let t332 = 0.25e1 < t322;
    let t3638 = t1074 * t1013;
    let t3643 = -0.64e0 * t3633 * t327 - 0.128e1 * t3373 * t1013 - 0.128e1 * t1300 * t3638 - 0.64e0 * t834 * t3633;
    let t3644 = t3643 * t330;
    let t3645 = t1079 * t1018;
    let t3646 = t3645 * t330;
    let t3648 = piecewise3(t332, 0.0, t3632);
    (t3638, t3643, t3644, t3645, t3646, t3648)
}
