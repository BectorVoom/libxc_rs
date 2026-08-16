//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 682/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk682(t20049: f64, t35: f64, t20022: f64, t8120: f64, t420: f64, t419: f64, t8101: f64, t20031: f64, t3088: f64, t8088: f64, t1527: f64, t20039: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20050 = t20049 * t35;
    let t20065 = t8120 * t20022;
    let t20066 = t420 * t20065;
    let t20067 = t419 * t20066;
    let t20069 = t8101 * t20022;
    let t20070 = t420 * t20069;
    let t20071 = t419 * t20070;
    let t20073 = t3088 * t20031;
    let t20074 = t419 * t20073;
    let t20076 = t8088 * t20022;
    let t20077 = t420 * t20076;
    let t20078 = t419 * t20077;
    let t20080 = t1527 * t20039;
    (t20050, t20065, t20067, t20069, t20071, t20074, t20076, t20078, t20080)
}
