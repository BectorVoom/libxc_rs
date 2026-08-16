//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1325/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1325(t9549: f64, t9551: f64, t118: f64, t2553: f64, t794: f64, t2576: f64, t154: f64, t845: f64) -> (f64, f64, f64, f64) {
    let t9552 = t9549 * t9551;
    let t9555 = t118 * t794 * t2553;
    let t9556 = t2576 * t9555;
    let t9558 = t154 * t845;
    (t9552, t9555, t9556, t9558)
}
