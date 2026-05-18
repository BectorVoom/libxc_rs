//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 885/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk885<F: Float>(t552: F, t7380: F, t573: F, t41: F, t7052: F, t556: F, t571: F, t2042: F, t2046: F, t6927: F, t572: F, t4255: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7381 = t7380 * t552;
    let t7382 = t7381 * sigma2;
    let t7383 = t7382 * t573;
    let t7385 = t7052 * t41;
    let t7386 = t7385 * t556;
    let t7387 = t571 * t7386;
    let t7389 = t2042 * t2046;
    let t7390 = t571 * t7389;
    let t7392 = t556 * t6927;
    let t7393 = t572 * t7392;
    let t7394 = t4255 * t7393;
    (t7382, t7383, t7385, t7386, t7387, t7389, t7390, t7392, t7393, t7394)
}
