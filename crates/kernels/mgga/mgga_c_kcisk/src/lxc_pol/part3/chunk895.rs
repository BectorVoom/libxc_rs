//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 895/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk895<F: Float>(t1163: F, t3742: F, t13377: F, t3482: F, t1413: F, t3906: F, t1441: F, t1411: F, t3739: F, t3788: F, t3792: F, t3512: F, t3778: F, sigma0: F) -> (F, F, F, F, F, F) {
    let t13378 = t3742 * t1163;
    let t13379 = t13377 * t13378;
    let t13380 = t3482 * t13379;
    let t13382 = t3906 * t1413;
    let t13383 = t13382 * sigma0;
    let t13384 = t13383 * t1441;
    let t13385 = t1411 * t13384;
    let t13387 = t3739 * t3788;
    let t13389 = t3739 * t3792;
    let t13391 = t3512 * t3778;
    (t13380, t13382, t13385, t13387, t13389, t13391)
}
