//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 869/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk869(t1994: f64, t30179: f64, t1039: f64, t1997: f64, t3055: f64, t1967: f64, t7784: f64, t1200: f64, t7614: f64, t30169: f64, t601: f64, t3646: f64, t597: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30180 = t30179 * t1994;
    let t30183 = t3055 * t1997 * t1039;
    let t30185 = t1967 * t7784;
    let t30187 = t7614 * t1200;
    let t30191 = t30169 * t601;
    let t30193 = t3646 * t597;
    (t30180, t30183, t30185, t30187, t30191, t30193)
}
