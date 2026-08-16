//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1336/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1336(t24: f64, t28906: f64, t10375: f64, t10384: f64, t11550: f64, t11557: f64, t1263: f64, t1265: f64, t31107: f64, t31596: f64, t31616: f64, t31634: f64, t31641: f64, t31654: f64, t31986: f64, t32408: f64, t3289: f64, t3293: f64, t3940: f64, t3944: f64, t422: f64, t423: f64, t960: f64, t962: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t90 = t24 <= zeta_threshold;
    let t332 = rho1 <= dens_threshold || t90;
    let t32426 = piecewise3(t90, 0.0_f64, t28906);
    let t32430 = piecewise3(t332, 0.0_f64, (t31107 + t31596 + t31616 + t31634 + t31641 + t31654 + t32408 + t31986) * t423 / 2.0_f64 + t11550 * t962 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t10375 * t1265 + 3.0_f64 / 2.0_f64 * t3940 * t3293 + 3.0_f64 / 2.0_f64 * t3289 * t3944 + 3.0_f64 / 2.0_f64 * t1263 * t10384 + t960 * t11557 / 2.0_f64 + t422 * t32426 / 2.0_f64);
    t32430
}
