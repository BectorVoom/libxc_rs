//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1859/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1859(t25120: f64, t7349: f64, t2247: f64, t239: f64, t38: f64, t6960: f64, t25163: f64, t7348: f64, t25162: f64, t2047: f64, t92576: f64, t92584: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t95290 = t25120 * t7349;
    let t95293 = t2247 * t38 * t239;
    let t95294 = t95293 * t6960;
    let t95296 = t7348 * t25163;
    let t95297 = t25162 * t95296;
    let t95303 = t2047 * t92576;
    let t95306 = t2047 * t92584;
    (t95290, t95293, t95294, t95296, t95297, t95303, t95306)
}
