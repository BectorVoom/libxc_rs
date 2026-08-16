//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1896/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1896(t22779: f64, t28067: f64, t1361: f64, t19924: f64, t26288: f64, t19994: f64, t19919: f64, t221: f64, t91194: f64, t26284: f64, t91198: f64, t20000: f64, t91361: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t97444 = t22779 * t28067;
    let t97447 = t26288 * t1361 * t19924;
    let t97450 = t26288 * t1361 * t19994;
    let t97453 = t91194 * t221 * t19919;
    let t97456 = t26284 * t221 * t19924;
    let t97459 = t91198 * t1361 * t19919;
    let t97461 = t91361 * t20000;
    (t97444, t97447, t97450, t97453, t97456, t97459, t97461)
}
