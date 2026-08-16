//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1382/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1382(t55487: f64, t55491: f64, t55500: f64, t55508: f64, t56978: f64, t56980: f64, t56982: f64, t56984: f64, t56986: f64, t56988: f64, t56990: f64, t56992: f64, t56994: f64) -> f64 {
    let t58655 = -t55487 + t56978 / 48.0_f64 + t56980 / 12.0_f64 - 7.0_f64 / 36.0_f64 * t56982 + 5.0_f64 / 192.0_f64 * t56984 - t55491 + t55500 + t56986 / 384.0_f64 - 7.0_f64 / 576.0_f64 * t56988 + t55508 - 7.0_f64 / 72.0_f64 * t56990 + 7.0_f64 / 36.0_f64 * t56992 + t56994 / 48.0_f64;
    t58655
}
