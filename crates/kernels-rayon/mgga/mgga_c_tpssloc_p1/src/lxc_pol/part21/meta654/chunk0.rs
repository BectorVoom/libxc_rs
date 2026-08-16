//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2452/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2452(t3185: f64, t42741: f64, t1014: f64, t42340: f64, t42341: f64, t3127: f64, t23508: f64, t3131: f64, t3199: f64, t10474: f64, t10482: f64, t11060: f64, t3120: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43480 = t42741 * t3185;
    let t43503 = t42340 * t42341 * t1014;
    let t43515 = t42340 * t42341 * t3127;
    let t43516 = t23508 * t3131;
    let t43536 = t42741 * t3199;
    let t43553 = t42340 * t42341 * t10474;
    let t43554 = t23508 * t10482;
    let t43558 = t11060 * t3120;
    (t43480, t43503, t43515, t43516, t43536, t43553, t43554, t43558)
}
