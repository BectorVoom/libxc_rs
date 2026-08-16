//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1118/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1118(t25282: f64, t2736: f64, t25251: f64, t25254: f64, t25257: f64, t25258: f64, t25263: f64, t25267: f64, t25271: f64, t25276: f64, t25279: f64, t25280: f64) -> f64 {
    let t25283 = t2736 * t25282;
    let t25284 = 0.50820002809285328225e-5_f64 * t25283;
    let t25285 = -0.42874018118069736972e-3_f64 * t25251 + t25254 + t25257 - 0.42874018118069736972e-3_f64 * t25258 + 0.85748036236139473944e-3_f64 * t25263 + 0.40015750243531754508e-2_f64 * t25267 + 0.34299214494455789578e-2_f64 * t25271 + t25276 + t25279 - t25280 / 48.0_f64 - t25284;
    t25285
}
