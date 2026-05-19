//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1090/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1090<F: Float>(t10710: F, t25480: F, t37658: F, t25486: F, t37582: F, t10776: F, t10810: F, t2563: F, t2650: F, t546: F, t565: F, t10698: F, t2559: F) -> (F, F, F, F, F, F) {
    let t39355 = t37658 * t10710 * t25480;
    let t39358 = t37582 * t10710 * t25486;
    let t39361 = t10776 * t10810 * t2563;
    let t39362 = F::cast_from(0.23115257973478049502e0_f64) * t39361;
    let t39375 = t546 * t2650;
    let t39378 = t565 * t2650;
    let t39395 = t10698 * t2559;
    (t39355, t39358, t39362, t39375, t39378, t39395)
}
