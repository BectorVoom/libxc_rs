//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1110/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1110<F: Float>(t3348: F, t910: F, t3270: F, t10667: F, t11496: F, t2262: F, t3262: F, t3263: F, t3618: F, t792: F, t11002: F, t3269: F) -> (F, F, F) {
    let t39323 = t3348 * t910;
    let t39324 = t3270 * t39323;
    let t39326 = F::new(3.0) / F::new(2.0) * t10667 * t39324;
    let t39327 = t11496 * t2262;
    let t39330 = F::new(3.0) / F::new(4.0) * t3262 * t3263 * t39327;
    let t39331 = t3618 * t792;
    let t39332 = t11002 * t39331;
    let t39334 = F::new(5.0) / F::new(8.0) * t3269 * t39332;
    (t39326, t39330, t39334)
}
