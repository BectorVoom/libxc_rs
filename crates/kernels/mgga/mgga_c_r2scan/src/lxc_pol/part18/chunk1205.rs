//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1205/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1205<F: Float>(t30213: F, t3332: F, t7628: F, t12543: F, t22731: F, t27996: F, t6165: F, t28000: F, t22868: F, t30292: F, t26185: F, t30296: F) -> (F, F, F, F, F, F) {
    let t43379 = t7628 * t3332 * t30213;
    let t43381 = t22731 * t12543;
    let t43384 = t6165 * t3332 * t27996;
    let t43387 = t6165 * t3332 * t28000;
    let t43390 = t22868 * t3332 * t30292;
    let t43393 = t26185 * t3332 * t30296;
    (t43379, t43381, t43384, t43387, t43390, t43393)
}
