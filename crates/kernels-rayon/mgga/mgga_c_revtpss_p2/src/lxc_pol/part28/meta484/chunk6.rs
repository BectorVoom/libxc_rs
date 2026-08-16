//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1843/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1843(t30: f64, t1996: f64, t2258: f64, t25459: f64, t25744: f64, t45: f64, t606: f64, t7194: f64, t2394: f64, t33: f64, t2411: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t25751 = piecewise3(t120, t25459, t25744 * t45 / 2.0_f64 + t7194 * t606 + t1996 * t2258 / 2.0_f64);
    let t25752 = t33 * t2394;
    let t25759 = t2411 * t33;
    (t25751, t25752, t25759)
}
