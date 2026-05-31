//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1197/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1197<F: Float>(t106: F, t7194: F, t97: F, t3271: F, t10615: F, t11531: F, t3275: F, t3579: F, t37257: F, t11621: F, t37292: F, t11560: F, t37271: F) -> (F, F, F, F, F) {
    let t40358 = t97 * t106 * t7194;
    let t40360 = t40358 * t3271 / F::cast_from(4.0_f64);
    let t40363 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t3275 * t10615 * t11531;
    let t40365 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t3579 * t37257;
    let t40368 = F::cast_from(45.0_f64) / F::cast_from(32.0_f64) * t3275 * t37292 * t11621;
    let t40370 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t37271 * t11560;
    (t40360, t40363, t40365, t40368, t40370)
}
