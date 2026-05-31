//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 895/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk895<F: Float>(t106: F, t797: F, t9563: F, t97: F, t2266: F, t6955: F, t910: F, t3245: F, t6897: F) -> (F, F, F) {
    let t9566 = t97 * t106 * t9563 * t797;
    let t9568 = t2266 * t6955 * t910;
    let t9569 = F::cast_from(6.0_f64) * t9568;
    let t9573 = t3245 * t6897;
    (t9566, t9569, t9573)
}
