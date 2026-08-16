//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2412/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2412(t2606: f64, t41008: f64, t782: f64, t9558: f64, t2617: f64, t9600: f64, t849: f64, t2642: f64, t9612: f64, t786: f64, t9569: f64, t805: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41009 = t41008 * t2606;
    let t41011 = t782 * t9558;
    let t41052 = t2617 * t9600;
    let t41053 = t41052 * t849;
    let t41063 = t9612 * t2642;
    let t41083 = t9569 * t786;
    let t41084 = t41083 * t805;
    (t41009, t41011, t41052, t41053, t41063, t41083, t41084)
}
