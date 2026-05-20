//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3382/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3382<F: Float>(t141: F, t2908: F, t63357: F, t11341: F, t63344: F, t41294: F, t63349: F, t2880: F, t63395: F, t41441: F, t63462: F, t63464: F, t63541: F, t63543: F, t63545: F, t63547: F, t63549: F, t63551: F, t63554: F, t63557: F) -> (F, F, F, F, F) {
    let t63560 = t141 * t2908 * t63357;
    let t63563 = t141 * t11341 * t63344;
    let t63566 = t141 * t41294 * t63349;
    let t63568 = t2880 * t63395;
    let t63573 = -F::cast_from(0.21908444444444444444e0_f64) * t63541 + F::cast_from(0.36514074074074074074e-1_f64) * t63543 - F::cast_from(0.91285185185185185185e-1_f64) * t63545 - F::cast_from(0.21908444444444444444e0_f64) * t63547 + F::cast_from(0.73028148148148148149e-1_f64) * t63549 + F::cast_from(0.48685432098765432099e-1_f64) * t63551 + F::cast_from(0.16431333333333333333e0_f64) * t63554 + F::cast_from(0.43816888888888888889e0_f64) * t63557 - F::cast_from(0.54771111111111111112e-1_f64) * t63560 - F::cast_from(0.36514074074074074075e-1_f64) * t63563 - F::cast_from(0.85199506172839506175e-1_f64) * t63566 - F::new(0.1898925e1) * t63568 + F::cast_from(0.486854320987654321e0_f64) * t41441 + F::cast_from(0.11958666666666666667e1_f64) * t63462 - F::cast_from(0.13287407407407407408e0_f64) * t63464;
    (t63560, t63563, t63566, t63568, t63573)
}
