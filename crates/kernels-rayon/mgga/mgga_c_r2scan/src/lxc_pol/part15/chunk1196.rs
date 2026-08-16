//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1196/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1196(t10918: f64, t11497: f64, t3262: f64, t11506: f64, t37342: f64, t37431: f64, t37438: f64, t37443: f64, t37444: f64, t37448: f64, t40327: f64, t40329: f64, t40331: f64, t40334: f64, t40338: f64, t40342: f64, t40346: f64, t40348: f64) -> (f64, f64, f64) {
    let t40351 = 3.0_f64 / 2.0_f64 * t3262 * t10918 * t11497;
    let t40353 = 3.0_f64 / 4.0_f64 * t11506 * t37342;
    let t40355 = -0.14408463291498358381e-2_f64 * t37431 + 0.20496175532535769484e-3_f64 * t37438 - t40327 - t40329 + 0.81300399444200075504e-3_f64 * t40331 - 0.1951603679568577289e-3_f64 * t40334 + t37443 + t40338 + t40342 - t40346 + t40348 + t40351 - t40353 + 0.60975299583150056628e-3_f64 * t37444 - t37448;
    (t40351, t40353, t40355)
}
