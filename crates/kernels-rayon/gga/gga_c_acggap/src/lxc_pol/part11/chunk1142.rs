//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1142/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1142(t2001: f64, t5255: f64, t5003: f64, t1418: f64, t7605: f64, t5260: f64, t4547: f64, t1347: f64, t1980: f64, t35383: f64, t7458: f64, t31773: f64, t8634: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35668 = t2001 * t5255;
    let t35670 = t2001 * t5003;
    let t35672 = t7605 * t1418;
    let t35673 = 0.68598428988911579156e-2_f64 * t35672;
    let t35674 = t2001 * t5260;
    let t35676 = t2001 * t4547;
    let t35678 = t7605 * t1347;
    let t35679 = 0.68598428988911579156e-2_f64 * t35678;
    let t35682 = t1980 * t7458 * t35383;
    let t35683 = 0.28582678745379824648e-3_f64 * t35682;
    let t35685 = t31773 * t8634;
    (t35668, t35670, t35673, t35674, t35676, t35679, t35683, t35685)
}
