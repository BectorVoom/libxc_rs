//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1176/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1176(t1298: f64, t2297: f64, t4256: f64, t7450: f64, t17752: f64, t2030: f64, t9711: f64, t4262: f64, t513: f64, t8539: f64, t1524: f64, t7447: f64, t9712: f64) -> (f64, f64, f64, f64, f64) {
    let t40358 = t7450 * t4256 * t2297 * t1298;
    let t40361 = t2030 * t17752 * t9711;
    let t40365 = t2030 * t4262 * t8539 * t513;
    let t40369 = t2030 * t4262 * t2297 * t1524;
    let t40371 = t7447 * t9712;
    (t40358, t40361, t40365, t40369, t40371)
}
