//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 984/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk984(t1040: f64, t11388: f64, t1026: f64, t424: f64, t1046: f64, t8: f64, t8652: f64, t667: f64) -> (f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t11389 = t11388 * t1040;
    let t11391 = t424 * t1026;
    let t11392 = t11391 * t1046;
    let t11395 = 1.0_f64 / t8 / t8652;
    let t11397 = t667 * t11395 * pi;
    (t11389, t11391, t11392, t11395, t11397)
}
