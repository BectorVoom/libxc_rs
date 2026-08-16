//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1227/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1227(t118: f64, t128069: f64, t128193: f64, t28166: f64, t8697: f64, t28168: f64, t13648: f64, t2014: f64, t8714: f64, t127545: f64, t127547: f64, t127549: f64, t127550: f64, t127556: f64, t127559: f64, t2089: f64, t28030: f64, t28160: f64, t32322: f64, t32389: f64, t4297: f64, t7378: f64, t7474: f64, t7725: f64, t8111: f64) -> f64 {
    let t128195 = t118 * (t128069 + t128193);
    let t128196 = t8697 * t28166;
    let t128198 = 6.0_f64 * t128196 * t28168;
    let t128200 = t2014 * t8714 * t13648;
    let t128201 = -t2089 * t28160 - 2.0_f64 * t28030 * t7378 - t32322 * t8111 - 2.0_f64 * t32389 * t4297 - t7474 * t7725 - t127545 - t127547 - t127549 - t127550 - t127556 + t127559 - t128195 + t128198 - t128200;
    t128201
}
