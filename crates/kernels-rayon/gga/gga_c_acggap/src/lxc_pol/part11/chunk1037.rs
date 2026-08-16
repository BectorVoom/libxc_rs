//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1037/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1037(t1181: f64, t30209: f64, t4347: f64, t604: f64, t4402: f64, t7822: f64, t5094: f64, t7564: f64, t8600: f64, t31878: f64, t4925: f64, t1541: f64, t31631: f64) -> (f64, f64, f64, f64, f64) {
    let t34263 = t30209 * t1181 * t604 * t4347;
    let t34265 = t7822 * t4402;
    let t34269 = t7564 * t1181 * t8600 * t5094;
    let t34271 = t31878 * t4925;
    let t34273 = t31631 * t1541;
    (t34263, t34265, t34269, t34271, t34273)
}
