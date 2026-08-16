//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3654/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3654(t56176: f64, t56183: f64, t56185: f64, t56187: f64, t56189: f64, t56209: f64, t56212: f64, t56214: f64, t56216: f64, t56228: f64, t68363: f64, t68366: f64) -> f64 {
    let t69072 = -0.28493333333333333333e0_f64 * t68363 + 0.79148148148148148147e-1_f64 * t68366 - 0.21106172839506172839e-1_f64 * t56176 + 0.63318518518518518517e-1_f64 * t56183 - 0.47488888888888888888e-1_f64 * t56185 - 0.23744444444444444444e-1_f64 * t56187 - 0.71233333333333333332e-1_f64 * t56189 + 0.15829629629629629629e-1_f64 * t56209 + 0.79148148148148148147e-2_f64 * t56212 + 0.47488888888888888888e-1_f64 * t56214 - 0.13191358024691358025e-1_f64 * t56216 + 0.31659259259259259258e-1_f64 * t56228;
    t69072
}
