//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 949/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk949<F: Float>(t3428: F, t3430: F, t6818: F, t260: F, t6100: F, t1102: F, t1104: F, t3314: F, t875: F, t10648: F, t10651: F, t10972: F, t37373: F, t37369: F, t10977: F, t10981: F, t37372: F) -> (F, F, F, F, F, F, F, F) {
    let t37447 = t6818 * t3428 * t3430;
    let t37448 = 0.91462949374725084942e-3 * t37447;
    let t37449 = t260 * t6100;
    let t37451 = t1102 * t37449 * t1104;
    let t37452 = 0.69557008413371175709e-2 * t37451;
    let t37453 = t3314 * t875;
    let t37455 = t10648 * t37453 * t10651;
    let t37458 = t37373 * t10972;
    let t37459 = 0.45731474687362542471e-3 * t37458;
    let t37460 = t37369 * t10972;
    let t37461 = 0.45731474687362542471e-3 * t37460;
    let t37463 = t37372 * t10977 * t10981;
    (t37448, t37449, t37452, t37453, t37455, t37459, t37461, t37463)
}
