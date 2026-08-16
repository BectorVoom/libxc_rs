//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 861/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk861(t3821: f64, t668: f64, t505: f64, t2493: f64, t2380: f64, t2393: f64, t200: f64, t1609: f64, t213: f64, t1109: f64, t2378: f64, t2417: f64, t679: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13389 = t3821 * t668;
    let t13390 = t13389 * t505;
    let t13391 = t2493 * t13390;
    let t13394 = t2393 * t2380;
    let t13395 = t13394 * t200;
    let t13399 = t1609 * t213;
    let t13400 = t13399 * t1109;
    let t13401 = t2378 * t2380;
    let t13402 = t13401 * t200;
    let t13406 = t679 * t2417;
    (t13390, t13391, t13395, t13400, t13402, t13406)
}
