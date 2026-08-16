//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1020/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1020(t1181: f64, t4822: f64, t599: f64, t8463: f64, t1988: f64, t8541: f64, t30811: f64, t4908: f64, t4680: f64, t7493: f64, t8648: f64, t1421: f64, t1992: f64, t30827: f64, t7842: f64) -> (f64, f64, f64, f64, f64) {
    let t34166 = t8463 * t1181 * t599 * t4822;
    let t34170 = t1988 * t8541;
    let t34172 = t30811 * t4908;
    let t34175 = t7493 * t4680 * t8648;
    let t34179 = t30827 * t7842 * t1992 * t1421;
    (t34166, t34170, t34172, t34175, t34179)
}
