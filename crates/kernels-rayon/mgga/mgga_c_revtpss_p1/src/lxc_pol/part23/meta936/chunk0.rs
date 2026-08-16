//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3076/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3076(t1196: f64, t20890: f64, t58647: f64, t24473: f64, t3531: f64, t24764: f64, t5206: f64, t20400: f64, t5207: f64, t20692: f64, t29322: f64, t5023: f64, t5501: f64, t73252: f64, t81322: f64, t81326: f64, t81328: f64, t81330: f64, t81333: f64) -> (f64, f64, f64, f64, f64) {
    let t81336 = 0.30762056574649219974e4_f64 * t1196 * t20890 * t58647;
    let t81338 = 0.51947577317044391277e2_f64 * t3531 * t24473;
    let t81341 = 0.6233709278045326953e3_f64 * t1196 * t24764 * t5206;
    let t81343 = 0.51947577317044391276e2_f64 * t20400 * t5207;
    let t81350 = -3.0_f64 * t20692 * t5023 * t5501 + 6.0_f64 * t29322 * t5023 * t73252 - t81322 - t81326 + t81328 + t81330 + t81333 - t81336 - t81338 - t81341 - t81343;
    (t81336, t81338, t81341, t81343, t81350)
}
