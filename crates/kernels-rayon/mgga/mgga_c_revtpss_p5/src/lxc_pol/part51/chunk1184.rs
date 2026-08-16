//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1184/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1184(t32298: f64, t7898: f64, t118: f64, t125345: f64, t125945: f64, t125948: f64, t125950: f64, t127189: f64, t127296: f64, t127299: f64, t127302: f64, t127305: f64, t127306: f64, t127308: f64, t127313: f64, t127314: f64, t127318: f64, t127324: f64, t127326: f64, t127328: f64, t127330: f64, t32162: f64, t4293: f64, t671: f64) -> f64 {
    let t127332 = t7898 * t32298;
    let t127333 = -4.0_f64 * t125945 - t125948 - t125950 - t118 * (t127189 + t127296) - 2.0_f64 * t127299 + t127302 + t127305 - 2.0_f64 * t127306 + 6.0_f64 * t127308 + t127313 + 2.0_f64 * t127314 + 2.0_f64 * t127318 - 2.0_f64 * t125345 * t671 - 2.0_f64 * t32162 * t4293 - 4.0_f64 * t127324 - 4.0_f64 * t127326 - 4.0_f64 * t127328 - 4.0_f64 * t127330 + t127332;
    t127333
}
