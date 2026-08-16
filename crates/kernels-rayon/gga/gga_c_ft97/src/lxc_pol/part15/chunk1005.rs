//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1005/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1005(t419: f64, t420: f64, t423: f64, t85501: f64, t45662: f64, t58730: f64, t85463: f64, t85467: f64, t85472: f64, t85476: f64, t85481: f64, t85485: f64, t85489: f64, t85493: f64, t85498: f64) -> (f64, f64) {
    let t85504 = t419 * t420 * t423 * t85501;
    let t85506 = 0.25537443351851851852e-1_f64 * t85463 + 0.34049924469135802469e-1_f64 * t85467 - 0.15322466011111111111e0_f64 * t85472 + 0.22983699016666666666e0_f64 * t85476 + 0.17024962234567901234e-1_f64 * t58730 - 0.23834947128395061728e0_f64 * t85481 + 0.11917473564197530864e0_f64 * t85485 + 0.30644932022222222222e0_f64 * t85489 - 0.30644932022222222222e0_f64 * t85493 + 0.94583123525377229081e-2_f64 * t45662 + 0.66208186467764060357e-1_f64 * t85498 + 0.6384360837962962963e-2_f64 * t85504;
    (t85504, t85506)
}
