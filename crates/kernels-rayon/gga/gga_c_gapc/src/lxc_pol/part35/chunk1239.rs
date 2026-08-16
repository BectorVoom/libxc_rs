//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1239/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1239(t1026: f64, t1845: f64, t3018: f64, t11391: f64, t3022: f64, t1803: f64, t8738: f64, t11594: f64, t21838: f64, t21631: f64, t11397: f64, t11402: f64, t424: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35334 = t1845 * t1026 * t3018;
    let t35336 = t11391 * t3022;
    let t35339 = t1803 * t1026 * t8738;
    let t35341 = t11594 * t21838;
    let t35343 = t11594 * t21631;
    let t35346 = t424 * t11397 * t11402;
    (t35334, t35336, t35339, t35341, t35343, t35346)
}
