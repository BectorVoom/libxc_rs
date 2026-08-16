//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3459/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3459(t41330: f64, t41332: f64, t52047: f64, t52049: f64, t52051: f64, t63399: f64, t63447: f64, t63451: f64, t63453: f64, t63457: f64, t63459: f64, t63462: f64, t63464: f64) -> f64 {
    let t65054 = 0.74074074074074074074e-2_f64 * t52047 + 0.37037037037037037037e-2_f64 * t52049 + 0.61728395061728395062e-2_f64 * t52051 - 0.2e0_f64 * t63399 - 0.37037037037037037037e-2_f64 * t41330 - 0.24691358024691358025e-2_f64 * t41332 + 0.55555555555555555556e-2_f64 * t63447 - 0.83333333333333333333e-2_f64 * t63451 - 0.24691358024691358024e-2_f64 * t63453 - 0.11111111111111111111e-1_f64 * t63457 + 0.74074074074074074076e-2_f64 * t63459 + 0.33333333333333333334e-1_f64 * t63462 - 0.37037037037037037037e-2_f64 * t63464;
    t65054
}
