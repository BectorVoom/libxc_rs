//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1134/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1134<F: Float>(t11336: F, t2850: F, t3270: F, t3493: F, t983: F, t11002: F, t3719: F, t481: F, t910: F, t14402: F, t986: F, t39355: F) -> (F, F, F, F, F, F) {
    let t41298 = t3270 * t11336 * t2850;
    let t41326 = t3493 * t983;
    let t41327 = t11002 * t41326;
    let t41336 = t3719 * t481;
    let t41337 = t3270 * t41336;
    let t41343 = t3493 * t910;
    let t41344 = t3270 * t41343;
    let t41347 = t14402 * t986;
    let t41348 = t3270 * t41347;
    let t41352 = F::cast_from(0.28565981518604370584e-1_f64) * t39355;
    (t41298, t41327, t41337, t41344, t41348, t41352)
}
