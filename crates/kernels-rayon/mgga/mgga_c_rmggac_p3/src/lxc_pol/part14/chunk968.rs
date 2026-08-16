//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 968/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk968(t1240: f64, t1971: f64, t515: f64, t570: f64, t7230: f64, t2289: f64, t36542: f64, t34884: f64, t8668: f64, t8831: f64, t8836: f64, t8843: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40554 = t7230 * t1971 * t515 * t570 * t1240;
    let t40556 = t36542 * t2289;
    let t40558 = t34884 * t8668;
    let t40559 = 0.24829349937757072982e-4_f64 * t40558;
    let t40560 = t34884 * t8831;
    let t40561 = 0.74488049813271218946e-4_f64 * t40560;
    let t40562 = t34884 * t8836;
    let t40563 = 0.74488049813271218946e-4_f64 * t40562;
    let t40564 = t34884 * t8843;
    (t40554, t40556, t40559, t40561, t40563, t40564)
}
