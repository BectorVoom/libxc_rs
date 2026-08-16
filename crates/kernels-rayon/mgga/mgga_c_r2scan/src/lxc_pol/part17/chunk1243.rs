//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1243/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1243(t40220: f64, t41743: f64, t41748: f64, t41749: f64, t43631: f64, t43635: f64, t43638: f64, t43641: f64, t43643: f64, t43645: f64, t43648: f64, t43650: f64) -> f64 {
    let t44483 = 0.21951497276451705328e0_f64 * t43631 - t41743 - 0.43902994552903410656e0_f64 * t43635 - 0.5200933044032561138e0_f64 * t43638 - 0.20803732176130244552e1_f64 * t43641 - 0.95219938395347901947e-2_f64 * t43643 - 0.28565981518604370584e-1_f64 * t43645 + 0.26198215989259945076e-1_f64 * t43648 + 0.17336443480108537126e0_f64 * t43650 - t41748 - t41749 + 0.90044238659382329742e0_f64 * t40220;
    t44483
}
