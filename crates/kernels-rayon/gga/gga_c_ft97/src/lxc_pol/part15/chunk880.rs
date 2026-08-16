//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 880/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk880(t2: f64, t41446: f64, t241: f64, t41751: f64, t41536: f64, t2344: f64, t2371: f64, t665: f64, t7514: f64, t675: f64, t9567: f64, t11176: f64, t249: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42087 = t2 * t41446;
    let t42094 = t41751 * t241;
    let t42095 = t2 * t41536;
    let t42109 = t2344 * t2371;
    let t42110 = t42109 * t2;
    let t42123 = t665 * t7514;
    let t42124 = t42123 * t2;
    let t42163 = t9567 * t675;
    let t42164 = t42163 * t2;
    let t42206 = 280.0_f64 / 81.0_f64 * t11176 * t249;
    (t42087, t42094, t42095, t42109, t42110, t42123, t42124, t42163, t42164, t42206)
}
