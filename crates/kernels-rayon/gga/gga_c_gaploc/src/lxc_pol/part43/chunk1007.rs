//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1007/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1007(t188: f64, t189: f64, t193: f64, t46952: f64, t13830: f64, t541: f64, t13822: f64, t1641: f64, t568: f64, t569: f64, t574: f64, t13778: f64, t587: f64, t589: f64) -> (f64, f64, f64, f64, f64) {
    let t48107 = 0.35750489951850426669e0_f64 * t188 * t189 * t46952 * t193;
    let t48109 = 0.23833659967900284446e0_f64 * t13830 * t541;
    let t48111 = 0.23005755572352449806e1_f64 * t1641 * t13822;
    let t48115 = 0.23005755572352449806e1_f64 * t574 * t568 * t569 * t46952;
    let t48121 = t587 * t589 * t13778;
    (t48107, t48109, t48111, t48115, t48121)
}
