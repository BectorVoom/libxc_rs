//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1011/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1011(t10157: f64, t446: f64, t6061: f64, t6852: f64, t140843: f64, t140857: f64, t140863: f64, t141201: f64, t150204: f64, t150208: f64, t150212: f64, t150216: f64, t150220: f64, t150224: f64, t150227: f64, t150231: f64, t150236: f64, t150241: f64, t150246: f64) -> (f64, f64) {
    let t150250 = t446 * t10157 * t6852 * t6061;
    let t150253 = -2.0_f64 / 3.0_f64 * t150204 + 2.0_f64 / 9.0_f64 * t150208 - 6.0_f64 * t150212 - 3.0_f64 * t150216 - t150220 / 2.0_f64 + 2.0_f64 * t150224 - t150227 - t150231 / 2.0_f64 + 2.0_f64 * t150236 - 3.0_f64 * t150241 - t140843 / 18.0_f64 + 24.0_f64 * t150246 - 12.0_f64 * t150250 - 2.0_f64 / 3.0_f64 * t140857 + t140863 + t141201;
    (t150250, t150253)
}
