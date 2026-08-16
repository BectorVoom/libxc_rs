//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1130/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1130(t1181: f64, t4623: f64, t604: f64, t7426: f64, t30090: f64, t8897: f64, t31362: f64, t8903: f64, t7839: f64, t8908: f64, t8912: f64, t1165: f64, t2068: f64, t35102: f64, t7351: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36081 = t7426 * t1181 * t604 * t4623;
    let t36083 = t30090 * t8897;
    let t36085 = t31362 * t8903;
    let t36087 = t7839 * t8908;
    let t36089 = t7839 * t8912;
    let t36093 = t2068 * t1165 * t7351 * t35102;
    (t36081, t36083, t36085, t36087, t36089, t36093)
}
