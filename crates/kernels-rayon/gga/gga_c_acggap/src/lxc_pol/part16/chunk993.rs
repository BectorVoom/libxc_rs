//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 993/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk993(t1181: f64, t33509: f64, t599: f64, t7346: f64, t1992: f64, t30262: f64, t7842: f64, t8406: f64, t30268: f64, t8903: f64, t1165: f64, t22040: f64, t7351: f64, t7493: f64) -> (f64, f64, f64, f64) {
    let t35180 = t7346 * t1181 * t599 * t33509;
    let t35184 = t30262 * t7842 * t1992 * t8406;
    let t35186 = t30268 * t8903;
    let t35190 = t7493 * t1165 * t7351 * t22040;
    (t35180, t35184, t35186, t35190)
}
