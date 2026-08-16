//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3114/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3114(t1164: f64, t14854: f64, t64451: f64, t18926: f64, t3411: f64, t14858: f64, t4884: f64, t18915: f64, t3419: f64, t18283: f64, t14855: f64, t4869: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t64454 = 0.10254018858216406658e4_f64 * t1164 * t64451 * t14854;
    let t64456 = 0.11696447245269292414e1_f64 * t3411 * t18926;
    let t64458 = 0.69263436422725855034e2_f64 * t14858 * t4884;
    let t64460 = 0.5848223622634646207e0_f64 * t18915 * t3419;
    let t64462 = 0.69263436422725855036e2_f64 * t3411 * t18283;
    let t64464 = 0.20508037716432813315e4_f64 * t4869 * t14855;
    (t64454, t64456, t64458, t64460, t64462, t64464)
}
