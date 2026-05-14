//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 922/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk922<F: Float>(t11422: F, t11425: F, t11428: F, t11432: F, t11433: F, t11444: F, t11817: F, t11843: F, t11845: F, t12534: F, t12536: F, t12539: F, t12541: F, t12544: F, t12548: F, t12552: F) -> (F,) {
    let t12809 = -0.87327386630866483588e-2 * t12534 + 0.87327386630866483588e-2 * t12536 + 0.43663693315433241794e-2 * t12539 + 0.86682217400542685632e-1 * t12541 - 0.26198215989259945076e-1 * t12544 + t11422 + t11425 + 0.95219938395347901946e-2 * t11817 - t11428 + t11432 + t11433 + 0.46230515946956099004e0 * t11843 + 0.25610080155860322884e0 * t11845 + 0.5200933044032561138e0 * t12548 - t11444 - 0.43663693315433241794e-2 * t12552;
    (t12809,)
}
