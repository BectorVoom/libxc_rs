//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1034/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1034(t11747: f64, t545: f64, t146: f64, t6533: f64, t978: f64, t2583: f64, t3433: f64, t2578: f64, t2574: f64, t20946: f64, t252: f64, t7600: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25983 = t545 * t11747;
    let t26088 = t146 * t6533 * t978;
    let t26145 = t3433 * t2583;
    let t26150 = t3433 * t2578;
    let t26176 = t3433 * t2574;
    let t26185 = t146 * t20946 * t252;
    let t26278 = t545 * t7600;
    (t25983, t26088, t26145, t26150, t26176, t26185, t26278)
}
