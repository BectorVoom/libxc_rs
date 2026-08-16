//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3242/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3242(t1298: f64, t18123: f64, t18128: f64, t3794: f64, t3801: f64, t5023: f64, t58598: f64, t58707: f64, t58711: f64, t58713: f64, t58715: f64, t58718: f64, t58720: f64, t58722: f64, t58726: f64) -> f64 {
    let t60155 = -3.0_f64 * t1298 * t18123 * t3801 * t5023 - 3.0_f64 * t18128 * t3794 * t5023 + t58598 - t58707 - t58711 - t58713 - t58715 + t58718 - t58720 - t58722 - t58726;
    t60155
}
