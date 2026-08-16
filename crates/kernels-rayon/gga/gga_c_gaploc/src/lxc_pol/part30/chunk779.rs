//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 779/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk779(t7371: f64, t7372: f64, t5654: f64, t824: f64, t2618: f64, t1902: f64, t2465: f64, t2464: f64, t2615: f64, t161: f64, t165: f64, t1710: f64) -> (f64, f64, f64, f64, f64) {
    let t7373 = t7371 * t7372;
    let t7375 = t5654 * t824;
    let t7376 = t7375 * t2618;
    let t7378 = t2465 * t1902;
    let t7379 = t2464 * t7378;
    let t7380 = t2615 * t7379;
    let t7383 = t161 * t165 * t1710;
    (t7373, t7375, t7376, t7380, t7383)
}
