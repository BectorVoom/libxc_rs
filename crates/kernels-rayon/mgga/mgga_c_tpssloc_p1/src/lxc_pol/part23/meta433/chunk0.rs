//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1271/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1271(t13969: f64, t22270: f64, t3506: f64, t1227: f64, t22257: f64, t21769: f64, t248: f64, t3521: f64, t22157: f64, t3577: f64, t45124: f64, t11697: f64, t22287: f64) -> (f64, f64, f64, f64, f64) {
    let t72470 = t3506 * t13969 * t22270;
    let t72495 = t1227 * t13969 * t22257;
    let t72501 = t1227 * t248 * t3521 * t21769;
    let t72512 = t3577 * t45124 * t22157;
    let t72530 = t3577 * t11697 * t22287;
    (t72470, t72495, t72501, t72512, t72530)
}
