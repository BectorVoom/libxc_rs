//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 596/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk596(t28: f64, t265: f64, t504: f64, t1649: f64, t1877: f64, t1915: f64, t2522: f64, t6670: f64, t7541: f64, t7650: f64, t7656: f64, t7642: f64, t1409: f64, t1972: f64, t52: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t7663 = 3.0_f64 / 2.0_f64 * t2522 * t7650 + t1877 * t7541 * t28 / 2.0_f64 - t1877 * t6670 * t7656 / 2.0_f64 + t1877 * t1915 * t1649 / 2.0_f64;
    let t7664 = piecewise3(t505, 0.0_f64, t7642);
    let t7669 = piecewise3(t401, t7663, -t1972 * t1409 / 2.0_f64 + t7664 * t52 / 2.0_f64);
    (t7664, t7669)
}
