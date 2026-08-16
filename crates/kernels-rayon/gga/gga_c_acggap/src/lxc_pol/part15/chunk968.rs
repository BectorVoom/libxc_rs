//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 968/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk968(t1466: f64, t30644: f64, t137: f64, t14423: f64, t1181: f64, t30209: f64, t5099: f64, t604: f64, t4347: f64, t31878: f64, t4925: f64, t1541: f64, t31631: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34239 = t30644 * t1466;
    let t34248 = t14423 * t137;
    let t34255 = t30209 * t1181 * t604 * t5099;
    let t34263 = t30209 * t1181 * t604 * t4347;
    let t34271 = t31878 * t4925;
    let t34273 = t31631 * t1541;
    (t34239, t34248, t34255, t34263, t34271, t34273)
}
