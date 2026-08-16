//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2241/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2241(t30: f64, t265: f64, t393: f64, t100882: f64, t100926: f64, t100833: f64, t13312: f64, t1469: f64, t1996: f64, t2258: f64, t25744: f64, t27755: f64, t4186: f64, t45: f64, t606: f64, t7194: f64, t7856: f64, t99565: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t100927 = t100882 + t100926;
    let t100928 = piecewise3(t394, t100833, t100927);
    let t100940 = piecewise3(t120, t99565, t100928 * t45 / 2.0_f64 + t27755 * t606 + t7856 * t2258 / 2.0_f64 + t25744 * t1469 / 2.0_f64 + t7194 * t4186 + t1996 * t13312 / 2.0_f64);
    (t100927, t100940)
}
