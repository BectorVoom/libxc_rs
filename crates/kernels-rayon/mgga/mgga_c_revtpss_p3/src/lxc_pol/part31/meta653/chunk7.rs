//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2183/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2183(t33: f64, t265: f64, t502: f64, t107922: f64, t107963: f64, t108001: f64, t108047: f64, t107868: f64, t1469: f64, t18281: f64, t2003: f64, t27822: f64, t29978: f64, t4186: f64, t57: f64, t5825: f64, t606: f64, t7215: f64, t7877: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t108049 = t107922 + t107963 + t108001 + t108047;
    let t108050 = piecewise3(t503, 0.0_f64, t107868);
    let t108062 = piecewise3(t400, t108049, t108050 * t57 / 2.0_f64 - t29978 * t606 / 2.0_f64 - t27822 * t1469 - t7877 * t4186 - t7215 * t5825 / 2.0_f64 - t2003 * t18281 / 2.0_f64);
    t108062
}
