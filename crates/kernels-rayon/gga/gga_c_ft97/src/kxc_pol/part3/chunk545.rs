//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 545/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk545(t230: f64, t327: f64, t3700: f64, t18: f64, t231: f64, t893: f64, t1270: f64, t2253: f64, t1268: f64, t668: f64, t2923: f64, t505: f64) -> (f64, f64, f64, f64, f64) {
    let t4342 = t230 * t327;
    let t4343 = t4342 * t3700;
    let t4347 = t231 * t893 * t18;
    let t4350 = t2253 * t1270;
    let t4352 = t1268 * t668;
    let t4354 = t2923 * t4352 * t505;
    (t4342, t4343, t4347, t4350, t4354)
}
