//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1062/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1062(t141206: f64, t141220: f64, t141223: f64, t141231: f64, t141240: f64, t141255: f64, t141282: f64, t141295: f64, t141304: f64, t141607: f64, t150259: f64, t150263: f64, t150267: f64, t150271: f64, t150277: f64, t150283: f64) -> f64 {
    let t151296 = -t141607 + t141206 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t141220 - 4.0_f64 / 9.0_f64 * t141223 + t150259 / 27.0_f64 + 8.0_f64 / 3.0_f64 * t150263 - t150267 / 8.0_f64 - t150271 / 6.0_f64 + 4.0_f64 / 9.0_f64 * t141231 - 8.0_f64 / 9.0_f64 * t141240 - t141255 / 36.0_f64 - 2.0_f64 / 9.0_f64 * t150277 - t141282 / 3.0_f64 + t141295 / 18.0_f64 - t141304 / 9.0_f64 - 2.0_f64 * t150283;
    t151296
}
