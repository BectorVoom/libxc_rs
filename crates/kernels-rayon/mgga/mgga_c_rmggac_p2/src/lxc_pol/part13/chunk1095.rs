//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1095/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1095(t558: f64, t8244: f64, t41811: f64, t41813: f64, t1562: f64, t8188: f64, t41817: f64, t41821: f64, t275: f64, t9596: f64, t36646: f64, t36663: f64, t36674: f64, t37921: f64, t38029: f64, t38031: f64, t38036: f64, t4041: f64, t41796: f64, t41803: f64, t41808: f64, t530: f64, t884: f64, t9302: f64) -> (f64, f64) {
    let t43854 = t8244 * t558;
    let t43861 = 0.39726959900411316772e-4_f64 * t41811;
    let t43862 = 0.11918087970123395032e-3_f64 * t41813;
    let t43864 = 0.4726e1_f64 * t1562 * t8188;
    let t43868 = 0.1440846329149835838e-2_f64 * t41817;
    let t43869 = 0.1440846329149835838e-2_f64 * t41821;
    let t43874 = 2.0_f64 * t275 * t9596;
    let t43875 = 0.11974241701863808564e0_f64 * t4041 * t9302 + 0.59871208509319042821e-1_f64 * t884 * t43854 - 0.23948483403727617128e0_f64 * t36646 + 0.212822999466489197e-4_f64 * t41796 - 0.212822999466489197e-4_f64 * t41803 - 0.1702583995731913576e-4_f64 * t41808 - t43861 + t43862 - t43864 - 0.2363e1_f64 * t530 * t37921 + 2.0_f64 * t38029 + t43868 + t43869 + t38031 - 0.39726959900411316772e-4_f64 * t36663 - 0.60975299583150056624e-3_f64 * t36674 + 2.0_f64 * t38036 + t43874;
    (t43854, t43875)
}
