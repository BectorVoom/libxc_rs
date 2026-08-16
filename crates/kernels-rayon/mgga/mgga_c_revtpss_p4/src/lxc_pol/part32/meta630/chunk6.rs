//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2037/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2037(t30: f64, t265: f64, t393: f64, t110792: f64, t110839: f64, t110158: f64, t110196: f64, t110711: f64, t110745: f64, t1469: f64, t18281: f64, t2078: f64, t28523: f64, t30463: f64, t4186: f64, t45: f64, t5825: f64, t606: f64, t7449: f64, t8040: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t110840 = t110792 + t110839;
    let t110841 = piecewise3(t394, 0.0_f64, t110840);
    let t110853 = piecewise3(t120, t110158 + t110196 + t110711 + t110745, t110841 * t45 / 2.0_f64 + t30463 * t606 / 2.0_f64 + t28523 * t1469 + t8040 * t4186 + t7449 * t5825 / 2.0_f64 + t2078 * t18281 / 2.0_f64);
    (t110840, t110853)
}
