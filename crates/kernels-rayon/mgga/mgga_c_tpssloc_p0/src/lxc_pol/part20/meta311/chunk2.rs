//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1568/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1568(t11383: f64, t11398: f64, t1156: f64, t1119: f64, t3307: f64, t3264: f64, t1117: f64, t3315: f64, t3313: f64, t1128: f64, t3324: f64, t1124: f64, t3356: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11399 = t11383 + t11398;
    let t11400 = t11399 * t1156;
    let t11403 = t1119 * t3307;
    let t11405 = 6.0_f64 * t3264 * t11403;
    let t11407 = t3307 * t3315 * t1117;
    let t11409 = 0.48245938496077605201e2_f64 * t3313 * t11407;
    let t11410 = t3324 * t1128;
    let t11415 = t1124 * t3356;
    (t11399, t11400, t11403, t11405, t11407, t11409, t11410, t11415)
}
