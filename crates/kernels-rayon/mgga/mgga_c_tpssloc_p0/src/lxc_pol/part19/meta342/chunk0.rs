//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1219/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1219(t41083: f64, t789: f64, t41011: f64, t9561: f64, t154: f64, t1891: f64, t205: f64, t792: f64, t9558: f64, t118: f64, t794: f64, t9458: f64) -> (f64, f64, f64, f64) {
    let t41156 = t41083 * t789;
    let t41158 = t41011 * t9561;
    let t41160 = t154 * t1891;
    let t41161 = t205 * t41160;
    let t41170 = t792 * t9558;
    let t41173 = t41170 * t118 * t794 * t9458;
    (t41156, t41158, t41161, t41173)
}
