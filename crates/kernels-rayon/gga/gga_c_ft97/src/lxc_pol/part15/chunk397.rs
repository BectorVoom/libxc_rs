//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 397/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk397(t2: f64, t942: f64, t2981: f64, t3006: f64, t376: f64, t89: f64, t973: f64, t103: f64, t1570: f64, t100: f64, t1780: f64, t1557: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3149 = t2 * t942;
    let t3161 = t2981 / 27.0_f64;
    let t3166 = t3006 / 9.0_f64;
    let t3177 = t89 * t376 * t973;
    let t3187 = t103 * t1570;
    let t3193 = t1780 * t100;
    let t3194 = t103 * t1557;
    (t3149, t3161, t3166, t3177, t3187, t3193, t3194)
}
