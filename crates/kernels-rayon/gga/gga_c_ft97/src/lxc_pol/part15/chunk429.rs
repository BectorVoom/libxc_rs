//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 429/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk429(t327: f64, t703: f64, t230: f64, t1270: f64, t2253: f64, t1268: f64, t2938: f64, t113: f64, t332: f64, t1528: f64, t920: f64, t72: f64, t942: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4334 = t703 * t327;
    let t4342 = t230 * t327;
    let t4350 = t2253 * t1270;
    let t4357 = t2938 * t1268;
    let t4381 = t332 * t113;
    let t4406 = t1528 * t920;
    let t4410 = t72 * t942;
    (t4334, t4342, t4350, t4357, t4381, t4406, t4410)
}
