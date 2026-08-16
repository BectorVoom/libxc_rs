//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2280/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2280(t11925: f64, t14972: f64, t24563: f64, t24589: f64, t24604: f64, t24884: f64, t27382: f64, t27406: f64, t27433: f64, t27751: f64, t3471: f64, t7283: f64, t7356: f64, t8002: f64, t8061: f64, t85701: f64, t85728: f64, t86415: f64, t94436: f64, t94439: f64, t94446: f64, t94451: f64, t94456: f64, t94458: f64) -> f64 {
    let t94464 = 0.36554090374405031922e-2_f64 * t85701 - 0.82246703342411321825e-2_f64 * t7283 * t27751 * t24563 + 4.0_f64 * t14972 * t7356 - 0.18277045187202515961e-2_f64 * t94436 - t94439 + 0.82246703342411321825e-2_f64 * t7283 * t3471 * t27382 + 0.73108180748810063843e-2_f64 * t27406 * t24884 - t94446 + 0.54831135561607547884e-2_f64 * t24589 * t86415 * t27433 + t94451 + 0.27415567780803773942e-2_f64 * t24589 * t85728 * t8002 - t94456 + 0.54831135561607547884e-2_f64 * t24589 * t94458 * t24604 + 2.0_f64 * t11925 * t8061;
    t94464
}
