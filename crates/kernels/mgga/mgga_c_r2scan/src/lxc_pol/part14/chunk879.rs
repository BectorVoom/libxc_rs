//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 879/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk879<F: Float>(t11033: F, t826: F, t1275: F, t263: F, t1277: F, t1289: F, t3358: F, t1070: F, t6651: F, t3363: F, t6654: F, t1271: F, t3366: F, t6661: F, t1276: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11034 = t11033 * t826;
    let t11035 = 2.0 / 3.0 * t11034;
    let t11036 = t263 * t1275;
    let t11037 = t11036 * t1277;
    let t11039 = t3358 * t1289;
    let t11041 = t6651 * t1070;
    let t11043 = t6654 * t3363;
    let t11045 = t1271 * t3366;
    let t11046 = 2.0 / 3.0 * t11045;
    let t11047 = t1070 * t1277;
    let t11048 = t6661 * t11047;
    let t11050 = t3366 * t826;
    let t11051 = t1276 * t11050;
    let t11052 = 4.0 / 3.0 * t11051;
    let t11053 = t1070 * t1289;
    let t11054 = t1276 * t11053;
    (t11034, t11035, t11036, t11037, t11039, t11041, t11043, t11045, t11046, t11047, t11048, t11050, t11051, t11052, t11053, t11054)
}
