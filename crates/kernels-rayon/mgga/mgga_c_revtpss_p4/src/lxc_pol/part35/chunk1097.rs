//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1097/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1097(t33: f64, t265: f64, t502: f64, t30462: f64, t1469: f64, t2085: f64, t30502: f64, t57: f64, t5825: f64, t8059: f64, t30470: f64, t26405: f64, t30122: f64, t2047: f64, t29532: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t30503 = piecewise3(t503, 0.0_f64, t30462);
    let t30510 = piecewise3(t400, t30502, t30503 * t57 / 2.0_f64 - t8059 * t1469 - t2085 * t5825 / 2.0_f64);
    let t30511 = t30470 + t30510;
    let t30513 = t26405 * t30122;
    let t30543 = t2047 * t29532;
    (t30503, t30511, t30513, t30543)
}
