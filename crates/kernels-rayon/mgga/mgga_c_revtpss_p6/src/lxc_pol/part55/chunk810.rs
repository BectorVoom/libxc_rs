//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 810/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk810(t30: f64, t33: f64, t265: f64, t393: f64, t502: f64, t1962: f64, t207: f64, t8656: f64, t1940: f64, t198: f64, t7432: f64, t892: f64, t45: f64, t8657: f64, t57: f64, dens_threshold: f64, rho0: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t8660 = t30 * t1962;
    let t8665 = t207 * t8656;
    let t8670 = -t1940 * t1962 * t7432 + t198 * t8665 * t892;
    let t8671 = piecewise3(t394, 0.0_f64, t8670);
    let t8674 = piecewise3(t120, t1940 * t8657 * t30 / 2.0_f64 - t1940 * t7432 * t8660 / 2.0_f64, t8671 * t45 / 2.0_f64);
    let t8677 = t33 * t1962;
    let t8682 = piecewise3(t503, 0.0_f64, t8670);
    let t8685 = piecewise3(t400, t1940 * t8657 * t33 / 2.0_f64 - t1940 * t7432 * t8677 / 2.0_f64, t8682 * t57 / 2.0_f64);
    (t8660, t8665, t8671, t8674, t8677, t8682, t8685)
}
