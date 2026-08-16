//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1730/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1730(t1166: f64, t18915: f64, t4858: f64, t4874: f64, t1164: f64, t3411: f64, t6098: f64, t4869: f64, t4884: f64, t1147: f64, t1156: f64, t18785: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18917 = 0.5848223622634646207e0_f64 * t18915 * t1166;
    let t18918 = t4874 * t4858;
    let t18920 = 0.23392894490538584828e1_f64 * t1164 * t18918;
    let t18922 = 0.11696447245269292414e1_f64 * t3411 * t6098;
    let t18924 = 0.34631718211362927517e2_f64 * t4869 * t4884;
    let t18926 = t1147 * t18785 * t1156;
    (t18917, t18918, t18920, t18922, t18924, t18926)
}
