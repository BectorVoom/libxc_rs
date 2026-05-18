//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1207/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1207<F: Float>(t11736: F, t11744: F, t10760: F, t20298: F, t30628: F, t20305: F, t29837: F, t11640: F, t30370: F, t11842: F, t2651: F, t10810: F, t574: F, t9292: F) -> (F, F, F, F, F, F) {
    let t43407 = t11744 * t11736;
    let t43410 = t20298 * t10760 * t30628;
    let t43413 = t20305 * t10760 * t29837;
    let t43415 = t30370 * t11640;
    let t43418 = t2651 * t11842;
    let t43421 = t574 * t10810 * t9292;
    (t43407, t43410, t43413, t43415, t43418, t43421)
}
