//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 914/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk914(t10377: f64, t10385: f64, t10480: f64, t10876: f64, t10883: f64, t14508: f64, t14511: f64, t17612: f64, t17616: f64, t21393: f64, t21398: f64, t21405: f64, t21483: f64, t21487: f64, t21490: f64, t21493: f64, t3130: f64, t378: f64, t5875: f64, t5880: f64, t973: f64) -> f64 {
    let t21498 = t14508 * t5875 / 512.0_f64 + t10480 * t21393 / 512.0_f64 - t10876 * t21398 / 512.0_f64 - t14511 * t5880 / 1024.0_f64 + t10883 * t21405 / 3072.0_f64 + t10377 + t21483 * t378 / 3072.0_f64 + t10385 + t3130 * t21487 / 512.0_f64 - t973 * t21490 / 48.0_f64 + t973 * t21493 / 72.0_f64 + t17612 / 1536.0_f64 + t17616 / 288.0_f64;
    t21498
}
