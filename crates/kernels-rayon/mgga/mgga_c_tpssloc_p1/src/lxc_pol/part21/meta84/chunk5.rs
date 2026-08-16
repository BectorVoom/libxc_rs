//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 606/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk606(t28: f64, t1409: f64, t1534: f64, t1649: f64, t1768: f64, t265: f64, t506: f64, t52: f64, t1647: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t1773 = piecewise3(t401, t1534 * t28 / 2.0_f64 + t265 * t1649 / 2.0_f64, -t506 * t1409 / 2.0_f64 + t1768 * t52 / 2.0_f64);
    let t1774 = t1647 + t1773;
    t1774
}
