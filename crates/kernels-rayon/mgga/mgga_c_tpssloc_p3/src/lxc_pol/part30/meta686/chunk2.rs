//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2169/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2169(t26318: f64, t7708: f64, t91351: f64, t19844: f64, t6916: f64, t22804: f64, t28077: f64, t22779: f64, t28067: f64, t1361: f64, t19924: f64, t26288: f64) -> (f64, f64, f64, f64, f64) {
    let t97435 = t91351 * t7708 * t26318;
    let t97437 = t6916 * t19844;
    let t97439 = t22804 * t28077;
    let t97444 = t22779 * t28067;
    let t97447 = t26288 * t1361 * t19924;
    (t97435, t97437, t97439, t97444, t97447)
}
