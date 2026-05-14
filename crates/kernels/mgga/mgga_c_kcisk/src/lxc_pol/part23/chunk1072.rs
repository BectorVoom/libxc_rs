//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1072/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1072<F: Float>(t13894: F, t21543: F, t14916: F, t14918: F, t14922: F, t1580: F, t21500: F, t21519: F, t21527: F, t21532: F, t21537: F, t21540: F, t4381: F, t6482: F, t6486: F, t2318: F, t4420: F) -> (F, F) {
    let t21544 = t13894 * t21543;
    let t21549 = -t21519 + 0.47975436576472845902e-1 * t4381 * t6482 + 0.95950873152945691803e-1 * t4381 * t6486 + 0.35981577432354634426e-1 * t21500 * t21527 + 0.35981577432354634426e-1 * t21500 * t21532 - 0.59969295720591057378e-2 * t14916 + 0.79959060960788076505e-2 * t21537 + 0.11993859144118211476e-1 * t1580 * t21540 + 0.27985671336275826777e-1 * t1580 * t21544 + 0.59969295720591057378e-2 * t14918 + 0.29984647860295528689e-2 * t14922;
    let t21555 = 0.17990788716177317213e-1 * t2318 * t4420;
    (t21549, t21555)
}
