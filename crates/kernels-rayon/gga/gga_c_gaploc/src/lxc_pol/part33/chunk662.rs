//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 662/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk662(t5586: f64, t822: f64, t1964: f64, t4416: f64, t1985: f64, t2012: f64, t1: f64, t5514: f64, t787: f64, t1980: f64, t2032: f64) -> (f64, f64, f64, f64, f64) {
    let t5662 = t822 * t5586;
    let t5665 = t1964 * t4416;
    let t5666 = t822 * t5665;
    let t5669 = t2012 * t1985;
    let t5672 = t5514 * t1;
    let t5673 = t787 * t5672;
    let t5676 = t1980 * t2032;
    (t5662, t5666, t5669, t5673, t5676)
}
