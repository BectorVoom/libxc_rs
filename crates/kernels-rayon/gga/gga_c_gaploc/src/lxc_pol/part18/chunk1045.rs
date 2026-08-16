//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1045/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1045(t2371: f64, t4360: f64, t4803: f64, t6715: f64, t20117: f64, t6508: f64, t20013: f64, t1433: f64, t9271: f64, t1323: f64, t874: f64, t2366: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20481 = t4360 * t2371;
    let t20496 = t4803 * t6715;
    let t20513 = t6508 * t20117;
    let t20521 = t6508 * t20013;
    let t20535 = t1433 * t9271;
    let t20539 = t874 * t1323;
    let t20540 = t2366 * t20539;
    (t20481, t20496, t20513, t20521, t20535, t20539, t20540)
}
