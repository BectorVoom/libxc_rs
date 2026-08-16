//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 250/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk250(t1131: f64, t156: f64, t155: f64, t2: f64, t388: f64, t428: f64, t180: f64, t214: f64, t243: f64, t426: f64, t1034: f64, t181: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1132 = t156 * t1131;
    let t1133 = t155 * t1132;
    let t1134 = t388 * t2;
    let t1135 = t1134 * t428;
    let t1136 = 0.36622894612013090108e-3_f64 * t1135;
    let t1138 = t243 * t214 * t180;
    let t1140 = 0.24415263074675393405e-3_f64 * t426 * t1138;
    let t1142 = 0.19751673498613801407e-1_f64 * t1034 * t181;
    (t1132, t1133, t1134, t1135, t1136, t1138, t1140, t1142)
}
