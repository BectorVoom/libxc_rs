//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2084/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2084(t2697: f64, t9609: f64, t2703: f64, t9601: f64, t40904: f64, t842: f64, t9573: f64, t9657: f64, t2559: f64, t2570: f64, t2606: f64, t782: f64, t9558: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40988 = t2697 * t9609;
    let t40990 = t9601 * t2703;
    let t40992 = t40904 * t842;
    let t40998 = t9573 * t9657;
    let t41008 = t2559 * t2570;
    let t41009 = t41008 * t2606;
    let t41011 = t782 * t9558;
    (t40988, t40990, t40992, t40998, t41008, t41009, t41011)
}
