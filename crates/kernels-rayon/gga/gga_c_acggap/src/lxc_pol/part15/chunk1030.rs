//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1030/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1030(t31362: f64, t8903: f64, t7839: f64, t8908: f64, t8912: f64, t8970: f64, t1181: f64, t31567: f64, t36019: f64, t599: f64, t1992: f64, t7585: f64, t7586: f64, t8960: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36085 = t31362 * t8903;
    let t36087 = t7839 * t8908;
    let t36089 = t7839 * t8912;
    let t36096 = t7839 * t8970;
    let t36115 = t31567 * t1181 * t599 * t36019;
    let t36119 = t7585 * t7586 * t1992 * t8960;
    (t36085, t36087, t36089, t36096, t36115, t36119)
}
