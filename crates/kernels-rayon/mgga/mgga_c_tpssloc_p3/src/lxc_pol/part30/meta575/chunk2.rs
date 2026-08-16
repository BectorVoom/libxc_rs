//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1950/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1950(t25: f64, t1409: f64, t1965: f64, t28469: f64, t28756: f64, t40: f64, t5398: f64, t7643: f64, t28: f64, t5527: f64, t1915: f64, t23788: f64, t28248: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t28763 = piecewise3(t115, t28469, t28756 * t40 / 2.0_f64 + t7643 * t1409 + t1965 * t5398 / 2.0_f64);
    let t28764 = t28 * t5527;
    let t28765 = t1915 * t28764;
    let t28771 = t23788 * t28248;
    (t28763, t28764, t28765, t28771)
}
