//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 848/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk848(t184: f64, t363: f64, t3663: f64, t1078: f64, t2299: f64, t3664: f64, t2300: f64, t920: f64, t1079: f64, t1580: f64, t3596: f64, t5: f64) -> (f64, f64, f64, f64, f64) {
    let t13255 = t184 * t363;
    let t13256 = t3663 * t13255;
    let t13259 = t1078 * t2299;
    let t13260 = t13259 * t3664;
    let t13263 = t2300 * t920;
    let t13268 = t1079 * t1580;
    let t13273 = t5 * t3596;
    (t13256, t13260, t13263, t13268, t13273)
}
