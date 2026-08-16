//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 808/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk808(t5267: f64, t7778: f64, t903: f64, t5898: f64, t290: f64, t38843: f64, t2012: f64, t7349: f64, t623: f64, t7191: f64, t34884: f64, t9206: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39535 = t903 * t7778 * t5267;
    let t39544 = t903 * t7778 * t5898;
    let t39553 = t290 * t38843;
    let t39555 = t7349 * t2012 * t39553;
    let t39570 = t623 * t7191;
    let t39591 = t34884 * t9206;
    (t39535, t39544, t39553, t39555, t39570, t39591)
}
