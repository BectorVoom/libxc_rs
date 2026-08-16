//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3398/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3398(t52126: f64, t52128: f64, t63447: f64, t63451: f64, t63453: f64, t63457: f64, t63459: f64, t63519: f64, t63522: f64, t63525: f64, t63528: f64, t63531: f64, t63533: f64, t63536: f64, t63538: f64) -> f64 {
    let t63797 = -0.36793333333333333334e0_f64 * t52126 + 0.49057777777777777779e0_f64 * t52128 + 0.20128333333333333334e0_f64 * t63447 - 0.301925e0_f64 * t63451 - 0.89459259259259259257e-1_f64 * t63453 - 0.40256666666666666666e0_f64 * t63457 + 0.26837777777777777777e0_f64 * t63459 + 0.16557e0_f64 * t63519 + 0.16557e0_f64 * t63522 - 0.27595e-1_f64 * t63525 - 0.36793333333333333333e-1_f64 * t63528 - 0.82785e-1_f64 * t63531 - 0.30661111111111111112e-1_f64 * t63533 - 0.5519e-1_f64 * t63536 + 0.18396666666666666667e0_f64 * t63538;
    t63797
}
