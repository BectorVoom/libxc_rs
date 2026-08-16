//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 720/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk720(t11351: f64, t35: f64, t3064: f64, t1711: f64, t938: f64, t371: f64, t122: f64, t409: f64, t1751: f64, t374: f64, t930: f64, t3021: f64, t401: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11352 = t11351 * t35;
    let t11353 = t3064 * t11352;
    let t11356 = t1711 * t938;
    let t11357 = t371 * t11356;
    let t11360 = t409 * t122;
    let t11361 = t371 * t11360;
    let t11368 = t374 * t930 * t1751;
    let t11371 = t3021 * t401;
    (t11353, t11356, t11357, t11361, t11368, t11371)
}
