//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3380/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3380(t2439: f64, t6135: f64, t52126: f64, t52128: f64, t63447: f64, t63451: f64, t63453: f64, t63457: f64, t63459: f64, t63519: f64, t63522: f64, t63525: f64, t63528: f64, t63531: f64, t63533: f64, t63536: f64) -> (f64, f64) {
    let t63538 = t2439 * t6135;
    let t63540 = -0.36514074074074074074e0_f64 * t52126 + 0.48685432098765432099e0_f64 * t52128 + 0.19931111111111111111e0_f64 * t63447 - 0.29896666666666666667e0_f64 * t63451 - 0.88582716049382716049e-1_f64 * t63453 - 0.39862222222222222222e0_f64 * t63457 + 0.26574814814814814815e0_f64 * t63459 + 0.16431333333333333333e0_f64 * t63519 + 0.16431333333333333333e0_f64 * t63522 - 0.27385555555555555556e-1_f64 * t63525 - 0.36514074074074074075e-1_f64 * t63528 - 0.82156666666666666667e-1_f64 * t63531 - 0.30428395061728395062e-1_f64 * t63533 - 0.54771111111111111112e-1_f64 * t63536 + 0.18257037037037037037e0_f64 * t63538;
    (t63538, t63540)
}
