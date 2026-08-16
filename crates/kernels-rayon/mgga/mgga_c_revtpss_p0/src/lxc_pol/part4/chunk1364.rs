//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1364/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1364(t17240: f64, t5052: f64, t1222: f64, t16738: f64, t5308: f64, t16742: f64, t16733: f64, t16771: f64, t247: f64, t3719: f64, t3636: f64, t5391: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17241 = t17240 * t5052;
    let t17243 = t1222 * t17241 / 216.0_f64;
    let t17244 = t5308 * t16738;
    let t17247 = t5308 * t16742;
    let t17250 = t5308 * t16733;
    let t17254 = t247 * t3719 * t16771;
    let t17258 = 0.10162730220579493208e-2_f64 * t5391 * t3636;
    (t17243, t17244, t17247, t17250, t17254, t17258)
}
