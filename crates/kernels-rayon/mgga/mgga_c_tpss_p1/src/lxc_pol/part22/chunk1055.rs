//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1055/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1055(t11475: f64, t11476: f64, t3931: f64, t11013: f64, t3919: f64, t11456: f64, t11459: f64, t11462: f64, t11464: f64, t11468: f64, t2748: f64, t3974: f64, t3979: f64, t8531: f64, t8586: f64, t925: f64, t967: f64) -> f64 {
    let t11477 = t11475 * t11476;
    let t11478 = t3931 * t11477;
    let t11481 = t3919 * t11013;
    let t11486 = -t11456 - t11459 + t11462 + 5.0_f64 / 6912.0_f64 * t967 * t11464 + 5.0_f64 / 13824.0_f64 * t967 * t11468 + t2748 * t3979 / 216.0_f64 - 5.0_f64 / 1296.0_f64 * t2748 * t3974 - 5.0_f64 / 2304.0_f64 * t967 * t11478 - t925 * t11481 / 36.0_f64 + 5.0_f64 / 20736.0_f64 * t8531 + 11.0_f64 / 324.0_f64 * t8586;
    t11486
}
