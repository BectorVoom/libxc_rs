//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 603/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk603(t28: f64, t265: f64, t504: f64, t5669: f64, t6278: f64, t1409: f64, t1534: f64, t1649: f64, t1768: f64, t506: f64, t52: f64, t5398: f64, t5966: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t6279 = piecewise3(t505, t6278, t5669);
    let t6286 = piecewise3(t401, t5669 * t28 / 2.0_f64 + t1534 * t1649 + t265 * t5966 / 2.0_f64, t6279 * t52 / 2.0_f64 - t1768 * t1409 - t506 * t5398 / 2.0_f64);
    (t6279, t6286)
}
