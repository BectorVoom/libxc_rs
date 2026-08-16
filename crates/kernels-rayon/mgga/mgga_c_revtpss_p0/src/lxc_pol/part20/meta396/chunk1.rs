//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1458/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1458(t11387: f64, t41500: f64, t41588: f64, t41245: f64, t41250: f64, t41255: f64, t41260: f64, t41265: f64, t41267: f64, t41273: f64, t41275: f64, t41279: f64, t41281: f64, t41283: f64, t41285: f64, t41287: f64, t41289: f64) -> (f64, f64) {
    let t41591 = 0.62071215503128080361e4_f64 * t41588 * t41500 * t11387;
    let t41592 = 0.13388493827160493828e1_f64 * t41245;
    let t41606 = t41592 - 0.21908444444444444444e0_f64 * t41250 + 0.65725333333333333332e0_f64 * t41255 - 0.10954222222222222222e0_f64 * t41260 + 0.98587999999999999999e0_f64 * t41265 - 0.13145066666666666666e1_f64 * t41267 + 0.43816888888888888889e0_f64 * t41273 + 0.13145066666666666666e1_f64 * t41275 - 0.98587999999999999998e0_f64 * t41279 + 0.10954222222222222222e1_f64 * t41281 - 0.43816888888888888888e0_f64 * t41283 - 0.54771111111111111111e0_f64 * t41285 - 0.18257037037037037037e0_f64 * t41287 + 0.21908444444444444444e0_f64 * t41289;
    (t41591, t41606)
}
