//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1252/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1252(t6888: f64, t6891: f64, t80707: f64, t1377: f64, t1385: f64, t22633: f64, t22635: f64, t3719: f64, t12033: f64, t1386: f64, t2016: f64, t22630: f64, t3752: f64, t3758: f64, t39916: f64, t568: f64, t6955: f64, t6963: f64, t81315: f64, t81318: f64, t81319: f64, t81328: f64, t81333: f64) -> f64 {
    let t81339 = t6888 * t80707 * t6891;
    let t81346 = t22633 * t22635 * t1377 * t3719 * t1385;
    let t81348 = 0.49348022005446793095e-1_f64 * t81315 - t81318 - 3.0_f64 * t81319 * t1386 - 3.0_f64 * t39916 * t2016 - 18.0_f64 * t3758 * t22630 - 0.49348022005446793095e-1_f64 * t81328 + 0.14804406601634037928e0_f64 * t81333 + 3.0_f64 * t3752 * t6955 * t568 - 0.49348022005446793095e-1_f64 * t81339 + 6.0_f64 * t12033 * t6963 + 0.49348022005446793095e-1_f64 * t81346;
    t81348
}
