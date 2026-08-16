//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1073/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1073(t3645: f64, t553: f64, t3077: f64, t4150: f64, t1160: f64, t5315: f64, t930: f64, t159: f64, t322: f64, t381: f64, t5299: f64, t1004: f64, t4226: f64) -> (f64, f64, f64, f64, f64) {
    let t19090 = t3645 * t553;
    let t19095 = t3077 * t4150;
    let t19098 = t1160 * t5315 * t930;
    let t19103 = t381 * t159 * t5299 * t322;
    let t19108 = t1004 * t4226;
    (t19090, t19095, t19098, t19103, t19108)
}
