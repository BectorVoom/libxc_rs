//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 137/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk137(t30: f64, t33: f64, t379: f64, t385: f64, t342: f64, t198: f64, t293: f64, t328: f64, t330: f64, t336: f64, t265: f64, t45: f64, t57: f64, dens_threshold: f64, rho0: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t386 = t379 * t385;
    let t389 = 1.0_f64 + 0.65854491829355115987e0_f64 * t342 * t386;
    let t390 = f64::ln(t389);
    let t393 = t198 * t336 * t390 - t293 + t328 + t330;
    let t394 = t265 < t393;
    let t395 = piecewise3(t394, t393, t265);
    let t398 = piecewise3(t120, t265 * t30 / 2.0_f64, t395 * t45 / 2.0_f64);
    let t400 = rho1 <= dens_threshold || t34;
    let t403 = 1.0_f64 / t57;
    (t386, t389, t395, t398, t403, t393)
}
