//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1221/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1221(t30053: f64, t3308: f64, t5136: f64, t30057: f64, t6218: f64, t11711: f64, t8240: f64, t11714: f64, t7383: f64, t10856: f64, t9377: f64, t38069: f64, t40107: f64, t41699: f64, t43559: f64, t43561: f64, t43565: f64, t43569: f64) -> f64 {
    let t43572 = t5136 * t3308 * t30053;
    let t43575 = t6218 * t3308 * t30057;
    let t43577 = t8240 * t11711;
    let t43579 = t7383 * t11714;
    let t43581 = t10856 * t9377;
    let t43583 = -0.13002332610081402845e0_f64 * t43559 - 0.2600466522016280569e0_f64 * t43561 - t38069 - 0.13002332610081402845e0_f64 * t43565 + 0.58544643236296698112e-1_f64 * t40107 - 0.27439371595564631661e-1_f64 * t43569 - 0.2600466522016280569e0_f64 * t43572 - 0.2600466522016280569e0_f64 * t43575 + 0.2600466522016280569e0_f64 * t43577 + 0.10401866088065122276e1_f64 * t43579 + t41699 - 0.48787202696913915093e-2_f64 * t43581;
    t43583
}
