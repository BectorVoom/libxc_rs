//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 851/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk851(t36935: f64, t9082: f64, t2185: f64, t678: f64, t9086: f64, t8340: f64, t8344: f64, t8347: f64, t8353: f64, t8359: f64, t8363: f64, t8366: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42250 = t36935 * t9082;
    let t42258 = t9086 * t2185 * t678;
    let t42369 = 0.13637330827122670865e-1_f64 * t8340;
    let t42372 = 0.1440846329149835838e-2_f64 * t8344;
    let t42373 = 0.1440846329149835838e-2_f64 * t8347;
    let t42374 = 0.1440846329149835838e-2_f64 * t8353;
    let t42375 = 0.1440846329149835838e-2_f64 * t8359;
    let t42376 = 0.1440846329149835838e-2_f64 * t8363;
    let t42377 = 0.5454932330849068346e-1_f64 * t8366;
    (t42250, t42258, t42369, t42372, t42373, t42374, t42375, t42376, t42377)
}
