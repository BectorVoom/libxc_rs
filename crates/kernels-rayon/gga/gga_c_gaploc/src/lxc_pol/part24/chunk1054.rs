//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1054/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1054(t20369: f64, t4130: f64, t20539: f64, t493: f64, t4803: f64, t6575: f64, t4786: f64, t6582: f64, t1406: f64, t6715: f64, t1339: f64, t20117: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21154 = t4130 * t20369;
    let t21172 = t493 * t20539;
    let t21272 = t4803 * t6575;
    let t21283 = t4786 * t6582;
    let t21370 = t1406 * t6715;
    let t21389 = t1339 * t20117;
    (t21154, t21172, t21272, t21283, t21370, t21389)
}
