//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1211/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1211(t9573: f64, t9657: f64, t2559: f64, t2570: f64, t2606: f64, t782: f64, t9558: f64, t10033: f64, t2632: f64, t9957: f64, t9638: f64, t9653: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40998 = t9573 * t9657;
    let t41008 = t2559 * t2570;
    let t41009 = t41008 * t2606;
    let t41011 = t782 * t9558;
    let t41012 = t41011 * t10033;
    let t41014 = t2632 * t9957;
    let t41025 = t9638 * t9653;
    (t40998, t41008, t41009, t41011, t41012, t41014, t41025)
}
