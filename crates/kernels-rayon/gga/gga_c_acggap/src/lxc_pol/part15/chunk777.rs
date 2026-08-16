//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 777/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk777(t1165: f64, t604: f64, t8791: f64, t7413: f64, t1323: f64, t7815: f64, t2030: f64, t1327: f64, t2060: f64, t2029: f64, t568: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8793 = t1165 * t604 * t8791;
    let t8794 = t7413 * t8793;
    let t8800 = t7815 * t1323;
    let t8801 = t2030 * t8800;
    let t8803 = t7815 * t1327;
    let t8804 = t2060 * t8803;
    let t8806 = t568 * t2029;
    (t8793, t8794, t8800, t8801, t8803, t8804, t8806)
}
