//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 438/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk438<F: Float>(t3558: F, t3559: F, t457: F, t306: F, t416: F, t1175: F, t425: F, t1364: F, t298: F, t301: F, t446: F) -> (F, F, F, F, F, F, F) {
    let t3560 = t3558 * t3559;
    let t3561 = t457 * t3560;
    let t3564 = t416 * t306;
    let t3565 = t425 * t1175;
    let t3566 = t3565 * t1364;
    let t3567 = t3564 * t3566;
    let t3571 = t298 * t446 * t301;
    (t3560, t3561, t3564, t3565, t3566, t3567, t3571)
}
