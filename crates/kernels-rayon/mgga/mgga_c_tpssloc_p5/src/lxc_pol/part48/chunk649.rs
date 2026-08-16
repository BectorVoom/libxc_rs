//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 649/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk649(t25: f64, t28: f64, t265: f64, t394: f64, t504: f64, t1914: f64, t202: f64, t8565: f64, t1877: f64, t193: f64, t7114: f64, t870: f64, t40: f64, t8566: f64, t52: f64, dens_threshold: f64, rho0: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t8569 = t25 * t1914;
    let t8574 = t202 * t8565;
    let t8579 = -t1877 * t1914 * t7114 + t193 * t8574 * t870;
    let t8580 = piecewise3(t395, 0.0_f64, t8579);
    let t8583 = piecewise3(t115, t1877 * t8566 * t25 / 2.0_f64 - t1877 * t7114 * t8569 / 2.0_f64, t8580 * t40 / 2.0_f64);
    let t8586 = t28 * t1914;
    let t8591 = piecewise3(t505, 0.0_f64, t8579);
    let t8594 = piecewise3(t401, t1877 * t8566 * t28 / 2.0_f64 - t1877 * t7114 * t8586 / 2.0_f64, t8591 * t52 / 2.0_f64);
    (t8569, t8580, t8583, t8586, t8591, t8594)
}
