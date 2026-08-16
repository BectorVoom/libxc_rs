//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1038/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1038(t2583: f64, t3433: f64, t2578: f64, t2574: f64, t146: f64, t20946: f64, t252: f64, t1543: f64, t2567: f64, t113: f64, t2526: f64, t494: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26145 = t3433 * t2583;
    let t26150 = t3433 * t2578;
    let t26176 = t3433 * t2574;
    let t26185 = t146 * t20946 * t252;
    let t26186 = t2567 * t1543;
    let t26274 = t2526 * t494 * t113;
    (t26145, t26150, t26176, t26185, t26186, t26274)
}
