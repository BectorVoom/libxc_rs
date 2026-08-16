//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1163/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1163(t139377: f64, t139380: f64, t148492: f64, t148496: f64, t148499: f64, t148502: f64, t148508: f64, t148511: f64, t148515: f64, t148520: f64, t148523: f64, t148527: f64, t148530: f64, t148533: f64, t148536: f64, t148540: f64) -> f64 {
    let t148814 = -2.0_f64 / 3.0_f64 * t148492 + 24.0_f64 * t148496 + t148499 / 6.0_f64 + t148502 + t139377 - 2.0_f64 / 3.0_f64 * t139380 + 2.0_f64 * t148508 + 2.0_f64 * t148511 + 4.0_f64 * t148515 + 3.0_f64 / 2.0_f64 * t148520 - 2.0_f64 / 3.0_f64 * t148523 - 3.0_f64 * t148527 + 2.0_f64 * t148530 - t148533 / 3.0_f64 + t148536 / 9.0_f64 + 3.0_f64 * t148540;
    t148814
}
