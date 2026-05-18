//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1197/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1197<F: Float>(t106: F, t7194: F, t97: F, t3271: F, t10615: F, t11531: F, t3275: F, t3579: F, t37257: F, t11621: F, t37292: F, t11560: F, t37271: F) -> (F, F, F, F, F) {
    let t40358 = t97 * t106 * t7194;
    let t40360 = t40358 * t3271 / F::new(4.0);
    let t40363 = F::new(5.0) / F::new(8.0) * t3275 * t10615 * t11531;
    let t40365 = F::new(5.0) / F::new(8.0) * t3579 * t37257;
    let t40368 = F::new(45.0) / F::new(32.0) * t3275 * t37292 * t11621;
    let t40370 = F::new(5.0) / F::new(8.0) * t37271 * t11560;
    (t40360, t40363, t40365, t40368, t40370)
}
