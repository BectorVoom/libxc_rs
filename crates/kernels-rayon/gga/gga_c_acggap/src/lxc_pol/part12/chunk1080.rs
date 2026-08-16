//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1080/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1080(t30154: f64, t35225: f64, t7586: f64, t1535: f64, t4180: f64, t7646: f64, t4393: f64, t8511: f64, t4414: f64, t7822: f64, t1181: f64, t30327: f64, t4358: f64, t599: f64) -> (f64, f64, f64, f64, f64) {
    let t35227 = t30154 * t7586 * t35225;
    let t35230 = t4180 * t7646 * t1535;
    let t35232 = t8511 * t4393;
    let t35234 = t7822 * t4414;
    let t35238 = t30327 * t1181 * t599 * t4358;
    (t35227, t35230, t35232, t35234, t35238)
}
