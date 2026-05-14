//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 927/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk927<F: Float>(t12929: F, t374: F, t11364: F, t11365: F, t11367: F, t11585: F, t11589: F, t11593: F, t11604: F, t12738: F, t12741: F, t12744: F, t12748: F, t11368: F, t11372: F, t11374: F, t11375: F, t11377: F, t11378: F, t11379: F, t11616: F, t12752: F, t12815: F, t12817: F) -> (F, F, F) {
    let t12930 = t12929 * t374;
    let t12939 = 0.1440846329149835838e-2 * t11585 + t12738 - t12741 + 0.1440846329149835838e-2 * t11589 - 0.20496175532535769482e-3 * t11593 - t12744 - 0.60975299583150056624e-3 * t11604 + t11364 - t11365 - t12748 + t11367;
    let t12942 = t11368 + t11372 - t11374 + t12752 - t12815 + t12817 + t11375 + t11377 - t11378 - 0.162600798888400151e-2 * t11616 + t11379;
    (t12930, t12939, t12942)
}
