//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1163/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1163<F: Float>(t23860: F, t30270: F, t39413: F, t39418: F, t49240: F, t49242: F, t49393: F, t49395: F, t56969: F, t57027: F, t57037: F, t57041: F, t57086: F, t787: F, t13939: F, t4793: F) -> (F, F, F) {
    let t57098 = -2.0 / 3.0 * t57027 - 20.0 / 9.0 * t56969 - 16.0 / 27.0 * t39413 + 16.0 / 9.0 * t39418 + t23860 + 8.0 / 9.0 * t49240 - 8.0 / 3.0 * t49242 + 4.0 / 9.0 * t49393 + 40.0 / 81.0 * t49395 + 112.0 / 81.0 * t30270 + 40.0 / 9.0 * t57037 - 80.0 / 81.0 * t57041;
    let t57099 = t57086 + t57098;
    let t57100 = t787 * t57099;
    let t57102 = t13939 * t4793;
    (t57099, t57100, t57102)
}
