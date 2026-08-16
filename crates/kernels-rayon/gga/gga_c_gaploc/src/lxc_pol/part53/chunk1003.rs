//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1003/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1003(t13821: f64, t1628: f64, t574: f64, t13825: f64, t597: f64, t13750: f64, t1589: f64, t557: f64, t13829: f64, t193: f64, t524: f64, t1: f64, t46873: f64, t544: f64) -> (f64, f64, f64, f64, f64) {
    let t48020 = 0.30674340763136599741e1_f64 * t574 * t1628 * t13821;
    let t48023 = 0.30674340763136599741e1_f64 * t597 * t1628 * t13825;
    let t48026 = 0.23833659967900284446e0_f64 * t557 * t1589 * t13750;
    let t48029 = 0.35750489951850426669e0_f64 * t524 * t13829 * t193;
    let t48032 = t544 * t46873 * t1;
    (t48020, t48023, t48026, t48029, t48032)
}
