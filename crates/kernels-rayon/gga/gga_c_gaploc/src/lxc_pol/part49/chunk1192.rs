//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1192/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1192(t40320: f64, t13826: f64, t1580: f64, t46952: f64, t568: f64, t597: f64, t600: f64, t13821: f64, t1628: f64, t574: f64, t13825: f64, t13750: f64, t1589: f64, t557: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48011 = 0.72851559312449424385e1_f64 * t40320;
    let t48013 = 0.23005755572352449806e1_f64 * t1580 * t13826;
    let t48017 = 0.23005755572352449806e1_f64 * t597 * t568 * t600 * t46952;
    let t48020 = 0.30674340763136599741e1_f64 * t574 * t1628 * t13821;
    let t48023 = 0.30674340763136599741e1_f64 * t597 * t1628 * t13825;
    let t48026 = 0.23833659967900284446e0_f64 * t557 * t1589 * t13750;
    (t48011, t48013, t48017, t48020, t48023, t48026)
}
