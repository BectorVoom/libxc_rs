//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1017/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1017(t28: f64, t265: f64, t504: f64, t128193: f64, t128239: f64, t128278: f64, t1409: f64, t33547: f64, t52: f64, t5398: f64, t8591: f64, t113: f64, t128201: f64, t1441: f64, t7467: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t128280 = piecewise3(t505, 0.0_f64, t128193);
    let t128287 = piecewise3(t401, t128239 + t128278, t128280 * t52 / 2.0_f64 - t33547 * t1409 - t8591 * t5398 / 2.0_f64);
    let t128289 = t113 * (t128201 + t128287);
    let t128296 = t1441 * t7467;
    (t128289, t128296)
}
