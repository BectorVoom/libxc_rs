//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1019/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1019(t11697: f64, t11700: f64, t11728: f64, t11730: f64, t12477: f64, t12480: f64, t12482: f64, t12487: f64, t12490: f64, t12493: f64, t12496: f64, t12499: f64, t12501: f64, t12504: f64, t12507: f64) -> f64 {
    let t12782 = 0.95219938395347901946e-2_f64 * t11697 + 0.28565981518604370584e-1_f64 * t11700 + 0.17336443480108537126e0_f64 * t12477 + 0.5200933044032561138e0_f64 * t12480 + 0.21951497276451705328e0_f64 * t12482 + 0.13869154784086829701e1_f64 * t11728 + 0.51220160311720645767e0_f64 * t11730 + 0.17336443480108537126e0_f64 * t12487 + 0.10401866088065122276e1_f64 * t12490 - 0.87327386630866483588e-2_f64 * t12493 - 0.26198215989259945076e-1_f64 * t12496 - 0.86682217400542685632e-1_f64 * t12499 - 0.5200933044032561138e0_f64 * t12501 - 0.2600466522016280569e0_f64 * t12504 + 0.10975748638225852664e0_f64 * t12507;
    t12782
}
