//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 918/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk918(t34884: f64, t9046: f64, t2289: f64, t34881: f64, t16501: f64, t7363: f64, t1966: f64, t34976: f64, t352: f64, t38422: f64, t4550: f64, t1180: f64, t34759: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39840 = t34884 * t9046;
    let t39841 = 0.24829349937757072982e-4_f64 * t39840;
    let t39842 = t34881 * t2289;
    let t39850 = t7363 * t16501;
    let t39851 = t1966 * t39850;
    let t39855 = t39851 * t34976 * t38422 * t4550 * t352;
    let t39857 = t1180 * t34759;
    (t39841, t39842, t39850, t39851, t39855, t39857)
}
