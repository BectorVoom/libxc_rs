//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1141/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1141(t20358: f64, t2365: f64, t7025: f64, t20731: f64, t544: f64, t9287: f64, t2371: f64, t4398: f64, t7030: f64, t20670: f64, t20671: f64, t20696: f64) -> (f64, f64, f64, f64, f64) {
    let t30650 = 0.29792074959875355558e-1_f64 * t7025 * t2365 * t20358;
    let t30703 = t544 * t20731;
    let t30705 = 0.59584149919750711116e-1_f64 * t30703 * t9287;
    let t30708 = 0.59584149919750711116e-1_f64 * t4398 * t2371 * t7030;
    let t30712 = 0.17041300423964777634e0_f64 * t20670 * t20671 * t20696;
    (t30650, t30703, t30705, t30708, t30712)
}
