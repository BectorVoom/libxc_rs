//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2027/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2027(t102647: f64, t102663: f64, t102679: f64, t102694: f64, t102705: f64, t102715: f64, t102732: f64, t102746: f64, t544: f64, t553: f64, t6378: f64, t7211: f64, t90993: f64, t91000: f64, t91002: f64, t93618: f64, t97119: f64, t97124: f64, t97129: f64, t97135: f64, t97137: f64, t97142: f64, t97146: f64, t97148: f64, t97152: f64, t97158: f64, t97161: f64) -> (f64, f64) {
    let t102749 = t102647 + t102663 + t102679 + t102694 + t102705 + t102715 + t102732 + t102746;
    let t102765 = -0.3289868133696452873e-1_f64 * t90993 + 0.6579736267392905746e-1_f64 * t97119 + t544 * t553 * t102749 - 0.15352717957250113407e0_f64 * t97124 + t6378 * t7211 - 0.3289868133696452873e-1_f64 * t97129 + 0.19739208802178717238e0_f64 * t97135 + 0.76763589786250567037e-1_f64 * t97137 + 0.82246703342411321825e-2_f64 * t97142 - 0.25587863262083522345e0_f64 * t91000 - 0.46058153871750340221e0_f64 * t91002 - 0.13159472534785811492e0_f64 * t97146 + 0.38381794893125283518e-1_f64 * t97148 + 0.3289868133696452873e-1_f64 * t97152 + t93618 + 0.9869604401089358619e-1_f64 * t97158 - 0.49348022005446793095e-1_f64 * t97161;
    (t102749, t102765)
}
