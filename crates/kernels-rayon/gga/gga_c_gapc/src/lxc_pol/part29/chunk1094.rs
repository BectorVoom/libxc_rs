//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1094/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1094(t11808: f64, t16181: f64, t9863: f64, t667: f64, t8709: f64, t17891: f64, t29070: f64, t1736: f64, t188: f64, t1180: f64, t11970: f64, t1084: f64, t327: f64) -> (f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t33536 = t11808 * t9863 * t16181;
    let t33539 = t667 * t8709 * pi;
    let t33541 = t17891 * t33539 * t29070;
    let t33543 = t188 * t1736;
    let t33546 = t11970 * t1180;
    let t33547 = t1084 * t33543 * t327 * t33546;
    (t33536, t33539, t33541, t33543, t33546, t33547)
}
