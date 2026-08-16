//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 774/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk774(t2494: f64, t3114: f64, t1846: f64, t2063: f64, t11612: f64, t2368: f64, t4741: f64, t2378: f64, t2877: f64, t4703: f64, t2510: f64, t3805: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16208 = t3114 * t2494;
    let t16225 = t1846 * t2063;
    let t16227 = t11612 * t2063;
    let t16356 = t2368 * t4741;
    let t16389 = t2877 * t2378;
    let t16541 = t2368 * t4703;
    let t16640 = t3805 * t2510;
    (t16208, t16225, t16227, t16356, t16389, t16541, t16640)
}
