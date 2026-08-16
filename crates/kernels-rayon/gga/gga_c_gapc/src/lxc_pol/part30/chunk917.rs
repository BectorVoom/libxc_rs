//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 917/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk917(t101: f64, t3643: f64, t128: f64, t1458: f64, t8297: f64, t19: f64, t8286: f64, t125: f64, t147: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11202 = t3643 * t101;
    let t11203 = t1458 * t128;
    let t11204 = t11202 * t11203;
    let t11205 = t11204 * t8297;
    let t11207 = t1458 * t19;
    let t11208 = t8286 * t11207;
    let t11209 = t147 * t125;
    let t11210 = t11209 * t128;
    (t11202, t11203, t11204, t11205, t11207, t11208, t11210)
}
