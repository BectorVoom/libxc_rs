//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1067/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1067<F: Float>(t3493: F, t983: F, t11002: F, t3269: F, t3262: F, t3465: F, t39183: F, t11621: F, t3275: F, t39040: F, t3719: F, t481: F, t3270: F, t10667: F, t11336: F, t37327: F, t40566: F) -> (F, F, F, F, F) {
    let t41326 = t3493 * t983;
    let t41327 = t11002 * t41326;
    let t41329 = 5.0 / 8.0 * t3269 * t41327;
    let t41332 = 3.0 / 2.0 * t3262 * t3465 * t39183;
    let t41335 = 45.0 / 32.0 * t3275 * t39040 * t11621;
    let t41336 = t3719 * t481;
    let t41337 = t3270 * t41336;
    let t41339 = 3.0 / 2.0 * t10667 * t41337;
    let t41342 = 15.0 / 8.0 * t37327 * t11336 * t40566;
    (t41329, t41332, t41335, t41339, t41342)
}
