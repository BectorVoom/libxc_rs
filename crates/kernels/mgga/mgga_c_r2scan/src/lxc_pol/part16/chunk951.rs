//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 951/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk951<F: Float>(t11031: F, t502: F, t818: F, t826: F, t1275: F, t263: F, t1271: F, t3366: F, t1276: F, t1266: F) -> (F, F, F, F, F, F, F, F) {
    let t11032 = F::new(11.0) / F::new(9.0) * t11031;
    let t11033 = t502 * t818;
    let t11034 = t11033 * t826;
    let t11036 = t263 * t1275;
    let t11045 = t1271 * t3366;
    let t11050 = t3366 * t826;
    let t11051 = t1276 * t11050;
    let t11056 = param_eta * t1266;
    (t11032, t11033, t11034, t11036, t11045, t11050, t11051, t11056)
}
