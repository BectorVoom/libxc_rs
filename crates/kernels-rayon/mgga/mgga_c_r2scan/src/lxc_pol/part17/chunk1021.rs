//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1021/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1021(t11422: f64, t11425: f64, t11428: f64, t11432: f64, t11433: f64, t11444: f64, t11817: f64, t11843: f64, t11845: f64, t12534: f64, t12536: f64, t12539: f64, t12541: f64, t12544: f64, t12548: f64, t12552: f64) -> f64 {
    let t12809 = -0.87327386630866483588e-2_f64 * t12534 + 0.87327386630866483588e-2_f64 * t12536 + 0.43663693315433241794e-2_f64 * t12539 + 0.86682217400542685632e-1_f64 * t12541 - 0.26198215989259945076e-1_f64 * t12544 + t11422 + t11425 + 0.95219938395347901946e-2_f64 * t11817 - t11428 + t11432 + t11433 + 0.46230515946956099004e0_f64 * t11843 + 0.25610080155860322884e0_f64 * t11845 + 0.5200933044032561138e0_f64 * t12548 - t11444 - 0.43663693315433241794e-2_f64 * t12552;
    t12809
}
