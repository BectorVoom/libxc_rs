//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 666/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk666(t1166: f64, t3411: f64, t1156: f64, t3375: f64, t3377: f64, t1164: f64, t1147: f64, t3395: f64, t3400: f64, t3403: f64, t457: f64, t697: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3413 = 0.11696447245269292414e1_f64 * t3411 * t1166;
    let t3415 = t3375 * t3377 * t1156;
    let t3417 = 0.11696447245269292414e1_f64 * t1164 * t3415;
    let t3419 = t1147 * t3395 * t1156;
    let t3421 = 0.5848223622634646207e0_f64 * t1164 * t3419;
    let t3422 = t3400 * t3377;
    let t3423 = t3422 * t3403;
    let t3425 = 0.17315859105681463759e2_f64 * t1164 * t3423;
    let t3426 = t697 * t457;
    (t3413, t3415, t3417, t3419, t3421, t3422, t3423, t3425, t3426)
}
