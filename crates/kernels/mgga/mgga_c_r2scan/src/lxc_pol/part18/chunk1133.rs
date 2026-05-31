//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1133/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1133<F: Float>(t12556: F, t1561: F, t3275: F, t3277: F, t31510: F, t795: F, t3263: F, t105: F, t3052: F, t97: F, t10669: F, t2526: F, t3574: F) -> (F, F, F, F) {
    let t42380 = t1561 * t12556;
    let t42383 = F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t3275 * t42380 * t3277;
    let t42384 = t31510 * t795;
    let t42387 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t3275 * t3263 * t42384;
    let t42389 = t97 * t105 * t3052;
    let t42391 = F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t42389 * t10669;
    let t42392 = t3574 * t2526;
    (t42383, t42387, t42391, t42392)
}
