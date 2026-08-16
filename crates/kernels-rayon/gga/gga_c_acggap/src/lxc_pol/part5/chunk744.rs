//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 744/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk744(t1459: f64, t398: f64, t5537: f64, t1008: f64, t1851: f64, t1298: f64, t513: f64, t1089: f64, t1095: f64, t1524: f64, t495: f64, t1856: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5539 = t398 * t1459 * t5537;
    let t5542 = t1008 * t1851;
    let t5544 = t1298 * t513;
    let t5546 = t1089 * t1095 * t5544;
    let t5549 = t495 * t1524;
    let t5551 = t1089 * t1095 * t5549;
    let t5554 = t1008 * t1856;
    (t5539, t5542, t5544, t5546, t5549, t5551, t5554)
}
