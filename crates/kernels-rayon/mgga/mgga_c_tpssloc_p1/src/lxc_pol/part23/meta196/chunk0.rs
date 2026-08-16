//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 834/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk834(t11135: f64, t11203: f64, t11153: f64, t461: f64, t1176: f64, t698: f64, t135: f64, t3439: f64, t3247: f64, t405: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11459 = 0.55403703703703703703e-1_f64 * t11135;
    let t11487 = 20.0_f64 / 27.0_f64 * t11203;
    let t11516 = t461 * t11153;
    let t11529 = t698 * t1176;
    let t11539 = t135 * t3439;
    let t11545 = 1.0_f64 / t405 / t3247;
    (t11459, t11487, t11516, t11529, t11539, t11545)
}
