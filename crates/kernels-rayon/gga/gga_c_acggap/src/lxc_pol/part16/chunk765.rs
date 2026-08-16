//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 765/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk765(t1988: f64, t2290: f64, t2268: f64, t7433: f64, t2264: f64, t7839: f64, t1511: f64, t570: f64, t1526: f64, t1298: f64, t579: f64, t336: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8578 = t1988 * t2290;
    let t8580 = t7433 * t2268;
    let t8582 = t7839 * t2264;
    let t8584 = t570 * t1511;
    let t8586 = t570 * t1526;
    let t8588 = t579 * t1298;
    let t8589 = t336 * t8588;
    (t8578, t8580, t8582, t8584, t8586, t8589)
}
