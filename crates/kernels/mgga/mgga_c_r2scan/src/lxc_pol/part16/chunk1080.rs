//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1080/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1080<F: Float>(t38297: F, t58: F, t6876: F, t423: F, t2315: F, t597: F, t10680: F, t10977: F, t10981: F, t37368: F, t3436: F, t122: F, t158: F, t166: F, t3434: F) -> (F, F, F, F, F, F) {
    let t38298 = F::new(0.91462949374725084942e-3) * t38297;
    let t38299 = t6876 * t58;
    let t38300 = t38299 * t423;
    let t38301 = t597 * t2315;
    let t38303 = t10680 * t38300 * t38301;
    let t38311 = t37368 * t10977 * t10981;
    let t38312 = F::new(0.65053455985619242968e-4) * t38311;
    let t38317 = t3436 * t6876;
    let t38322 = t3434 * t38317 * t158 * t166 * t2315 * t122;
    (t38298, t38299, t38301, t38303, t38312, t38322)
}
