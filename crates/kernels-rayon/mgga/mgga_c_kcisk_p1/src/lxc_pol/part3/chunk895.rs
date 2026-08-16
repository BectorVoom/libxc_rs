//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 895/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk895(t1163: f64, t3742: f64, t13377: f64, t3482: f64, t1413: f64, t3906: f64, t1441: f64, t1411: f64, t3739: f64, t3788: f64, t3792: f64, t3512: f64, t3778: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64) {
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
