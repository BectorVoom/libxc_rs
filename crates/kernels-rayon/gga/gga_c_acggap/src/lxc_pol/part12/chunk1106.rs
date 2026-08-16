//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1106/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1106(t2001: f64, t5260: f64, t4547: f64, t1347: f64, t7605: f64, t1980: f64, t35383: f64, t7458: f64, t31773: f64, t8634: f64, t13299: f64, t33944: f64, t33945: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35674 = t2001 * t5260;
    let t35676 = t2001 * t4547;
    let t35678 = t7605 * t1347;
    let t35682 = t1980 * t7458 * t35383;
    let t35685 = t31773 * t8634;
    let t35691 = t33944 * t13299 * t33945;
    (t35674, t35676, t35678, t35682, t35685, t35691)
}
