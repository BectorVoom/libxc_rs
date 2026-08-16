//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1481/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1481(t41341: f64, t41344: f64, t41347: f64, t41350: f64, t41353: f64, t41356: f64, t41359: f64, t41361: f64, t41363: f64, t41365: f64, t41367: f64, t41369: f64) -> f64 {
    let t42031 = -0.24691358024691358025e-1_f64 * t41341 - 0.83333333333333333333e-2_f64 * t41344 - 0.19999999999999999999e0_f64 * t41347 + 0.11111111111111111111e0_f64 * t41350 - 0.55555555555555555555e-1_f64 * t41353 + 0.66666666666666666668e-1_f64 * t41356 - 0.22222222222222222222e-1_f64 * t41359 + 0.34567901234567901235e-1_f64 * t41361 + 0.44444444444444444445e-1_f64 * t41363 - 0.66666666666666666668e-1_f64 * t41365 + 0.22222222222222222222e-1_f64 * t41367 - 0.44444444444444444444e-1_f64 * t41369;
    t42031
}
