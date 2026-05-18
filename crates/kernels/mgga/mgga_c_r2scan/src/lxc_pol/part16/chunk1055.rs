//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1055/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1055<F: Float>(t10648: F, t10651: F, t37453: F, t10972: F, t37373: F, t37369: F, t10977: F, t10981: F, t37372: F, t122: F, t607: F, t10928: F, t3434: F, t874: F) -> (F, F, F, F, F) {
    let t37455 = t10648 * t37453 * t10651;
    let t37458 = t37373 * t10972;
    let t37459 = F::new(0.45731474687362542471e-3) * t37458;
    let t37460 = t37369 * t10972;
    let t37461 = F::new(0.45731474687362542471e-3) * t37460;
    let t37463 = t37372 * t10977 * t10981;
    let t37464 = F::new(0.65053455985619242968e-4) * t37463;
    let t37465 = t607 * t122;
    let t37468 = t3434 * t10928 * t37465 * t874;
    (t37455, t37459, t37461, t37464, t37468)
}
