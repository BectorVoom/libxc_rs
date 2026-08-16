//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1142/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1142(t1816: f64, t637: f64, t46: f64, t552: f64, t6798: f64, t1548: f64, t2607: f64, t16632: f64, t135: f64, t1634: f64, t1009: f64, t4882: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19790 = t637 * t1816;
    let t19795 = t6798 * t46 * t552;
    let t19796 = 0.54934341918019635162e-3_f64 * t19795;
    let t19797 = t1548 * t2607;
    let t19798 = 96.0_f64 * t19797;
    let t19799 = 36.0_f64 * t16632;
    let t19800 = t135 * t1634;
    let t19803 = t4882 * t1009;
    (t19790, t19796, t19798, t19799, t19800, t19803)
}
