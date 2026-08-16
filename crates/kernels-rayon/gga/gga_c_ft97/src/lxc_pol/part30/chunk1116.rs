//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1116/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1116(t193: f64, t33983: f64, t4129: f64, t89: f64, t28671: f64, t33414: f64, t28599: f64, t33404: f64, t28562: f64, t28567: f64, t150436: f64, t28558: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t152979 = t89 * t193 * t33983 * t4129;
    let t152981 = t33414 * t28671;
    let t152984 = t33404 * t28599;
    let t152987 = t33404 * t28562;
    let t152996 = t33404 * t28567;
    let t153007 = t28558 * t150436;
    (t152979, t152981, t152984, t152987, t152996, t153007)
}
