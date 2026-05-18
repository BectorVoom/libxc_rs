//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1221/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1221<F: Float>(t30053: F, t3308: F, t5136: F, t30057: F, t6218: F, t11711: F, t8240: F, t11714: F, t7383: F, t10856: F, t9377: F, t38069: F, t40107: F, t41699: F, t43559: F, t43561: F, t43565: F, t43569: F) -> F {
    let t43572 = t5136 * t3308 * t30053;
    let t43575 = t6218 * t3308 * t30057;
    let t43577 = t8240 * t11711;
    let t43579 = t7383 * t11714;
    let t43581 = t10856 * t9377;
    let t43583 = -F::new(0.13002332610081402845e0) * t43559 - F::new(0.2600466522016280569e0) * t43561 - t38069 - F::new(0.13002332610081402845e0) * t43565 + F::new(0.58544643236296698112e-1) * t40107 - F::new(0.27439371595564631661e-1) * t43569 - F::new(0.2600466522016280569e0) * t43572 - F::new(0.2600466522016280569e0) * t43575 + F::new(0.2600466522016280569e0) * t43577 + F::new(0.10401866088065122276e1) * t43579 + t41699 - F::new(0.48787202696913915093e-2) * t43581;
    t43583
}
