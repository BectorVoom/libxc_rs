//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 128/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk128(t33: f64, t488: f64, t494: f64, t460: f64, t198: f64, t336: f64, t424: f64, t452: f64, t454: f64, t265: f64, t57: f64, t398: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t495 = t488 * t494;
    let t498 = 1.0_f64 + 0.65854491829355115987e0_f64 * t460 * t495;
    let t499 = f64::ln(t498);
    let t502 = t198 * t336 * t499 - t424 + t452 + t454;
    let t503 = t265 < t502;
    let t504 = piecewise3(t503, t502, t265);
    let t507 = piecewise3(t400, t265 * t33 / 2.0_f64, t504 * t57 / 2.0_f64);
    let t508 = t398 + t507;
    (t495, t498, t504, t508, t502)
}
