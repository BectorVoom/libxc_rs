//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 336/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk336(t310: f64, t547: f64, t315: f64, t545: f64, t323: f64, t145: f64, t495: f64, t301: f64) -> (f64, f64, f64, f64, f64) {
    let t1306 = t310 * t547;
    let t1308 = t315 * t545;
    let t1309 = t1308 * t323;
    let t1313 = t145 * t495;
    let t1314 = t1313 * t301;
    (t1306, t1308, t1309, t1313, t1314)
}
