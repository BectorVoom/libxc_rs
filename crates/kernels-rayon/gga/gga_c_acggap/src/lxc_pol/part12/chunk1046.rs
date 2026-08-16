//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1046/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1046(t30934: f64, t8450: f64, t30937: f64, t8597: f64, t8602: f64, t1165: f64, t4718: f64, t7351: f64, t7426: f64, t1181: f64, t4818: f64, t599: f64, t8463: f64) -> (f64, f64, f64, f64, f64) {
    let t34618 = t30934 * t8450;
    let t34620 = t30937 * t8597;
    let t34622 = t30937 * t8602;
    let t34626 = t7426 * t1165 * t7351 * t4718;
    let t34630 = t8463 * t1181 * t599 * t4818;
    (t34618, t34620, t34622, t34626, t34630)
}
