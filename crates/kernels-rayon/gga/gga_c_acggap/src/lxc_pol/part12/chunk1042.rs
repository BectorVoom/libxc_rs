//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1042/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1042(t1434: f64, t7746: f64, t1181: f64, t4526: f64, t7351: f64, t7564: f64, t4680: f64, t7426: f64, t8476: f64, t7575: f64, t8445: f64, t30937: f64, t8450: f64) -> (f64, f64, f64, f64, f64) {
    let t34549 = t7746 * t1434;
    let t34553 = t7564 * t1181 * t7351 * t4526;
    let t34556 = t7426 * t4680 * t8476;
    let t34559 = t7575 * t4680 * t8445;
    let t34561 = t30937 * t8450;
    (t34549, t34553, t34556, t34559, t34561)
}
