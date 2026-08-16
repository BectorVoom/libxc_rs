//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 813/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk813(t28: f64, t265: f64, t504: f64, t29148: f64, t1409: f64, t2071: f64, t29188: f64, t52: f64, t5398: f64, t7884: f64, t29156: f64, t5161: f64, t7940: f64, t1458: f64, t7890: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t29189 = piecewise3(t505, 0.0_f64, t29148);
    let t29196 = piecewise3(t401, t29188, t29189 * t52 / 2.0_f64 - t7884 * t1409 - t2071 * t5398 / 2.0_f64);
    let t29197 = t29156 + t29196;
    let t29201 = t7940 * t5161;
    let t29205 = t7890 * t1458;
    (t29197, t29201, t29205)
}
