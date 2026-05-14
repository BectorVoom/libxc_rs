//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 697/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk697<F: Float>(t1582: F, t259: F, t546: F, t565: F, t503: F, t6068: F, t2110: F, t3436: F, t22: F, t6: F, t506: F, t2162: F, t3303: F, t545: F) -> (F, F, F, F, F, F, F, F) {
    let t6148 = t1582 * t259;
    let t6149 = t546 * t6148;
    let t6152 = t565 * t6148;
    let t6155 = t503 * t6068;
    let t6159 = t3436 * t2110;
    let t6161 = t22 * t6;
    let t6162 = t506 * t6161;
    let t6164 = 0.14457274399185490173e-4 * t6159 * t2162 * t6162;
    let t6165 = t545 * t3303;
    (t6149, t6152, t6155, t6159, t6161, t6162, t6164, t6165)
}
