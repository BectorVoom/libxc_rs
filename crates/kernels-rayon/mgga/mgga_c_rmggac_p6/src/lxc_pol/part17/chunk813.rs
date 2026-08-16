//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 813/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk813(t1686: f64, t2046: f64, t2050: f64, t31: f64, t2131: f64, t5321: f64, t38350: f64, t7473: f64, t34884: f64, t9046: f64, t2289: f64, t34881: f64) -> (f64, f64, f64, f64, f64) {
    let t39808 = t2046 * t2050 * t1686 * t31;
    let t39809 = 0.43368970657079495312e-4_f64 * t39808;
    let t39827 = 0.4726e1_f64 * t5321 * t2131;
    let t39832 = t38350 * t7473;
    let t39840 = t34884 * t9046;
    let t39841 = 0.24829349937757072982e-4_f64 * t39840;
    let t39842 = t34881 * t2289;
    (t39809, t39827, t39832, t39841, t39842)
}
