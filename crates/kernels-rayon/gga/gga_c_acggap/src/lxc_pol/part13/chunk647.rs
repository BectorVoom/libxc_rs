//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 647/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk647(t1017: f64, t506: f64, t3300: f64, t398: f64, t513: f64, t864: f64, t1095: f64, t1036: f64, t1032: f64, t1434: f64, t922: f64, t1426: f64, t368: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5094 = t506 * t1017;
    let t5096 = t398 * t3300 * t5094;
    let t5099 = t513 * t864;
    let t5101 = t398 * t1095 * t5099;
    let t5102 = t1036 * t5101;
    let t5104 = t1032 * t1434;
    let t5106 = t506 * t922;
    let t5108 = t1426 * t368 * t5106;
    (t5094, t5096, t5099, t5101, t5102, t5104, t5106, t5108)
}
