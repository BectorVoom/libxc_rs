//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 781/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk781(t2566: f64, t786: f64, t2570: f64, t792: f64, t154: f64, t845: f64, t205: f64, t59: f64, t8705: f64) -> (f64, f64, f64, f64, f64) {
    let t9546 = t2566 * t786;
    let t9549 = t792 * t2570;
    let t9558 = t154 * t845;
    let t9559 = t205 * t9558;
    let t9569 = t59 * t8705;
    (t9546, t9549, t9558, t9559, t9569)
}
