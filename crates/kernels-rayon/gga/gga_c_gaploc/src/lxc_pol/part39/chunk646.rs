//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 646/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk646(t10564: f64, t10570: f64, t10573: f64, t10578: f64, t10584: f64, t10587: f64, t10591: f64, t10594: f64, t10599: f64, t10603: f64, t10604: f64, t1441: f64, t1580: f64, t1599: f64, t1641: f64, t193: f64, t3372: f64, t3387: f64, t3403: f64, t3415: f64, t541: f64, t557: f64, t574: f64, t597: f64) -> f64 {
    let t10607 = 0.23833659967900284446e0_f64 * t3372 * t541 - 0.30674340763136599741e1_f64 * t574 * t10564 + 0.23005755572352449806e1_f64 * t1580 * t3415 + 0.23005755572352449806e1_f64 * t597 * t10570 + 0.30674340763136599741e1_f64 * t597 * t10573 - 0.35750489951850426669e0_f64 * t1599 * t3387 - 0.35750489951850426669e0_f64 * t557 * t10578 - 0.23005755572352449806e1_f64 * t1641 * t3403 - 0.23005755572352449806e1_f64 * t574 * t10584 + 0.35750489951850426669e0_f64 * t10587 * t193 + 0.35750489951850426669e0_f64 * t10591 * t193 - 0.23833659967900284446e0_f64 * t557 * t10594 + t10599 - t10603 + 0.51123901271894332902e0_f64 * t1441 * t10604;
    t10607
}
