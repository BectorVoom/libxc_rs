//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2175/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2175(t30: f64, t265: f64, t393: f64, t107820: f64, t107867: f64, t107772: f64, t106638: f64, t1469: f64, t18281: f64, t1996: f64, t27755: f64, t29931: f64, t4186: f64, t45: f64, t5825: f64, t606: f64, t7194: f64, t7856: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t107868 = t107820 + t107867;
    let t107869 = piecewise3(t394, t107772, t107868);
    let t107881 = piecewise3(t120, t106638, t107869 * t45 / 2.0_f64 + t29931 * t606 / 2.0_f64 + t27755 * t1469 + t7856 * t4186 + t7194 * t5825 / 2.0_f64 + t1996 * t18281 / 2.0_f64);
    (t107868, t107881)
}
