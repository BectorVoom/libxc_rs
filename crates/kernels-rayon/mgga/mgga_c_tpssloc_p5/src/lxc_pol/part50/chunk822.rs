//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 822/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk822(t28: f64, t265: f64, t504: f64, t1877: f64, t8366: f64, t8370: f64, t8424: f64, t52: f64, t8428: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t8434 = t1877 * t8366 * t28 / 2.0_f64 - t1877 * t8370 * t28 / 2.0_f64;
    let t8435 = piecewise3(t505, 0.0_f64, t8424);
    let t8438 = piecewise3(t401, t8434, t8435 * t52 / 2.0_f64);
    let t8439 = t8428 + t8438;
    (t8435, t8439)
}
