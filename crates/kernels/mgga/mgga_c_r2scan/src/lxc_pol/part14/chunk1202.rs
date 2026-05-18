//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1202/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1202<F: Float>(t3270: F, t41336: F, t10667: F, t11336: F, t37327: F, t40566: F, t3493: F, t910: F, t14402: F, t986: F, t3269: F, t39355: F) -> (F, F, F, F, F) {
    let t41337 = t3270 * t41336;
    let t41339 = F::new(3.0) / F::new(2.0) * t10667 * t41337;
    let t41342 = F::new(15.0) / F::new(8.0) * t37327 * t11336 * t40566;
    let t41343 = t3493 * t910;
    let t41344 = t3270 * t41343;
    let t41346 = F::new(3.0) / F::new(2.0) * t10667 * t41344;
    let t41347 = t14402 * t986;
    let t41348 = t3270 * t41347;
    let t41350 = t3269 * t41348 / F::new(2.0);
    let t41352 = F::new(0.28565981518604370584e-1) * t39355;
    (t41339, t41342, t41346, t41350, t41352)
}
