//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3382/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3382(t141: f64, t2908: f64, t63357: f64, t11341: f64, t63344: f64, t41294: f64, t63349: f64, t2880: f64, t63395: f64, t41441: f64, t63462: f64, t63464: f64, t63541: f64, t63543: f64, t63545: f64, t63547: f64, t63549: f64, t63551: f64, t63554: f64, t63557: f64) -> (f64, f64, f64, f64, f64) {
    let t63560 = t141 * t2908 * t63357;
    let t63563 = t141 * t11341 * t63344;
    let t63566 = t141 * t41294 * t63349;
    let t63568 = t2880 * t63395;
    let t63573 = -0.21908444444444444444e0_f64 * t63541 + 0.36514074074074074074e-1_f64 * t63543 - 0.91285185185185185185e-1_f64 * t63545 - 0.21908444444444444444e0_f64 * t63547 + 0.73028148148148148149e-1_f64 * t63549 + 0.48685432098765432099e-1_f64 * t63551 + 0.16431333333333333333e0_f64 * t63554 + 0.43816888888888888889e0_f64 * t63557 - 0.54771111111111111112e-1_f64 * t63560 - 0.36514074074074074075e-1_f64 * t63563 - 0.85199506172839506175e-1_f64 * t63566 - 0.1898925e1_f64 * t63568 + 0.486854320987654321e0_f64 * t41441 + 0.11958666666666666667e1_f64 * t63462 - 0.13287407407407407408e0_f64 * t63464;
    (t63560, t63563, t63566, t63568, t63573)
}
