//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1244/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1244(t21131: f64, t699: f64, t21135: f64, t21139: f64, t21119: f64, t21697: f64, t3216: f64, t21238: f64, t2929: f64, t21334: f64, t892: f64, t21347: f64, t300: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t68500 = t699 * t21131;
    let t68502 = t699 * t21135;
    let t68504 = t699 * t21139;
    let t68506 = t699 * t21119;
    let t68711 = t21697 * t3216;
    let t68902 = t2929 * t21238;
    let t68924 = t21334 * t892;
    let t69012 = t300 * t21347;
    (t68500, t68502, t68504, t68506, t68711, t68902, t68924, t69012)
}
