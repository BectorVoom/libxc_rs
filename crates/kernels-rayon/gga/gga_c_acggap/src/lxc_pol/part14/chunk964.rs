//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 964/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk964(t2450: f64, t7583: f64, t8461: f64, t1427: f64, t1992: f64, t7842: f64, t1530: f64, t1535: f64, t30539: f64, t2304: f64, t7610: f64, t1988: f64, t8561: f64) -> (f64, f64, f64, f64, f64) {
    let t34186 = t2450 * t7583 * t8461;
    let t34189 = t34186 * t7842 * t1992 * t1427;
    let t34204 = t1530 * t30539 * t1535;
    let t34215 = t7610 * t2304;
    let t34217 = t1988 * t8561;
    (t34186, t34189, t34204, t34215, t34217)
}
