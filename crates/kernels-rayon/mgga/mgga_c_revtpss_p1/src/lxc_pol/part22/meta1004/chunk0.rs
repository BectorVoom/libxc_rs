//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3428/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3428(t41330: f64, t41332: f64, t52047: f64, t52049: f64, t52051: f64, t63399: f64, t63447: f64, t63451: f64, t63453: f64, t63457: f64, t63459: f64, t63462: f64, t63464: f64) -> f64 {
    let t64400 = 0.15829629629629629629e-1_f64 * t52047 + 0.79148148148148148147e-2_f64 * t52049 + 0.13191358024691358025e-1_f64 * t52051 - 0.4274e0_f64 * t63399 - 0.79148148148148148147e-2_f64 * t41330 - 0.52765432098765432098e-2_f64 * t41332 + 0.11872222222222222222e-1_f64 * t63447 - 0.17808333333333333333e-1_f64 * t63451 - 0.52765432098765432097e-2_f64 * t63453 - 0.23744444444444444444e-1_f64 * t63457 + 0.15829629629629629629e-1_f64 * t63459 + 0.71233333333333333332e-1_f64 * t63462 - 0.79148148148148148146e-2_f64 * t63464;
    t64400
}
