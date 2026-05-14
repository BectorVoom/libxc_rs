//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 999/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk999<F: Float>(t3348: F, t910: F, t3270: F, t10667: F, t11496: F, t2262: F, t3262: F, t3263: F, t3618: F, t792: F, t11002: F, t3269: F, t6967: F, t3275: F, t7040: F, t3276: F) -> (F, F, F, F, F) {
    let t39323 = t3348 * t910;
    let t39324 = t3270 * t39323;
    let t39326 = 3.0 / 2.0 * t10667 * t39324;
    let t39327 = t11496 * t2262;
    let t39330 = 3.0 / 4.0 * t3262 * t3263 * t39327;
    let t39331 = t3618 * t792;
    let t39332 = t11002 * t39331;
    let t39334 = 5.0 / 8.0 * t3269 * t39332;
    let t39335 = t6967 * t2262;
    let t39338 = t3275 * t3263 * t39335 / 2.0;
    let t39339 = t7040 * t792;
    let t39342 = 5.0 / 8.0 * t3275 * t3276 * t39339;
    (t39326, t39330, t39334, t39338, t39342)
}
