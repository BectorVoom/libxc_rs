//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 407/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk407<F: Float>(t2491: F, t62: F, t2490: F, t752: F, t128: F, t88: F, t109: F, t15: F, t113: F, t143: F, t130: F, t647: F) -> (F, F, F, F, F, F, F) {
    let t2492 = t62 * t2491;
    let t2493 = t2490 * t2492;
    let t2494 = t752 * t2493;
    let t2496 = t88 * t128;
    let t2500 = t109 * t15;
    let t2507 = t143 * t113;
    let t2508 = t647 * t130;
    (t2492, t2493, t2494, t2496, t2500, t2507, t2508)
}
