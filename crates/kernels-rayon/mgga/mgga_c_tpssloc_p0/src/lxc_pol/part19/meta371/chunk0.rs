//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1379/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1379(t11282: f64, t1164: f64, t3403: f64, t43679: f64, t11294: f64, t3411: f64, t11131: f64, t3399: f64, t3402: f64, t11176: f64, t300: f64, t1166: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43683 = 0.6233709278045326953e3_f64 * t1164 * t11282 * t43679 * t3403;
    let t43685 = 0.4155806185363551302e3_f64 * t3411 * t11294;
    let t43687 = 0.14035736694323150897e2_f64 * t3411 * t11131;
    let t43688 = t3399 * t3399;
    let t43689 = 1.0_f64 / t43688;
    let t43691 = t3402 * t3402;
    let t43692 = 1.0_f64 / t43691;
    let t43695 = 0.91082604192152556044e5_f64 * t1164 * t43689 * t43679 * t43692;
    let t43700 = t300 * t11176;
    let t43702 = 0.23392894490538584828e1_f64 * t43700 * t1166;
    (t43683, t43685, t43687, t43689, t43692, t43695, t43702)
}
