//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 684/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk684(t30: f64, t265: f64, t393: f64, t1100: f64, t1102: f64, t198: f64, t336: f64, t5023: f64, t7177: f64, t7181: f64, t7193: f64, t1996: f64, t45: f64, t606: f64, t7099: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t7194 = piecewise3(t394, t1102 * t198 * t336 * t7177 - t1100 * t5023 * t7181, t7193);
    let t7199 = piecewise3(t120, t7099, t1996 * t606 / 2.0_f64 + t7194 * t45 / 2.0_f64);
    (t7194, t7199)
}
