//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 153/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk153(t373: f64, t376: f64, t379: f64, t383: f64) -> (f64, f64, f64) {
    let t398 = 0.705945e1_f64 * t376 + 0.1549425e1_f64 * t373 + 0.420775e0_f64 * t379 + 0.1562925e0_f64 * t383;
    let t401 = 1.0_f64 + 0.32164683177870697974e2_f64 / t398;
    let t402 = f64::ln(t401);
    (t398, t401, t402)
}
