//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 884/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk884<F: Float>(t3610: F, t6395: F, t3332: F, t8066: F, t2147: F, t3316: F, t980: F, t10868: F, t2608: F, t1055: F, t2834: F, t10699: F, t10712: F, t11641: F, t11644: F, t11647: F, t11650: F) -> (F, F, F, F, F) {
    let t11652 = t6395 * t3610;
    let t11654 = t3332 * t8066;
    let t11655 = t2147 * t11654;
    let t11657 = t980 * t3316;
    let t11659 = t10868 * t2608;
    let t11660 = t2147 * t11659;
    let t11663 = t2834 * t1055;
    let t11665 = 0.64025200389650807209e-1 * t10699 - 0.43663693315433241792e-2 * t11641 - 0.65495539973149862688e-2 * t11644 - 0.65495539973149862688e-2 * t11647 - 0.26198215989259945075e-1 * t11650 + 0.21831846657716620896e-2 * t11652 + 0.21831846657716620896e-2 * t11655 - 0.11557628986739024751e0 * t11657 - 0.23287303101564395623e-1 * t11660 + 0.71414953796510926458e-2 * t10712 + 0.43341108700271342816e-1 * t11663;
    (t11654, t11657, t11659, t11660, t11665)
}
