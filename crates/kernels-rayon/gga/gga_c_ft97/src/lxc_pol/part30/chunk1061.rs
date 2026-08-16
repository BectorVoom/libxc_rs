//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1061/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1061(t140843: f64, t140857: f64, t140863: f64, t141606: f64, t150204: f64, t150208: f64, t150212: f64, t150216: f64, t150220: f64, t150224: f64, t150227: f64, t150231: f64, t150236: f64, t150241: f64, t150246: f64, t150250: f64) -> f64 {
    let t151278 = -2.0_f64 / 9.0_f64 * t150204 + 2.0_f64 / 27.0_f64 * t150208 - 2.0_f64 * t150212 - t150216 - t150220 / 6.0_f64 + 2.0_f64 / 3.0_f64 * t150224 - t150227 / 3.0_f64 - t150231 / 6.0_f64 + 2.0_f64 / 3.0_f64 * t150236 - t150241 - t140843 / 54.0_f64 + 8.0_f64 * t150246 - 4.0_f64 * t150250 - 2.0_f64 / 9.0_f64 * t140857 + t140863 / 3.0_f64 + t141606;
    t151278
}
