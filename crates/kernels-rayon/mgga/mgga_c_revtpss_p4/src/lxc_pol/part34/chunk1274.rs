//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1274/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1274(t1558: f64, t231: f64, t6071: f64, t23327: f64, t25270: f64, t23297: f64, t23346: f64, t7045: f64, t23331: f64, t23293: f64, t23301: f64, t27261: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t113163 = t6071 * t1558 * t231;
    let t113171 = t25270 * t23327;
    let t113173 = t25270 * t23297;
    let t113177 = t7045 * t23346;
    let t113180 = t25270 * t23331;
    let t113182 = t25270 * t23293;
    let t113184 = t27261 * t23301;
    (t113163, t113171, t113173, t113177, t113180, t113182, t113184)
}
