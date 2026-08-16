//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2629/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2629(t48333: f64, t5571: f64, t9419: f64, t40076: f64, t40079: f64, t47131: f64, t47138: f64, t47140: f64, t47142: f64, t47152: f64, t48322: f64, t48323: f64, t48325: f64, t48327: f64, t48328: f64, t48329: f64, t48330: f64, t48332: f64) -> (f64, f64, f64) {
    let t48334 = 36.0_f64 * t48333;
    let t48335 = t5571 * t9419;
    let t48336 = 0.10389515463408878255e3_f64 * t48335;
    let t48337 = t47131 + t48322 - t48323 - t47138 - t47140 + t47142 - t48325 - t48327 + t40076 - t40079 - t48328 - t48329 + t48330 - t48332 + t47152 + t48334 + t48336;
    (t48334, t48336, t48337)
}
