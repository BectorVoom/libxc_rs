//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1013/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1013(t35537: f64, t681: f64, t89: f64, t140756: f64, t140762: f64, t27796: f64, t33294: f64, t141204: f64, t141206: f64, t141220: f64, t141223: f64, t141231: f64, t141240: f64, t141255: f64, t141282: f64, t141295: f64, t141304: f64, t150259: f64, t150263: f64, t150267: f64, t150271: f64) -> (f64, f64, f64) {
    let t150277 = t89 * t681 * t35537;
    let t150283 = t140762 * t140756 * t33294 * t27796;
    let t150285 = -t141204 + t141206 / 3.0_f64 + 2.0_f64 * t141220 - 4.0_f64 / 3.0_f64 * t141223 + t150259 / 9.0_f64 + 8.0_f64 * t150263 - 3.0_f64 / 8.0_f64 * t150267 - t150271 / 2.0_f64 + 4.0_f64 / 3.0_f64 * t141231 - 8.0_f64 / 3.0_f64 * t141240 - t141255 / 12.0_f64 - 2.0_f64 / 3.0_f64 * t150277 - t141282 + t141295 / 6.0_f64 - t141304 / 3.0_f64 - 6.0_f64 * t150283;
    (t150277, t150283, t150285)
}
