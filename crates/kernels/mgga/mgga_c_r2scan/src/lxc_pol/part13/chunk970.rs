//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 970/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk970<F: Float>(t1070: F, t6651: F, t3363: F, t6654: F, t1271: F, t3366: F, t1277: F, t6661: F, t826: F, t1276: F, t1289: F, t1266: F, param_eta: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11041 = t6651 * t1070;
    let t11043 = t6654 * t3363;
    let t11045 = t1271 * t3366;
    let t11046 = F::new(2.0) / F::new(3.0) * t11045;
    let t11047 = t1070 * t1277;
    let t11048 = t6661 * t11047;
    let t11050 = t3366 * t826;
    let t11051 = t1276 * t11050;
    let t11052 = F::new(4.0) / F::new(3.0) * t11051;
    let t11053 = t1070 * t1289;
    let t11054 = t1276 * t11053;
    let t11056 = param_eta * t1266;
    (t11041, t11043, t11045, t11046, t11047, t11048, t11050, t11051, t11052, t11053, t11054, t11056)
}
