//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1233/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1233(t39906: f64, t39908: f64, t41607: f64, t41608: f64, t41611: f64, t41615: f64, t43407: f64, t43410: f64, t43413: f64, t43415: f64, t43418: f64, t43421: f64) -> f64 {
    let t44380 = -t41607 - t41608 + 0.27013271597814698923e1_f64 * t39906 - 0.13170898365871023197e0_f64 * t39908 - t41611 - 0.17336443480108537126e0_f64 * t43407 + 0.87327386630866483588e-2_f64 * t43410 + 0.26198215989259945076e-1_f64 * t43413 - 0.17465477326173296718e-1_f64 * t43415 + t41615 + 0.46230515946956099003e0_f64 * t43418 + 0.23115257973478049502e0_f64 * t43421;
    t44380
}
