//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 989/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk989(t45: f64, t57: f64, t14441: f64, t10446: f64, t22671: f64, t22688: f64, t4377: f64, t5825: f64, t78: f64, t10457: f64, t4384: f64, t81: f64, t162: f64, t187: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t23193 = 12.0_f64 * t14441;
    let t23201 = piecewise3(t151, 0.0_f64, -8.0_f64 / 27.0_f64 * t10446 * t22688 + 4.0_f64 / 3.0_f64 * t4377 * t5825 + 4.0_f64 / 3.0_f64 * t78 * t22671);
    let t23209 = piecewise3(t155, 0.0_f64, 8.0_f64 / 27.0_f64 * t10457 * t22688 + 4.0_f64 / 3.0_f64 * t4384 * t5825 - 4.0_f64 / 3.0_f64 * t81 * t22671);
    let t23210 = t23201 + t23209;
    let t23211 = t23210 * t162;
    let t23213 = 0.19751673498613801407e-1_f64 * t23211 * t187;
    (t23193, t23210, t23213)
}
