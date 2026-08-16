//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1120/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1120(t35915: f64, t35916: f64, t816: f64, t2725: f64, t6793: f64, t4092: f64, t150603: f64, t7205: f64, t811: f64, t10364: f64, t1200: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t153071 = t35915 * t35916 * t816;
    let t153074 = t2725 * t6793;
    let t153075 = t4092 * t153074;
    let t153077 = t7205 * t150603 * t811;
    let t153080 = t10364 * t6793;
    let t153081 = t1200 * t153080;
    let t153083 = t7205 * t150603 * t820;
    (t153071, t153074, t153075, t153077, t153080, t153081, t153083)
}
