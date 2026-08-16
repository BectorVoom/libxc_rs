//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 905/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk905(t13633: f64, t151: f64, t395: f64, t409: f64, t1103: f64, t3700: f64, t3570: f64, t962: f64, t1077: f64, t336: f64, t1163: f64, t1181: f64, t3169: f64, t991: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13635 = t151 * t395 * t13633;
    let t13636 = t13635 * t409;
    let t13638 = t3700 * t1103;
    let t13654 = t3570 * t962;
    let t13656 = t336 * t1077;
    let t13664 = t1163 * t1181 * t991 * t3169;
    (t13635, t13636, t13638, t13654, t13656, t13664)
}
