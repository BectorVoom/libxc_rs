//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1247/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1247(t24232: f64, t3417: f64, t141: f64, t1145: f64, t24240: f64, t24248: f64, t24236: f64, t12296: f64, t16706: f64, t20283: f64, t20285: f64, t20287: f64, t24230: f64, t24234: f64, t24238: f64, t24242: f64, t24246: f64, t24250: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t24288 = t3417 * t24232;
    let t24289 = t141 * t24288;
    let t24291 = t1145 * t24240;
    let t24292 = t141 * t24291;
    let t24294 = t1145 * t24248;
    let t24295 = t141 * t24294;
    let t24297 = t3417 * t24236;
    let t24298 = t141 * t24297;
    let t24312 = -t12296 + 4.0_f64 / 9.0_f64 * t16706 + 2.0_f64 / 9.0_f64 * t20283 - 2.0_f64 / 3.0_f64 * t20285 - t20287 / 3.0_f64 + 10.0_f64 / 27.0_f64 * t24230 - 4.0_f64 / 3.0_f64 * t24234 - 2.0_f64 / 3.0_f64 * t24238 + 2.0_f64 * t24242 + 2.0_f64 * t24246 + t24250 / 3.0_f64;
    (t24288, t24289, t24291, t24292, t24294, t24295, t24297, t24298, t24312)
}
