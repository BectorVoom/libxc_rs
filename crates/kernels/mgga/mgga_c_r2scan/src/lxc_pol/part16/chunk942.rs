//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 942/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk942<F: Float>(t352: F, t9769: F, t910: F, t986: F, t113: F, t5086: F, t104: F, t494: F, t97: F, t1299: F, t3370: F, t1074: F, t6692: F, t1275: F, t502: F, t263: F, t6660: F) -> (F, F, F, F, F, F, F, F) {
    let t35220 = t352 * t9769;
    let t35373 = t986 * t910;
    let t36967 = t113 * t5086;
    let t36985 = t104 * t494;
    let t36986 = t97 * t36985;
    let t37020 = t3370 * t1299;
    let t37023 = t1074 * t6692;
    let t37028 = t502 * t1275;
    let t37031 = t263 * t6660;
    (t35220, t35373, t36967, t36986, t37020, t37023, t37028, t37031)
}
