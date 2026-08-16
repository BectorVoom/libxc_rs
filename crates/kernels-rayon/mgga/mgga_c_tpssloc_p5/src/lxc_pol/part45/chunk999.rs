//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 999/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk999(t28: f64, t265: f64, t504: f64, t115099: f64, t115143: f64, t115184: f64, t2250: f64, t31512: f64, t52: f64, t607: f64, t8591: f64, t113: f64, t115107: f64, t31540: f64, t7057: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t115186 = piecewise3(t505, 0.0_f64, t115099);
    let t115193 = piecewise3(t401, t115143 + t115184, t115186 * t52 / 2.0_f64 - t31512 * t607 - t8591 * t2250 / 2.0_f64);
    let t115195 = t113 * (t115107 + t115193);
    let t115208 = 4.0_f64 * t31540 * t7057;
    (t115195, t115208)
}
