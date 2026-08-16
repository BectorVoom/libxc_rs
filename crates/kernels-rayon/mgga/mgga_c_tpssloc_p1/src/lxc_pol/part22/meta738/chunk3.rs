//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2425/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2425(t49378: f64, t59657: f64, t60163: f64, t60168: f64, t60173: f64, t68536: f64, t68541: f64, t68545: f64, t68549: f64, t68552: f64, t68556: f64, t68563: f64) -> f64 {
    let t69105 = 0.20839e0_f64 * t68536 - 0.34731666666666666667e-1_f64 * t68541 + 0.250068e1_f64 * t68545 - 0.187551e1_f64 * t68549 - 0.125034e1_f64 * t68552 + 0.62517e0_f64 * t68556 + 0.20839e0_f64 * t60163 + 0.69463333333333333335e0_f64 * t60168 - 0.34731666666666666667e0_f64 * t60173 - 0.45908888888888888888e0_f64 * t59657 - 0.13892666666666666667e0_f64 * t68563 + t49378;
    t69105
}
