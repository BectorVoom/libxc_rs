//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 688/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk688(t20022: f64, t7764: f64, t7761: f64, t89: f64, t4454: f64, t942: f64, t7793: f64, t446: f64, t7801: f64, t1555: f64, t1866: f64, t20031: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20124 = t7764 * t20022;
    let t20126 = t89 * t7761 * t20124;
    let t20130 = t4454 * t942;
    let t20131 = t7793 * t20130;
    let t20132 = t446 * t20131;
    let t20134 = t7801 * t20022;
    let t20136 = t89 * t1555 * t20134;
    let t20138 = t1866 * t20031;
    (t20124, t20126, t20130, t20131, t20132, t20134, t20136, t20138)
}
