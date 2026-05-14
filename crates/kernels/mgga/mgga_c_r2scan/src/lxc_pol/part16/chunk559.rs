//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 559/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk559<F: Float>(t2892: F, t552: F, t551: F, t506: F, t529: F, t938: F) -> (F, F, F, F, F) {
    let t3182 = t552 * t2892;
    let t3183 = t551 * t3182;
    let t3186 = t506 * t2892;
    let t3187 = t529 * t3186;
    let t3190 = t938 * t938;
    (t3182, t3183, t3186, t3187, t3190)
}
