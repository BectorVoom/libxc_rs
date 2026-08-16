//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2226/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2226(t16219: f64, t7111: f64, t139: f64, t27526: f64, t3252: f64, t4574: f64, t1014: f64, t4579: f64, t15130: f64, t15135: f64, t15140: f64, t15145: f64, t15149: f64, t15154: f64, t15651: f64, t1665: f64, t25490: f64, t27527: f64, t27531: f64, t4854: f64, t53321: f64, t7117: f64, t93736: f64) -> f64 {
    let t100365 = t7111 * t16219;
    let t100370 = t27526 * t139 * t3252 * t4574 / 324.0_f64;
    let t100398 = t27526 * t139 * t1014 * t4579 / 216.0_f64;
    let t100399 = -t100365 / 1296.0_f64 + t100370 - t27526 * t27527 * t15145 / 72.0_f64 - t27526 * t27527 * t15149 / 144.0_f64 - t27526 * t27531 * t15154 / 36.0_f64 + t27526 * t27531 * t15130 / 108.0_f64 + t27526 * t27531 * t15135 / 216.0_f64 + 7.0_f64 / 648.0_f64 * t27526 * t53321 * t15140 - 0.85748036236139473944e-3_f64 * t25490 * t4854 - 0.42874018118069736972e-3_f64 * t7117 * t15651 - 0.42874018118069736972e-3_f64 * t93736 * t1665 - t100398;
    t100399
}
