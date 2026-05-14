//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 616/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk616<F: Float>(t529: F, t4348: F, t4350: F, t3729: F, t41: F, t1287: F, t1558: F, t382: F, t4144: F, t4148: F, t525: F, t526: F, t79: F, t534: F, t1587: F, t538: F) -> (F, F, F, F, F, F) {
    let t530 = t529 < -0.66725e-1;
    let t4351 = t4348 * t4350;
    let t4354 = t3729 * t41;
    let t4368 = piecewise3(t530, 0.0, 10.0 / 9.0 * t525 * t4354 * t382 - 20.0 / 27.0 * t525 * t1558 * t1287 + 40.0 / 81.0 * t525 * t526 * t4144 - 10.0 / 27.0 * t525 * t526 * t4148);
    let t4369 = t79 * t4368;
    let t4370 = t4369 * t534;
    let t4374 = 1.0 / t1587 / t538;
    (t4351, t4354, t4368, t4369, t4370, t4374)
}
