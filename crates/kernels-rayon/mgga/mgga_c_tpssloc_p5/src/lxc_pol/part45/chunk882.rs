//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 882/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk882(t28: f64, t265: f64, t504: f64, t23788: f64, t31441: f64, t25927: f64, t31448: f64, t1081: f64, t1914: f64, t31477: f64, t1877: f64, t24191: f64, t24339: f64, t2522: f64, t26756: f64, t30974: f64, t31430: f64, t31434: f64, t52: f64, t607: f64, t6841: f64, t6848: f64, t7114: f64, t8566: f64, t8586: f64, t8591: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t31496 = t23788 * t31441;
    let t31502 = t25927 * t31448;
    let t31504 = t1081 * t1914;
    let t31512 = piecewise3(t505, 0.0_f64, t31477);
    let t31517 = piecewise3(t401, 3.0_f64 / 2.0_f64 * t2522 * t8566 * t6841 + t1877 * t31430 * t28 / 2.0_f64 - t1877 * t31434 * t6848 / 2.0_f64 + t1877 * t8566 * t1081 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t24191 * t31496 - t1877 * t24339 * t8586 / 2.0_f64 + t26756 * t31502 - t1877 * t7114 * t31504 / 2.0_f64 - t1877 * t7114 * t30974 / 2.0_f64, t31512 * t52 / 2.0_f64 - t8591 * t607 / 2.0_f64);
    (t31496, t31502, t31504, t31512, t31517)
}
