//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 979/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk979(t1350: f64, t1992: f64, t30147: f64, t7586: f64, t5129: f64, t7647: f64, t5133: f64, t5101: f64, t7741: f64, t1434: f64, t7746: f64, t4680: f64, t7426: f64, t8476: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34526 = t30147 * t7586 * t1992 * t1350;
    let t34534 = t7647 * t5129;
    let t34537 = t7647 * t5133;
    let t34547 = t7741 * t5101;
    let t34549 = t7746 * t1434;
    let t34556 = t7426 * t4680 * t8476;
    (t34526, t34534, t34537, t34547, t34549, t34556)
}
