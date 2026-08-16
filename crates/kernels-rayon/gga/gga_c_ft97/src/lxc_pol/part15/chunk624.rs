//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 624/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk624(t1270: f64, t8640: f64, t2440: f64, t327: f64, t10845: f64, t2347: f64, t2360: f64, t2923: f64, t10864: f64, t1268: f64, t1186: f64, t89: f64, t9733: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14445 = t8640 * t1270;
    let t14487 = t2440 * t327;
    let t14514 = t10845 * t2347;
    let t14519 = t2923 * t2360;
    let t14523 = t10864 * t1268;
    let t14715 = t89 * t9733 * t1186;
    (t14445, t14487, t14514, t14519, t14523, t14715)
}
