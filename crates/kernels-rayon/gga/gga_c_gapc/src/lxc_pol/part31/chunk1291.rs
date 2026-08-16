//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1291/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1291(t1030: f64, t27597: f64, t34026: f64, t21825: f64, t3680: f64, t1026: f64, t1845: f64, t3018: f64, t11391: f64, t3022: f64, t1803: f64, t8738: f64) -> (f64, f64, f64, f64, f64) {
    let t35328 = t1030 * t34026 * t27597;
    let t35330 = t21825 * t3680;
    let t35334 = t1845 * t1026 * t3018;
    let t35336 = t11391 * t3022;
    let t35339 = t1803 * t1026 * t8738;
    (t35328, t35330, t35334, t35336, t35339)
}
