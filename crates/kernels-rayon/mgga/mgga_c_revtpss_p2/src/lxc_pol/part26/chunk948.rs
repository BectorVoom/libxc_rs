//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 948/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk948(t11217: f64, t12189: f64, t1100: f64, t1102: f64, t11105: f64, t11108: f64, t11114: f64, t11118: f64, t11398: f64, t11530: f64, t11533: f64, t11547: f64, t11608: f64, t11612: f64, t11614: f64, t11618: f64, t198: f64, t3329: f64, t3336: f64, t336: f64, t5023: f64) -> f64 {
    let t12190 = t11217 + t12189;
    let t12198 = -3.0_f64 * t1100 * t3329 * t3336 * t5023 + t1102 * t12190 * t198 * t336 + 2.0_f64 * t11105 * t11108 * t198 * t336 - t11114 + t11118 - t11398 - t11530 + t11533 - t11547 + t11608 - t11612 + t11614 - t11618;
    t12198
}
