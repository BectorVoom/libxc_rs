//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1246/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1246(t11257: f64, t3639: f64, t4644: f64, t1265: f64, t1459: f64, t3649: f64, t3652: f64, t11182: f64, t11185: f64, t11249: f64, t25176: f64, t11215: f64, t13676: f64, t13679: f64, t520: f64) -> (f64, f64, f64, f64, f64) {
    let t35634 = t11257 * t3639 * t4644;
    let t35638 = t3649 * t1265 * t1459 * t3652;
    let t35640 = t11182 * t11185;
    let t35643 = t25176 * t1459 * t11249;
    let t35647 = t11215 * t13676 * t520 * t13679;
    (t35634, t35638, t35640, t35643, t35647)
}
