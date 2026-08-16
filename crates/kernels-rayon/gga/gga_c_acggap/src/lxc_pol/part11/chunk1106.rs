//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1106/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1106(t1992: f64, t30262: f64, t7842: f64, t8406: f64, t30268: f64, t8903: f64, t1165: f64, t22040: f64, t7351: f64, t7493: f64, t1181: f64, t20311: f64, t7426: f64) -> (f64, f64, f64, f64) {
    let t35184 = t30262 * t7842 * t1992 * t8406;
    let t35186 = t30268 * t8903;
    let t35190 = t7493 * t1165 * t7351 * t22040;
    let t35191 = 0.47172138434406228102e-2_f64 * t35190;
    let t35194 = t7426 * t1181 * t7351 * t20311;
    (t35184, t35186, t35191, t35194)
}
