//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2175/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2175(t80722: f64, t80744: f64, t81264: f64, t90605: f64, t90609: f64, t90646: f64, t93438: f64, t93445: f64, t97509: f64, t97513: f64, t97516: f64, t1992: f64, t22635: f64, t26354: f64, t5353: f64) -> (f64, f64) {
    let t97519 = -t90605 - 0.49348022005446793095e-1_f64 * t90609 + 0.63969658155208805863e-1_f64 * t80722 - t80744 + 0.82246703342411321825e-2_f64 * t97509 - 0.82246703342411321825e-2_f64 * t97513 + 0.3289868133696452873e-1_f64 * t97516 + t93438 + t90646 + 0.26044789391763585244e-1_f64 * t81264 - t93445;
    let t97524 = t1992 * t22635 * t26354 * t5353;
    (t97519, t97524)
}
