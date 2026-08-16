//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 834/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk834(t8444: f64, t9022: f64, t9057: f64, t9079: f64, t105: f64, t469: f64, t301: f64, t560: f64, t2541: f64, t566: f64, t95: f64, t3952: f64, t624: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9081 = t8444 + t9022 + t9057 + t9079;
    let t9082 = t105 * t9081;
    let t9083 = t9082 * t469;
    let t9089 = t560 * t301;
    let t9090 = t2541 * t9089;
    let t9096 = t566 * t95 * t105;
    let t9097 = t624 * t3952;
    (t9081, t9082, t9083, t9089, t9090, t9096, t9097)
}
