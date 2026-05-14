//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 805/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk805<F: Float>(t13382: F, t1441: F, t1411: F, t3739: F, t3788: F, t3792: F, t3512: F, t3778: F, t1286: F, t3502: F, sigma0: F) -> (F, F, F, F, F) {
    let t13383 = t13382 * sigma0;
    let t13384 = t13383 * t1441;
    let t13385 = t1411 * t13384;
    let t13387 = t3739 * t3788;
    let t13389 = t3739 * t3792;
    let t13391 = t3512 * t3778;
    let t13392 = t1411 * t13391;
    let t13394 = t3502 * t1286;
    (t13385, t13387, t13389, t13392, t13394)
}
