//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3638/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3638(t43911: f64, t56176: f64, t56183: f64, t56185: f64, t68342: f64, t68347: f64, t68350: f64, t68353: f64, t68357: f64, t68360: f64, t68363: f64, t68366: f64, t68368: f64, t68370: f64, t68373: f64) -> f64 {
    let t68854 = 0.33218518518518518518e0_f64 * t68342 + 0.39862222222222222223e1_f64 * t68347 - 0.11958666666666666667e1_f64 * t68350 - 0.71752000000000000002e1_f64 * t68353 - 0.39862222222222222222e0_f64 * t68357 + 0.71752000000000000001e1_f64 * t68360 - 0.47834666666666666668e1_f64 * t68363 + 0.13287407407407407407e1_f64 * t68366 - 0.21908444444444444444e0_f64 * t68368 - 0.48685432098765432099e-1_f64 * t68370 + 0.3071625e0_f64 * t68373 - 0.30428395061728395062e-1_f64 * t43911 - 0.35433086419753086419e0_f64 * t56176 + 0.10629925925925925926e1_f64 * t56183 - 0.79724444444444444444e0_f64 * t56185;
    t68854
}
