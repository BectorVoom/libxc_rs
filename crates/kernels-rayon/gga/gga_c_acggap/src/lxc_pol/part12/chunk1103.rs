//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1103/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1103(t1077: f64, t1181: f64, t2068: f64, t525: f64, t604: f64, t7839: f64, t8966: f64, t33953: f64, t5284: f64, t13299: f64, t31115: f64, t31195: f64, t35420: f64) -> (f64, f64, f64, f64, f64) {
    let t35629 = t2068 * t1181 * t604 * t525 * t1077;
    let t35631 = t7839 * t8966;
    let t35633 = t33953 * t5284;
    let t35635 = t31115 * t13299 * t35633;
    let t35638 = t31195 * t13299 * t35420;
    (t35629, t35631, t35633, t35635, t35638)
}
