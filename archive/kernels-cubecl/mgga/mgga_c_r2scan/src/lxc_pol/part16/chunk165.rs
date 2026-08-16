//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 165/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk165<F: Float>(t122: F, t503: F, t507: F, t118: F) -> (F, F, F) {
    let t509 = F::cast_from(0.27439371595564631661e-2_f64) * t503 * t122 * t507;
    let t510 = t118 * t118;
    let t511 = F::cast_from(1.0_f64) / t510;
    (t509, t510, t511)
}
