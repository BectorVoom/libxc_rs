//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 911/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk911<F: Float>(t11659: F, t2147: F, t1055: F, t2834: F, t10699: F, t10712: F, t11641: F, t11644: F, t11647: F, t11650: F, t11652: F, t11655: F, t11657: F, t20: F, t5119: F, t3293: F) -> (F, F, F) {
    let t11660 = t2147 * t11659;
    let t11663 = t2834 * t1055;
    let t11665 = 0.64025200389650807209e-1 * t10699 - 0.43663693315433241792e-2 * t11641 - 0.65495539973149862688e-2 * t11644 - 0.65495539973149862688e-2 * t11647 - 0.26198215989259945075e-1 * t11650 + 0.21831846657716620896e-2 * t11652 + 0.21831846657716620896e-2 * t11655 - 0.11557628986739024751e0 * t11657 - 0.23287303101564395623e-1 * t11660 + 0.71414953796510926458e-2 * t10712 + 0.43341108700271342816e-1 * t11663;
    let t11669 = t5119 * t20;
    let t11670 = t3293 * t11669;
    (t11665, t11669, t11670)
}
