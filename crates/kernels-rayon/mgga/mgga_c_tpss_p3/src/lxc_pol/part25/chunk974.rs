//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 974/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk974(t13442: f64, t77: f64, t1291: f64, t1307: f64, t1314: f64, t13365: f64, t13407: f64, t3433: f64, t3436: f64, t3441: f64, t3463: f64, t3483: f64, t4609: f64, t4623: f64, t583: f64, t603: f64, t616: f64, t71: f64, t85: f64) -> f64 {
    let t13443 = t77 * t13442;
    let t13446 = -t3433 * t1314 / 6.0_f64 - t3436 * t1314 / 6.0_f64 - t1291 * t3483 / 6.0_f64 - t13365 * t85 / 12.0_f64 + t13407 * t85 / 24.0_f64 + t4609 * t616 / 24.0_f64 - t3441 * t1314 / 6.0_f64 + t3463 * t1314 / 12.0_f64 + t1307 * t3483 / 12.0_f64 - t583 * t4623 / 12.0_f64 + t603 * t4623 / 24.0_f64 + t71 * t13443 / 24.0_f64;
    t13446
}
