//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 704/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk704(t30: f64, t265: f64, t393: f64, t207: f64, t7427: f64, t1940: f64, t198: f64, t2071: f64, t2403: f64, t7432: f64, t775: f64, t890: f64, t892: f64, t2078: f64, t45: f64, t605: f64, t606: f64, t7010: f64, t7092: f64, t7428: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t7443 = t207 * t7427;
    let t7448 = -t1940 * t7432 * t890 + t198 * t7443 * t892 + 3.0_f64 * t2071 * t2403 * t775;
    let t7449 = piecewise3(t394, 0.0_f64, t7448);
    let t7454 = piecewise3(t120, 3.0_f64 / 2.0_f64 * t2403 * t2071 * t7010 + t1940 * t7428 * t30 / 2.0_f64 - t1940 * t7432 * t7092 / 2.0_f64 + t1940 * t2071 * t605 / 2.0_f64, t2078 * t606 / 2.0_f64 + t7449 * t45 / 2.0_f64);
    (t7443, t7448, t7449, t7454)
}
