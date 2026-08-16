//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 970/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk970(t10868: f64, t2608: f64, t2147: f64, t1055: f64, t2834: f64, t10699: f64, t10712: f64, t11641: f64, t11644: f64, t11647: f64, t11650: f64, t11652: f64, t11655: f64, t11657: f64) -> (f64, f64, f64) {
    let t11659 = t10868 * t2608;
    let t11660 = t2147 * t11659;
    let t11663 = t2834 * t1055;
    let t11665 = 0.64025200389650807209e-1_f64 * t10699 - 0.43663693315433241792e-2_f64 * t11641 - 0.65495539973149862688e-2_f64 * t11644 - 0.65495539973149862688e-2_f64 * t11647 - 0.26198215989259945075e-1_f64 * t11650 + 0.21831846657716620896e-2_f64 * t11652 + 0.21831846657716620896e-2_f64 * t11655 - 0.11557628986739024751e0_f64 * t11657 - 0.23287303101564395623e-1_f64 * t11660 + 0.71414953796510926458e-2_f64 * t10712 + 0.43341108700271342816e-1_f64 * t11663;
    (t11659, t11660, t11665)
}
