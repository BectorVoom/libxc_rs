//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2550/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2550(t63911: f64, t71144: f64, t71400: f64, t71403: f64, t71406: f64, t71408: f64, t71411: f64, t71414: f64, t71417: f64, t71420: f64, t71423: f64, t71426: f64) -> f64 {
    let t71585 = -0.59793333333333333333e0_f64 * t71144 - 0.85199506172839506175e-1_f64 * t71400 + 0.27385555555555555555e0_f64 * t63911 + 0.1898925e1_f64 * t71403 + 0.82156666666666666667e-1_f64 * t71406 - 0.54771111111111111111e-1_f64 * t71408 + 0.10954222222222222222e0_f64 * t71411 + 0.43816888888888888889e0_f64 * t71414 - 0.49293999999999999999e0_f64 * t71417 - 0.98587999999999999998e0_f64 * t71420 + 0.147882e1_f64 * t71423 + 0.197176e1_f64 * t71426;
    t71585
}
