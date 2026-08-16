//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 893/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk893(t8326: f64, t971: f64, t8216: f64, t1786: f64, t3238: f64, t3281: f64, t981: f64, t7943: f64, t89: f64, t973: f64, t955: f64, t951: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47222 = t8326 * t971;
    let t47273 = t8216 * t971;
    let t47443 = t1786 * t3238;
    let t47727 = t3281 * t981;
    let t47836 = t89 * t7943 * t973;
    let t47860 = t3281 * t955;
    let t47926 = t3281 * t951;
    (t47222, t47273, t47443, t47727, t47836, t47860, t47926)
}
