//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1076/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1076<F: Float>(t11714: F, t7383: F, t10856: F, t9377: F, t38069: F, t40107: F, t41699: F, t43559: F, t43561: F, t43565: F, t43569: F, t43572: F, t43575: F, t43577: F, t37769: F, t9373: F) -> (F, F) {
    let t43579 = t7383 * t11714;
    let t43581 = t10856 * t9377;
    let t43583 = -0.13002332610081402845e0 * t43559 - 0.2600466522016280569e0 * t43561 - t38069 - 0.13002332610081402845e0 * t43565 + 0.58544643236296698112e-1 * t40107 - 0.27439371595564631661e-1 * t43569 - 0.2600466522016280569e0 * t43572 - 0.2600466522016280569e0 * t43575 + 0.2600466522016280569e0 * t43577 + 0.10401866088065122276e1 * t43579 + t41699 - 0.48787202696913915093e-2 * t43581;
    let t43586 = t37769 * t9373;
    (t43583, t43586)
}
